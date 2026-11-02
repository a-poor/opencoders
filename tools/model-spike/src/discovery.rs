//! Reviewed adaptations for location and agent discovery response models.

use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

pub fn generate_discovery(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    if spec.pointer("/components/schemas/Agent.Info/properties/steps")
        != Some(&json!({
            "type": "integer", "exclusiveMinimum": 0, "maximum": 9_007_199_254_740_991_u64
        }))
    {
        return Err("Agent.Info.steps changed; review its positive safe-integer adapter".into());
    }
    let mut spec = spec.clone();
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
    let mut schema = select_schemas(&spec, &["Location.PublicInfo", "AgentListResponse"])?;
    allow_additive_fields(&mut schema);
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_replacement(
        "ProviderTimeout",
        "super::ProviderTimeout",
        std::iter::empty(),
    );
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut steps_found = false;
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
                if item.ident == "ProviderRequest" {
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
    if !steps_found {
        return Err("AgentInfo.steps missing; review the discovery model adaptation".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --discovery.\n// Do not edit manually; adaptations live in tools/model-spike/src/discovery.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}
