//! Reviewed adaptations for the first production response model.

use crate::{generate, select_schemas};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

/// Generate only ServerInfo and its nested types; leave upstream bytes intact.
pub fn generate_server_info(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    if spec.pointer("/components/schemas/ServerInfo/properties/pid")
        != Some(&json!({
            "type": "integer", "minimum": 0, "maximum": 9_007_199_254_740_991_u64
        }))
    {
        return Err("ServerInfo.pid schema changed; review its integer-range adapter".into());
    }
    let mut schema = select_schemas(spec, &["ServerInfo"])?;
    allow_additive_fields(&mut schema);
    let mut syntax = syn::parse_file(&generate(schema, false)?)?;
    let mut pid_found = false;
    for item in &mut syntax.items {
        if let Item::Struct(item) = item {
            item.attrs.push(parse_quote!(#[non_exhaustive]));
            if item.ident == "ServerInfo" {
                for field in &mut item.fields {
                    if field.ident.as_ref().is_some_and(|name| name == "pid") {
                        field.ty = parse_quote!(u64);
                        field.attrs.push(
                            parse_quote!(#[serde(deserialize_with = "super::deserialize_pid")]),
                        );
                        pid_found = true;
                    }
                }
            }
        }
    }
    if !pid_found {
        return Err("ServerInfo.pid changed; review the production model adaptation".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --server-info.\n// Do not edit manually; adaptations live in tools/model-spike/src/server_info.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

pub(crate) fn allow_additive_fields(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.get("additionalProperties") == Some(&Value::Bool(false)) {
                // Unlike `true`, `{}` makes Typify retain a flattened map.
                map.insert("additionalProperties".into(), json!({}));
            }
            for child in map.values_mut() {
                allow_additive_fields(child);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(allow_additive_fields),
        _ => {}
    }
}
