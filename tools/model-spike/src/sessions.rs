//! Reviewed session-list response models, reusing shared discovery model types.

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
    let mut schema = select_schemas(spec, &["SessionsResponse"])?;
    allow_additive_fields(&mut schema);
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
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
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
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --sessions.\n// Do not edit manually; adaptations live in tools/model-spike/src/sessions.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}
