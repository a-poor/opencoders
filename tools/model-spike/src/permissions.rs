//! Reviewed permission envelopes and strict inputs with explicit optionality.
use crate::{
    generate_with_settings, select_schemas, server_info::allow_additive_fields,
    sessions::rename_input_refs,
};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

const SESSION: &str = "/paths/~1api~1session~1{sessionID}~1permission";
const REPLY: &str = "/paths/~1api~1session~1{sessionID}~1permission~1{requestID}~1reply/post";
pub fn generate_permissions(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let mut spec = spec.clone();
    for (name,path) in [
        ("PermissionRequestListResponse","/paths/~1api~1permission~1request/get/responses/200/content/application~1json/schema".into()),
        ("SessionPermissionListResponse",format!("{SESSION}/get/responses/200/content/application~1json/schema")),
        ("SessionPermissionResponse","/paths/~1api~1session~1{sessionID}~1permission~1{requestID}/get/responses/200/content/application~1json/schema".into()),
        ("SessionPermissionCreateResponse",format!("{SESSION}/post/responses/200/content/application~1json/schema")),
        ("SessionPermissionCreateRequest",format!("{SESSION}/post/requestBody/content/application~1json/schema")),
        ("SessionPermissionReplyRequest",format!("{REPLY}/requestBody/content/application~1json/schema")),
    ] { spec["components"]["schemas"][name]=spec.pointer(&path).ok_or("missing permission endpoint schema")?.clone(); }
    guard(&spec)?;
    let mut schema = select_schemas(
        &spec,
        &[
            "PermissionRequestListResponse",
            "SessionPermissionListResponse",
            "SessionPermissionResponse",
            "SessionPermissionCreateResponse",
        ],
    )?;
    allow_additive_fields(&mut schema);
    spec["components"]["schemas"]["PermissionSourceInput"] =
        spec["components"]["schemas"]["Permission.Source"].clone();
    rename_input_refs(
        &mut spec["components"]["schemas"]["SessionPermissionCreateRequest"],
        &[("Permission.Source", "PermissionSourceInput")],
    );
    let inputs = select_schemas(
        &spec,
        &[
            "SessionPermissionCreateRequest",
            "SessionPermissionReplyRequest",
        ],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "AgentId",
        "SessionId",
        "PermissionId",
        "PermissionEffect",
        "PermissionReply",
        "PermissionSource",
        "LocationPublicRef",
        "AbsolutePath",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut nullable = 0;
    let mut optional = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                let name = item.ident.to_string();
                if !matches!(
                    name.as_str(),
                    "SessionPermissionCreateRequest" | "SessionPermissionReplyRequest"
                ) {
                    continue;
                }
                for field in &mut item.fields {
                    let key = field.ident.as_ref().unwrap().to_string();
                    let is_nullable = matches!(key.as_str(), "id" | "agent" | "message");
                    if !is_nullable && !matches!(key.as_str(), "save" | "metadata" | "source") {
                        continue;
                    }
                    let inner = option_inner(&field.ty).unwrap_or_else(|| field.ty.clone());
                    field.attrs.retain(|a| !a.path().is_ident("serde"));
                    if is_nullable {
                        if option_inner(&field.ty).is_none() {
                            return Err("nullable permission input changed".into());
                        }
                        field.ty = parse_quote!(super::RequestField<#inner>);
                        field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="super::RequestField::is_omitted")]));
                        nullable += 1;
                    } else {
                        field.ty = parse_quote!(Option<#inner>);
                        field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                        optional += 1;
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
    if nullable != 3 || optional != 3 {
        return Err("permission input fields changed; review optionality adapters".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --permissions.\n// Do not edit manually; adaptations live in tools/model-spike/src/permissions.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}
fn option_inner(ty: &syn::Type) -> Option<syn::Type> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    let syn::GenericArgument::Type(inner) = args.args.first()? else {
        return None;
    };
    Some(inner.clone())
}
fn guard(spec: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let schemas = &spec["components"]["schemas"];
    let source = json!({"anyOf":[{"type":"object","properties":{"type":{"type":"string","enum":["tool"]},"messageID":{"type":"string"},"id":{"type":"string"}},"required":["type","messageID","id"],"additionalProperties":false}]});
    if schemas["Permission.Source"] != source {
        return Err("Permission.Source changed; review strict input adapter".into());
    }
    for (name, required, properties) in [
        (
            "SessionPermissionCreateRequest",
            json!(["action", "resources"]),
            json!({
            "id":{"anyOf":[{"$ref":"#/components/schemas/Permission.ID"},{"type":"null"}]},
            "agent":{"anyOf":[{"$ref":"#/components/schemas/Agent.ID"},{"type":"null"}]},
            "action":{"type":"string"},"resources":{"type":"array","items":{"type":"string"}},
            "save":{"type":"array","items":{"type":"string"}},"metadata":{"type":"object"},"source":{"$ref":"#/components/schemas/Permission.Source"}}),
        ),
        (
            "SessionPermissionReplyRequest",
            json!(["decision"]),
            json!({"decision":{"$ref":"#/components/schemas/Permission.Reply"},"message":{"anyOf":[{"type":"string"},{"type":"null"}]}}),
        ),
    ] {
        if schemas[name]
            != json!({"type":"object","properties":properties,"required":required,"additionalProperties":false})
        {
            return Err(format!("{name} changed; review permission input adapters").into());
        }
    }
    Ok(())
}
