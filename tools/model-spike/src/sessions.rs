//! Reviewed session response and input models, reusing shared scalar types.

use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

pub fn generate_sessions(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    for name in ["additions", "deletions"] {
        if spec.pointer(&format!(
            "/components/schemas/FileDiff.Info/properties/{name}"
        )) != Some(
            &json!({"type": "integer", "minimum": 0, "maximum": 9_007_199_254_740_991_u64}),
        ) {
            return Err(
                format!("FileDiff.Info.{name} changed; review its safe-integer adapter").into(),
            );
        }
    }
    if spec.pointer("/components/schemas/FileDiff.Info/properties/more")
        != Some(&json!({"type": "boolean", "enum": [true]}))
    {
        return Err("FileDiff.Info.more changed; review its true-only adapter".into());
    }
    let mut spec = spec.clone();
    let create = spec
        .pointer("/paths/~1api~1session/post/responses/200/content/application~1json/schema")
        .ok_or("session.create response missing")?
        .clone();
    if spec.pointer(
        "/paths/~1api~1session~1{sessionID}/get/responses/200/content/application~1json/schema",
    ) != Some(&create)
    {
        return Err("session.create/get envelopes differ; review SessionResponse".into());
    }
    spec["components"]["schemas"]["SessionResponse"] = create;
    let mut schema = select_schemas(&spec, &["SessionsResponse", "SessionResponse"])?;
    allow_additive_fields(&mut schema);
    // Inputs keep strict objects and their own nested types. Never reuse tolerant
    // response objects whose extension maps could be sent back as request fields.
    add_request_schemas(&mut spec)?;
    let request = select_schemas(
        &spec,
        &[
            "SessionCreateRequest",
            "SessionUpdateRequest",
            "SessionSwitchAgentRequest",
            "SessionSwitchModelRequest",
        ],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(request["definitions"].as_object().unwrap().clone());
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "AbsolutePath",
        "AgentId",
        "LocationPublicRef",
        "ModelRef",
        "PermissionEffect",
        "PermissionRule",
        "PermissionRuleset",
        "ProjectId",
        "ProviderId",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    settings.with_conversion(
        serde_json::from_value(json!({"type": "number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut adapted_fields = 0;
    let mut request_fields = [0, 0];
    let mut model_variant = false;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                if item.ident == "SessionCreateRequest" || item.ident == "SessionUpdateRequest" {
                    // Typify defaults nullable optional properties to Option<T>.
                    // Replace those with a three-state field and skip only Omitted.
                    for field in &mut item.fields {
                        let syn::Type::Path(path) = &field.ty else {
                            return Err("unexpected request field type".into());
                        };
                        let segment = path
                            .path
                            .segments
                            .last()
                            .ok_or("request field type missing")?;
                        if segment.ident != "Option" {
                            return Err("request fields must remain optional".into());
                        }
                        let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
                            return Err("request Option arguments missing".into());
                        };
                        let Some(syn::GenericArgument::Type(inner)) = args.args.first() else {
                            return Err("request Option type missing".into());
                        };
                        let inner = inner.clone();
                        field.ty = parse_quote!(super::RequestField<#inner>);
                        field.attrs.retain(|attr| !attr.path().is_ident("serde"));
                        if field.ident.as_ref().is_some_and(|name| name == "parent_id") {
                            field
                                .attrs
                                .push(parse_quote!(#[serde(rename = "parentID")]));
                        }
                        field.attrs.push(parse_quote!(#[serde(default, skip_serializing_if = "super::RequestField::is_omitted")]));
                        request_fields[usize::from(item.ident == "SessionUpdateRequest")] += 1;
                    }
                }
                if item.ident == "SessionModelInput" {
                    for field in &mut item.fields {
                        if field.ident.as_ref().is_some_and(|name| name == "variant") {
                            field.attrs.push(parse_quote!(#[serde(default, deserialize_with = "super::deserialize_optional_value")]));
                            model_variant = true;
                        }
                    }
                }
                if item.ident == "FileDiffInfo" {
                    for field in &mut item.fields {
                        match field.ident.as_ref().map(ToString::to_string).as_deref() {
                            Some("additions" | "deletions") => {
                                field.ty = parse_quote!(u64);
                                field.attrs.push(parse_quote!(#[serde(deserialize_with = "super::deserialize_diff_count")]));
                                adapted_fields += 1;
                            }
                            Some("more") => {
                                field.attrs.push(parse_quote!(#[serde(default, deserialize_with = "super::deserialize_more")]));
                                adapted_fields += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }
            Item::Enum(item) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for variant in &mut item.variants {
                    if matches!(variant.fields, syn::Fields::Named(_)) {
                        variant.attrs.push(parse_quote!(#[non_exhaustive]));
                    }
                }
            }
            _ => {}
        }
    }
    if adapted_fields != 3 {
        return Err("FileDiffInfo fields changed; review the session model adapters".into());
    }
    if request_fields != [8, 4] || !model_variant {
        return Err("session input fields changed; review request adapters".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --sessions.\n// Do not edit manually; adaptations live in tools/model-spike/src/sessions.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn add_request_schemas(spec: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = nullable_request(
        spec,
        "/paths/~1api~1session/post/requestBody/content/application~1json/schema",
        &[
            "agent",
            "id",
            "location",
            "metadata",
            "model",
            "parentID",
            "permissions",
            "title",
        ],
    )?;
    let mut update = nullable_request(
        spec,
        "/paths/~1api~1session~1{sessionID}/patch/requestBody/content/application~1json/schema",
        &["archived", "metadata", "permissions", "title"],
    )?;
    if spec.pointer("/components/schemas/Model.Ref/properties/variant")
        != Some(&json!({"type": "string"}))
    {
        return Err("Model.Ref.variant changed; review the non-null input adapter".into());
    }
    let names = [
        ("Location.PublicRef", "SessionLocationInput"),
        ("Model.Ref", "SessionModelInput"),
        ("Permission.Rule", "PermissionRuleInput"),
        ("Permission.Ruleset", "PermissionRulesetInput"),
    ];
    rename_input_refs(&mut input, &names);
    rename_input_refs(&mut update, &names);
    for (original, renamed) in names {
        let mut schema = spec["components"]["schemas"][original].clone();
        if schema.is_null() {
            return Err(format!("missing {original}").into());
        }
        rename_input_refs(&mut schema, &names);
        spec["components"]["schemas"][renamed] = schema;
    }
    spec["components"]["schemas"]["SessionCreateRequest"] = input;
    spec["components"]["schemas"]["SessionUpdateRequest"] = update;
    for (name, route, field, reference) in [
        ("SessionSwitchAgentRequest", "agent", "agent", "Agent.ID"),
        ("SessionSwitchModelRequest", "model", "model", "Model.Ref"),
    ] {
        let pointer = format!(
            "/paths/~1api~1session~1{{sessionID}}~1{route}/post/requestBody/content/application~1json/schema"
        );
        let mut schema = spec
            .pointer(&pointer)
            .ok_or("session selection input missing")?
            .clone();
        if schema
            != json!({"type":"object","properties":{field:{"$ref":format!("#/components/schemas/{reference}")}},"required":[field],"additionalProperties":false})
        {
            return Err(format!("{name} changed; review required selection input").into());
        }
        rename_input_refs(&mut schema, &names);
        spec["components"]["schemas"][name] = schema;
    }
    Ok(())
}

fn nullable_request(
    spec: &Value,
    pointer: &str,
    expected_names: &[&str],
) -> Result<Value, Box<dyn std::error::Error>> {
    let input = spec
        .pointer(pointer)
        .ok_or("session request missing")?
        .clone();
    let properties = input["properties"]
        .as_object()
        .ok_or("request properties missing")?;
    let names: Vec<_> = properties.keys().map(String::as_str).collect();
    if names != expected_names
        || input
            .get("required")
            .is_some_and(|required| required != &json!([]))
        || input["additionalProperties"] != json!(false)
        || properties.values().any(|property| {
            property["anyOf"].as_array().is_none_or(|items| {
                items.len() != 2
                    || items[1] != json!({"type": "null"})
                    || items[0] == json!({"type": "null"})
            })
        })
    {
        return Err(format!(
            "{pointer}: fields must remain optional and nullable; review request adapters"
        )
        .into());
    }
    Ok(input)
}

pub(crate) fn rename_input_refs(value: &mut Value, names: &[(&str, &str)]) {
    match value {
        Value::Object(map) => {
            if let Some(Value::String(reference)) = map.get_mut("$ref") {
                for (original, renamed) in names {
                    if reference == &format!("#/components/schemas/{original}") {
                        *reference = format!("#/components/schemas/{renamed}");
                        break;
                    }
                }
            }
            for child in map.values_mut() {
                rename_input_refs(child, names);
            }
        }
        Value::Array(items) => items
            .iter_mut()
            .for_each(|value| rename_input_refs(value, names)),
        _ => {}
    }
}
