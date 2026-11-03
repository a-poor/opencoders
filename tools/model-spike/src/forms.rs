//! Form lifecycle envelopes and strict inputs; reviewed scalar/union adaptations.
use crate::{
    event_models::{enum_dispatch_tag, pascal},
    generate_with_settings, select_schemas,
    server_info::allow_additive_fields,
    sessions::rename_input_refs,
};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

/// Remove the redundant special-number string branch before Typify can flatten
/// overlapping scalar alternatives into an object. All strings remain strings.
pub(crate) fn normalize_form_values(
    schema: &mut Value,
    suffix: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for (name, pointer, array) in [
        (format!("Form.Value{suffix}"), "", true),
        (format!("Form.When{suffix}"), "/properties/value", false),
    ] {
        let value = schema["definitions"][&name]
            .pointer_mut(pointer)
            .ok_or("missing form value schema")?;
        let mut expected = vec![
            json!({"type":"string"}),
            json!({"type":"number"}),
            json!({"type":"string","enum": if suffix.is_empty() {vec!["Infinity","-Infinity","NaN"]} else {vec!["NaN","Infinity","-Infinity"]}}),
            json!({"type":"boolean"}),
        ];
        if array {
            expected.push(json!({"type":"array","items":{"type":"string"}}));
        }
        if *value != json!({"anyOf":expected}) {
            return Err(format!("{name} changed; review form scalar adapter").into());
        }
        expected.remove(2);
        *value = json!({"anyOf":expected});
    }
    Ok(())
}

