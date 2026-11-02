//! Reviewed adaptations for location, agent, and model discovery responses.

use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

pub fn generate_discovery(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    guard_catalog(spec)?;
    if spec.pointer("/components/schemas/Agent.Info/properties/steps")
        != Some(&json!({
            "type": "integer", "exclusiveMinimum": 0, "maximum": 9_007_199_254_740_991_u64
        }))
    {
        return Err("Agent.Info.steps changed; review its positive safe-integer adapter".into());
    }
    let mut spec = spec.clone();
    for (name, values) in [
        ("Model.Type", json!(["chat", "decision", "image"])),
        (
            "Model.ReasoningField",
            json!(["reasoning", "reasoning_content", "reasoning_text"]),
        ),
    ] {
        let schema = &mut spec["components"]["schemas"][name];
        if *schema != json!({"anyOf":[{"type":"string","enum":values},{"type":"string"}]}) {
            return Err(format!("{name} changed; review open-string simplification").into());
        }
        *schema = json!({"type":"string"});
    }
    let timeout_schema =
        json!({"anyOf": [{"type": "number"}, {"type": "boolean", "enum": [false]}]});
    for field in ["timeout", "headerTimeout", "chunkTimeout"] {
        let field_schema =
            &mut spec["components"]["schemas"]["Provider.Settings"]["properties"][field];
        if *field_schema != timeout_schema {
            return Err(
                format!("Provider.Settings.{field} changed; review its timeout adapter").into(),
            );
        }
        *field_schema = json!({"$ref": "#/components/schemas/ProviderTimeout"});
    }
    spec["components"]["schemas"]["ProviderTimeout"] = timeout_schema;
    let interval = spec
        .pointer_mut("/components/schemas/Provider.CacheWarming/anyOf/1/properties/interval")
        .ok_or("missing cache warming interval schema")?;
    if *interval != json!({"anyOf": [{"type": "string", "enum": ["auto"]}, {"type": "string"}]}) {
        return Err(
            "cache warming interval changed; review its string-union simplification".into(),
        );
    }
    // `"auto" | string` is exactly string. Typify otherwise emits a flattened object.
    *interval = json!({"type": "string"});
    let envelope = spec
        .pointer("/paths/~1api~1agent/get/responses/200/content/application~1json/schema")
        .ok_or("missing agent.list response schema")?
        .clone();
    spec["components"]["schemas"]["AgentListResponse"] = envelope;
    for (name, pointer) in [
        (
            "AgentResponse",
            "/paths/~1api~1agent~1{agentID}/get/responses/200/content/application~1json/schema",
        ),
        (
            "ModelListResponse",
            "/paths/~1api~1model/get/responses/200/content/application~1json/schema",
        ),
        (
            "ModelDefaultResponse",
            "/paths/~1api~1model~1default/get/responses/200/content/application~1json/schema",
        ),
    ] {
        let response = spec
            .pointer(pointer)
            .ok_or("discovery envelope missing")?
            .clone();
        spec["components"]["schemas"][name] = response;
    }
    let mut schema = select_schemas(
        &spec,
        &[
            "Location.PublicInfo",
            "AgentListResponse",
            "AgentResponse",
            "ModelListResponse",
            "ModelDefaultResponse",
        ],
    )?;
    allow_additive_fields(&mut schema);
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_replacement(
        "ProviderTimeout",
        "super::ProviderTimeout",
        std::iter::empty(),
    );
    settings.with_conversion(
        serde_json::from_value(json!({"type":"number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut steps_found = false;
    let mut catalog_fields = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) => {
                // Tuple newtypes remain constructible; response objects may grow fields.
                if matches!(item.fields, syn::Fields::Named(_)) {
                    item.attrs.push(parse_quote!(#[non_exhaustive]));
                }
                if item.ident == "AgentInfo" {
                    for field in &mut item.fields {
                        if field.ident.as_ref().is_some_and(|name| name == "steps") {
                            field.ty = parse_quote!(Option<u64>);
                            field.attrs.push(parse_quote!(#[serde(default, deserialize_with = "super::deserialize_steps")]));
                            steps_found = true;
                        }
                    }
                }
                for field in &mut item.fields {
                    let name = field
                        .ident
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_default();
                    match (item.ident.to_string().as_str(), name.as_str()) {
                        ("ModelInfoLimit", "context" | "output") | ("ModelCostTier", "size") => {
                            field.ty = parse_quote!(i64);
                            field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_model_integer")]));
                            catalog_fields += 1;
                        }
                        ("ModelInfoLimit", "input") => {
                            field.ty = parse_quote!(Option<i64>);
                            field.attrs.push(parse_quote!(#[serde(default,deserialize_with="super::deserialize_optional_model_integer")]));
                            catalog_fields += 1;
                        }
                        ("ModelDefaultResponse", "data") => {
                            field.attrs.retain(|a| !a.path().is_ident("serde"));
                            field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_nullable_model")]));
                            catalog_fields += 1;
                        }
                        _ => {}
                    }
                }
                if item.ident == "ProviderRequest"
                    || item.ident == "ModelInfo"
                    || item.ident == "ModelVariant"
                {
                    for attribute in &mut item.attrs {
                        if attribute.path().is_ident("derive") {
                            *attribute = parse_quote!(#[derive(::serde::Deserialize, ::serde::Serialize, Clone)]);
                        }
                    }
                }
            }
            Item::Enum(item) => item.attrs.push(parse_quote!(#[non_exhaustive])),
            _ => {}
        }
    }
    if !steps_found || catalog_fields != 5 {
        return Err("discovery fields changed; review model adaptations".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --discovery.\n// Do not edit manually; adaptations live in tools/model-spike/src/discovery.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn guard_catalog(spec: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let integer = json!({"type":"integer","minimum":-9_007_199_254_740_991_i64,"maximum":9_007_199_254_740_991_i64});
    for (pointer, field, required) in [
        (
            "/components/schemas/Model.Info/properties/limit",
            "context",
            true,
        ),
        (
            "/components/schemas/Model.Info/properties/limit",
            "input",
            false,
        ),
        (
            "/components/schemas/Model.Info/properties/limit",
            "output",
            true,
        ),
        (
            "/components/schemas/Model.Cost/properties/tier",
            "size",
            true,
        ),
    ] {
        let parent = spec.pointer(pointer).ok_or("model bound parent missing")?;
        if parent["properties"][field] != integer
            || parent["required"]
                .as_array()
                .is_some_and(|a| a.contains(&json!(field)))
                != required
        {
            return Err(format!("{pointer}.{field} changed; review integer adapter").into());
        }
    }
    let default = spec
        .pointer("/paths/~1api~1model~1default/get/responses/200/content/application~1json/schema")
        .ok_or("default model envelope missing")?;
    if default
        != &json!({"type":"object","properties":{"location":{"$ref":"#/components/schemas/Location.PublicRef"},"data":{"anyOf":[{"$ref":"#/components/schemas/Model.Info"},{"type":"null"}]}},"required":["location","data"],"additionalProperties":false})
    {
        return Err("default model envelope changed; review required nullable data".into());
    }
    Ok(())
}