pub fn generate_forms(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    guard_inputs(spec)?;
    normalize_form_values(&mut select_schemas(spec, &["Form.Value", "Form.When"])?, "")?;
    let mut spec = spec.clone();
    // Shared response/event types are safe only while both wire contracts agree.
    for name in [
        "Info",
        "Fields",
        "Field",
        "StringField",
        "NumberField",
        "IntegerField",
        "BooleanField",
        "MultiselectField",
        "When",
        "Value",
        "Answer",
    ] {
        let mut event = spec["components"]["schemas"][format!("Form.{name}_1")].clone();
        normalize_event_refs(&mut event);
        let mut http = spec["components"]["schemas"][format!("Form.{name}")].clone();
        sort_enums(&mut http);
        sort_enums(&mut event);
        if http != event {
            return Err(format!("Form.{name} differs from event contract; review reuse").into());
        }
    }
    for (name, path) in [
        (
            "FormListResponse",
            "/paths/~1api~1form/get/responses/200/content/application~1json/schema",
        ),
        (
            "SessionFormListResponse",
            "/paths/~1api~1session~1{sessionID}~1form/get/responses/200/content/application~1json/schema",
        ),
        (
            "SessionFormCreateResponse",
            "/paths/~1api~1session~1{sessionID}~1form/post/responses/200/content/application~1json/schema",
        ),
        (
            "SessionFormResponse",
            "/paths/~1api~1session~1{sessionID}~1form~1{formID}/get/responses/200/content/application~1json/schema",
        ),
    ] {
        spec["components"]["schemas"][name] = spec
            .pointer(path)
            .ok_or("missing form endpoint schema")?
            .clone();
    }
    for (path, name) in [
        (
            "/paths/~1api~1session~1{sessionID}~1form/post/requestBody/content/application~1json/schema",
            "Form.CreatePayload",
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1form~1{formID}~1reply/post/requestBody/content/application~1json/schema",
            "Form.Reply",
        ),
    ] {
        if spec.pointer(path) != Some(&json!({"$ref":format!("#/components/schemas/{name}")})) {
            return Err("form request reference changed".into());
        }
    }
    let input_spec = spec.clone();
    for name in [
        "Form.Info",
        "Form.Fields",
        "Form.Answer",
        "Form.Metadata",
        "Location.PublicRef",
    ] {
        spec["components"]["schemas"][name] = json!({});
    }
    let mut schema = select_schemas(
        &spec,
        &[
            "FormListResponse",
            "SessionFormListResponse",
            "SessionFormCreateResponse",
            "SessionFormResponse",
        ],
    )?;
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "FormId",
        "FormInfo",
        "FormFields",
        "FormAnswer",
        "FormMetadata",
        "LocationPublicRef",
        "AbsolutePath",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    let mut dispatch = String::new();
    dispatch_union(
        &mut schema,
        &mut settings,
        ("Form.State", "FormState", "status"),
        &["pending", "answered", "cancelled"],
        &mut dispatch,
    )?;
    allow_additive_fields(&mut schema);

    spec = input_spec;
    let names = [
        ("Form.CreatePayload", "SessionFormCreateRequest"),
        ("Form.Reply", "SessionFormReplyRequest"),
        ("Form.Field", "FormFieldInput"),
        ("Form.Fields", "FormFieldsInput"),
        ("Form.StringField", "FormStringFieldInput"),
        ("Form.NumberField", "FormNumberFieldInput"),
        ("Form.IntegerField", "FormIntegerFieldInput"),
        ("Form.BooleanField", "FormBooleanFieldInput"),
        ("Form.MultiselectField", "FormMultiselectFieldInput"),
        ("Form.ExternalField", "FormExternalFieldInput"),
        ("Form.Option", "FormOptionInput"),
        ("Form.When", "FormWhenInput"),
    ];
    for (old, new) in names {
        let mut v = spec["components"]["schemas"][old].clone();
        rename_input_refs(&mut v, &names);
        spec["components"]["schemas"][new] = v;
    }
    let mut inputs = select_schemas(
        &spec,
        &["SessionFormCreateRequest", "SessionFormReplyRequest"],
    )?;
    guard_integers(&inputs)?;
    let fields = json!({"type":"array","minItems":1,"prefixItems":[{"$ref":"#/definitions/FormFieldInput"}],"items":{"$ref":"#/definitions/FormFieldInput"}});
    if inputs["definitions"]["FormFieldsInput"] != fields {
        return Err("form nonempty fields changed".into());
    }
    inputs["definitions"]["FormFieldsInput"] = json!({});
    settings.with_replacement(
        "FormFieldsInput",
        "super::FormFieldsInput",
        std::iter::empty(),
    );
    settings.with_replacement("FormValue", "super::FormValue", std::iter::empty());
    // The condition is the same scalar union as the reviewed event condition.
    inputs["definitions"]["FormWhenInput"]["properties"]["value"] =
        json!({"$ref":"#/definitions/FormConditionValue"});
    inputs["definitions"]["FormConditionValue"] = json!({});
    settings.with_replacement(
        "FormConditionValue",
        "super::FormConditionValue",
        std::iter::empty(),
    );
    // Give numeric input limits/defaults one ergonomic shared wire type.
    let number = json!({"anyOf":[{"type":"number"},{"type":"string","enum":["Infinity","-Infinity","NaN"]}]});
    for name in ["FormNumberFieldInput", "FormIntegerFieldInput"] {
        for key in ["minimum", "maximum", "default"] {
            if inputs["definitions"][name]["properties"][key] != number {
                return Err("form number contract changed".into());
            }
            inputs["definitions"][name]["properties"][key] =
                json!({"$ref":"#/definitions/FormNumber"});
        }
    }
    inputs["definitions"]["FormNumber"] = number;
    dispatch_union(
        &mut inputs,
        &mut settings,
        ("FormFieldInput", "FormFieldInput", "type"),
        &[
            "string",
            "number",
            "integer",
            "boolean",
            "multiselect",
            "external",
        ],
        &mut dispatch,
    )?;
    // Record optional fields before generation: Typify otherwise collapses absent
    // versus empty arrays/maps. Every nonnullable input rejects explicit null.
    let mut optional = std::collections::BTreeMap::new();
    for (name, v) in inputs["definitions"].as_object().unwrap() {
        if !name.ends_with("Input")
            && !name.starts_with("SessionForm")
            && !name.starts_with("FormFieldInput")
        {
            continue;
        }
        if let Some(properties) = v["properties"].as_object() {
            if v["additionalProperties"] != json!(false) {
                return Err("form inputs must remain strict objects".into());
            }
            for (key, property) in properties {
                if !v["required"]
                    .as_array()
                    .is_some_and(|r| r.contains(&json!(key)))
                {
                    let nullable = property
                        .get("anyOf")
                        .is_some_and(|a| a.as_array().unwrap().contains(&json!({"type":"null"})));
                    if nullable && (name != "SessionFormCreateRequest" || key != "id") {
                        return Err("new nullable form input requires review".into());
                    }
                    optional.insert((pascal(name), key.clone()), nullable);
                }
            }
        }
    }
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    for (wire, rust) in [
        (json!({"type":"number"}), "::serde_json::Number"),
        (
            json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64}),
            "super::EventInteger<0,9007199254740991>",
        ),
    ] {
        settings.with_conversion(serde_json::from_value(wire)?, rust, std::iter::empty());
    }
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for field in &mut item.fields {
                    let ident = field.ident.as_ref().unwrap().to_string();
                    let mut key = ident.trim_end_matches('_').to_owned();
                    for attr in &field.attrs {
                        if attr.path().is_ident("serde") {
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("rename") {
                                    key = meta.value()?.parse::<syn::LitStr>()?.value();
                                } else if meta.input.peek(syn::Token![=]) {
                                    let _ = meta.value()?.parse::<syn::Expr>()?;
                                }
                                Ok(())
                            })?;
                        }
                    }
                    if let Some(nullable) = optional.remove(&(item.ident.to_string(), key.clone()))
                    {
                        let inner = option_inner(&field.ty).unwrap_or_else(|| field.ty.clone());
                        field.attrs.retain(|a| !a.path().is_ident("serde"));
                        if key != ident {
                            field.attrs.push(parse_quote!(#[serde(rename=#key)]));
                        }
                        if nullable {
                            field.ty = parse_quote!(super::RequestField<#inner>);
                            field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="super::RequestField::is_omitted")]));
                        } else {
                            field.ty = parse_quote!(Option<#inner>);
                            field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                        }
                    }
                }
            }
            Item::Enum(item) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
            }
            _ => {}
        }
    }
    if !optional.is_empty() {
        return Err(format!("unadapted form input fields: {optional:?}").into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --forms.\n// Do not edit manually; adaptations live in tools/model-spike/src/forms.rs.\n{}{}",
        prettyplease::unparse(&syntax),
        prettyplease::unparse(&syn::parse_file(&dispatch)?)
    ))
}
fn guard_inputs(spec: &Value) -> Result<(), Box<dyn std::error::Error>> {
    for (name, expected) in [
        (
            "Form.CreatePayload",
            json!({"type":"object","properties":{"id":{"anyOf":[{"$ref":"#/components/schemas/Form.ID"},{"type":"null"}]},"title":{"type":"string"},"metadata":{"$ref":"#/components/schemas/Form.Metadata"},"fields":{"$ref":"#/components/schemas/Form.Fields"}},"required":["title","fields"],"additionalProperties":false}),
        ),
        (
            "Form.Reply",
            json!({"type":"object","properties":{"answer":{"$ref":"#/components/schemas/Form.Answer"}},"required":["answer"],"additionalProperties":false}),
        ),
        (
            "Form.State",
            json!({"anyOf":[
                {"type":"object","properties":{"status":{"type":"string","enum":["pending"]}},"required":["status"],"additionalProperties":false},
                {"type":"object","properties":{"status":{"type":"string","enum":["answered"]},"answer":{"$ref":"#/components/schemas/Form.Answer"}},"required":["status","answer"],"additionalProperties":false},
                {"type":"object","properties":{"status":{"type":"string","enum":["cancelled"]},"message":{"type":"string"}},"required":["status"],"additionalProperties":false}
            ]}),
        ),
    ] {
        if spec["components"]["schemas"][name] != expected {
            return Err(format!("{name} changed; review form input/state adapters").into());
        }
    }
    Ok(())
}
fn guard_integers(v: &Value) -> Result<(), Box<dyn std::error::Error>> {
    match v {
        Value::Object(o) => {
            if o.get("type") == Some(&json!("integer"))
                && *v != json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64})
            {
                return Err("unreviewed form integer constraint".into());
            }
            for v in o.values() {
                guard_integers(v)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                guard_integers(v)?;
            }
        }
        _ => {}
    }
    Ok(())
}
fn option_inner(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(p) = ty else { return None };
    let s = p.path.segments.last()?;
    if s.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(a) = &s.arguments else {
        return None;
    };
    let syn::GenericArgument::Type(t) = a.args.first()? else {
        return None;
    };
    Some(t.clone())
}
fn normalize_event_refs(v: &mut Value) {
    match v {
        Value::Object(o) => {
            if let Some(Value::String(r)) = o.get_mut("$ref")
                && r.starts_with("#/components/schemas/Form.")
                && r.ends_with("_1")
            {
                r.truncate(r.len() - 2);
            }
            for v in o.values_mut() {
                normalize_event_refs(v);
            }
        }
        Value::Array(a) => a.iter_mut().for_each(normalize_event_refs),
        _ => {}
    }
}
fn sort_enums(v: &mut Value) {
    match v {
        Value::Object(o) => {
            if let Some(Value::Array(a)) = o.get_mut("enum") {
                a.sort_by_key(Value::to_string);
            }
            for v in o.values_mut() {
                sort_enums(v);
            }
        }
        Value::Array(a) => a.iter_mut().for_each(sort_enums),
        _ => {}
    }
}
fn dispatch_union(
    schema: &mut Value,
    settings: &mut typify::TypeSpaceSettings,
    names: (&str, &str, &str),
    expected: &[&str],
    out: &mut String,
) -> Result<(), Box<dyn std::error::Error>> {
    let (wire, rust, tag_key) = names;
    let branches = schema["definitions"][wire]["anyOf"]
        .as_array()
        .ok_or("form union changed")?
        .clone();
    let mut variants = Vec::new();
    for b in branches {
        let branch_name = b["$ref"]
            .as_str()
            .and_then(|r| r.strip_prefix("#/definitions/"))
            .map(str::to_owned);
        let b = if let Some(r) = b["$ref"].as_str() {
            schema["definitions"][r
                .strip_prefix("#/definitions/")
                .ok_or("unexpected form ref")?]
            .clone()
        } else {
            b
        };
        let tag = b["properties"][tag_key]["enum"]
            .as_array()
            .filter(|a| a.len() == 1)
            .and_then(|a| a[0].as_str())
            .ok_or("form discriminator changed")?;
        if !b["required"]
            .as_array()
            .is_some_and(|a| a.contains(&json!(tag_key)))
        {
            return Err("form discriminator no longer required".into());
        }
        let variant = pascal(tag);
        let name = branch_name.unwrap_or_else(|| format!("{rust}{variant}"));
        schema["definitions"][&name] = b.clone();
        variants.push((variant, name, tag.to_owned()));
    }
    if variants
        .iter()
        .map(|(_, _, tag)| tag.as_str())
        .collect::<Vec<_>>()
        != expected
    {
        return Err("form union variants changed".into());
    }
    schema["definitions"][wire] = json!({});
    settings.with_replacement(pascal(wire), rust, std::iter::empty());
    out.push_str(&enum_dispatch_tag(rust, &variants, false, tag_key));
    Ok(())
}
