//! Reviewed projected message responses. No message mutation inputs are generated here.

use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

pub fn generate_messages(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    guard_unions(spec)?;
    // These adapters supply defaults only for fields that are actually optional.
    for (pointer, field, required) in [
        (
            "/components/schemas/Session.Message.Shell/properties/output",
            "cursor",
            true,
        ),
        (
            "/components/schemas/Session.Message.Shell/properties/output",
            "size",
            true,
        ),
        (
            "/components/schemas/Session.StructuredError",
            "status",
            false,
        ),
        (
            "/components/schemas/Session.StructuredError",
            "retryAfterMs",
            false,
        ),
        (
            "/components/schemas/Session.Message.Assistant.Retry",
            "attempt",
            true,
        ),
        (
            "/components/schemas/Session.Message.ToolState.Completed",
            "content",
            true,
        ),
        (
            "/components/schemas/Session.Message.ToolState.Error",
            "content",
            false,
        ),
    ] {
        let schema = spec
            .pointer(pointer)
            .ok_or("message adapter schema missing")?;
        if schema["required"]
            .as_array()
            .is_some_and(|fields| fields.contains(&json!(field)))
            != required
        {
            return Err(format!("{pointer}.{field} optionality changed; review adapters").into());
        }
    }
    let safe = json!({"type":"integer","minimum":0,"maximum":9_007_199_254_740_991_u64});
    for (pointer, expected) in [
        (
            "/components/schemas/Session.Message.Shell/properties/output/properties/cursor",
            &safe,
        ),
        (
            "/components/schemas/Session.Message.Shell/properties/output/properties/size",
            &safe,
        ),
        (
            "/components/schemas/Session.StructuredError/properties/retryAfterMs",
            &safe,
        ),
        (
            "/components/schemas/Session.StructuredError/properties/status",
            &json!({"type":"integer","minimum":100,"maximum":599}),
        ),
        (
            "/components/schemas/Session.Message.Assistant.Retry/properties/attempt",
            &json!({"type":"integer","maximum":9_007_199_254_740_991_u64,"exclusiveMinimum":0}),
        ),
    ] {
        if spec.pointer(pointer) != Some(expected) {
            return Err(format!("{pointer} changed; review message numeric adapters").into());
        }
    }
    let mut spec = spec.clone();
    let response=spec.pointer("/paths/~1api~1session~1{sessionID}~1message~1{messageID}/get/responses/200/content/application~1json/schema").ok_or("message.get response missing")?.clone();
    spec["components"]["schemas"]["SessionMessageResponse"] = response;
    let parameters = spec
        .pointer("/paths/~1api~1session~1{sessionID}~1message/get/parameters")
        .and_then(Value::as_array)
        .ok_or("message.list parameters missing")?;
    let filter = parameters
        .iter()
        .find(|p| p["name"] == "type")
        .ok_or("message type filter missing")?["schema"]["anyOf"]
        .as_array()
        .filter(|v| v.len() == 1)
        .ok_or("message type filter shape changed")?[0]
        .clone();
    spec["components"]["schemas"]["SessionMessageType"] = filter;
    let mut schema = select_schemas(
        &spec,
        &[
            "SessionMessagesResponse",
            "SessionMessageResponse",
            "SessionMessageType",
        ],
    )?;
    allow_additive_fields(&mut schema);
    let mut settings = typify::TypeSpaceSettings::default();
    settings.with_replacement(
        "SessionMessageInfo",
        "super::SessionMessageInfo",
        std::iter::empty(),
    );
    for name in [
        "AbsolutePath",
        "AgentId",
        "LocationPublicRef",
        "ModelRef",
        "MoneyUsd",
        "ProjectId",
        "ProviderId",
        "RelativePath",
        "SessionMessageId",
        "SnapshotId",
        "TokenUsageInfo",
        "TokenUsageInfoCache",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    settings.with_conversion(
        serde_json::from_value(json!({"type":"number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut adapted = 0;
    let mut unions = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for field in &mut item.fields {
                    let name = field.ident.as_ref().unwrap().to_string();
                    match (item.ident.to_string().as_str(), name.as_str()) {
                        ("SessionMessageShellOutput", "cursor" | "size") => {
                            field.ty = parse_quote!(u64);
                            field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_diff_count")]));
                            adapted += 1;
                        }
                        ("SessionStructuredError", "status") => {
                            field.ty = parse_quote!(Option<u16>);
                            field.attrs.push(parse_quote!(#[serde(default,deserialize_with="super::deserialize_message_status")]));
                            adapted += 1;
                        }
                        ("SessionStructuredError", "retry_after_ms") => {
                            field.ty = parse_quote!(Option<u64>);
                            field.attrs.push(parse_quote!(#[serde(default,deserialize_with="super::deserialize_message_retry_delay")]));
                            adapted += 1;
                        }
                        ("SessionMessageAssistantRetry", "attempt") => {
                            field.ty = parse_quote!(u64);
                            field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_message_attempt")]));
                            adapted += 1;
                        }
                        (
                            "SessionMessageToolStateCompleted" | "SessionMessageToolStateError",
                            "content",
                        ) => {
                            field.attrs.push(parse_quote!(#[serde(deserialize_with="super::deserialize_tool_content")]));
                            adapted += 1;
                        }
                        _ => {}
                    }
                }
            }
            Item::Enum(item) => {
                if item.ident == "SessionMessageAssistantContentItem" {
                    let variant = item
                        .variants
                        .iter_mut()
                        .find(|v| v.ident == "Tool")
                        .ok_or("tool content variant missing")?;
                    let field = variant
                        .fields
                        .iter_mut()
                        .next()
                        .ok_or("tool content payload missing")?;
                    field.ty = parse_quote!(Box<SessionMessageAssistantTool>);
                }
                if [
                    "SessionMessageCompaction",
                    "SessionMessageAssistantContentItem",
                    "SessionMessageAssistantToolState",
                    "ToolContent",
                ]
                .iter()
                .any(|name| item.ident == *name)
                {
                    item.attrs.retain(|a| !a.path().is_ident("derive"));
                    item.attrs
                        .insert(0, parse_quote!(#[derive(::serde::Serialize,Clone,Debug)]));
                    unions += 1;
                }
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for variant in &mut item.variants {
                    if matches!(variant.fields, syn::Fields::Named(_)) {
                        variant.attrs.push(parse_quote!(#[non_exhaustive]));
                    }
                }
            }
            Item::Impl(item) if matches!(&*item.self_ty,syn::Type::Path(p) if p.path.is_ident("SessionMessageAssistantContentItem")) => {
                for method in &mut item.items {
                    if let syn::ImplItem::Fn(method) = method
                        && method.sig.inputs.iter().any(|arg|matches!(arg,syn::FnArg::Typed(arg) if matches!(&*arg.ty,syn::Type::Path(p) if p.path.is_ident("SessionMessageAssistantTool")))) {
                        method.block=parse_quote!({Self::Tool(Box::new(value))});
                    }
                }
            }
            _ => {}
        }
    }
    if adapted != 7 || unions != 4 {
        return Err("message fields/unions changed; review adapters".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --messages.\n// Do not edit manually; adaptations live in tools/model-spike/src/messages.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

fn guard_unions(spec: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let schemas = &spec["components"]["schemas"];
    for (pointer, tag, variants) in [
        (
            "/components/schemas/Session.Message.Info",
            "type",
            vec![
                ("Session.Message.AgentSelected", "agent-switched"),
                ("Session.Message.ModelSelected", "model-switched"),
                ("Session.Message.LocationSwitched", "location-switched"),
                ("Session.Message.User", "user"),
                ("Session.Message.Synthetic", "synthetic"),
                ("Session.Message.System", "system"),
                ("Session.Message.Skill", "skill"),
                ("Session.Message.Shell", "shell"),
                ("Session.Message.Assistant", "assistant"),
                ("Session.Message.Compaction", "compaction"),
                ("Session.Message.Idle", "idle"),
            ],
        ),
        (
            "/components/schemas/Session.Message.Compaction",
            "status",
            vec![
                ("Session.Message.Compaction.Running", "running"),
                ("Session.Message.Compaction.Completed", "completed"),
                ("Session.Message.Compaction.Failed", "failed"),
            ],
        ),
        (
            "/components/schemas/Session.Message.Assistant/properties/content/items",
            "type",
            vec![
                ("Session.Message.Assistant.Text", "text"),
                ("Session.Message.Assistant.Reasoning", "reasoning"),
                ("Session.Message.Assistant.Tool", "tool"),
            ],
        ),
        (
            "/components/schemas/Session.Message.Assistant.Tool/properties/state",
            "status",
            vec![
                ("Session.Message.ToolState.Streaming", "streaming"),
                ("Session.Message.ToolState.Running", "running"),
                ("Session.Message.ToolState.Completed", "completed"),
                ("Session.Message.ToolState.Error", "error"),
            ],
        ),
        (
            "/components/schemas/Tool.Content",
            "type",
            vec![("Tool.TextContent", "text"), ("Tool.FileContent", "file")],
        ),
    ] {
        let expected: Vec<_> = variants
            .iter()
            .map(|(name, _)| json!({"$ref":format!("#/components/schemas/{name}")}))
            .collect();
        if spec.pointer(pointer) != Some(&json!({"anyOf":expected})) {
            return Err(format!("{pointer} changed; review tag dispatch").into());
        }
        for (name, value) in variants {
            let leaves = if name == "Session.Message.Compaction" {
                vec![
                    "Session.Message.Compaction.Running",
                    "Session.Message.Compaction.Completed",
                    "Session.Message.Compaction.Failed",
                ]
            } else {
                vec![name]
            };
            for leaf in leaves {
                if schemas[leaf]["properties"][tag] != json!({"type":"string","enum":[value]})
                    || !schemas[leaf]["required"]
                        .as_array()
                        .is_some_and(|a| a.contains(&json!(tag)))
                {
                    return Err(format!("{leaf}.{tag} changed; review tag dispatch").into());
                }
            }
        }
    }
    for name in [
        "Session.Message.ToolState.Completed",
        "Session.Message.ToolState.Error",
    ] {
        if schemas[name]["properties"]["content"]
            != json!({"type":"array","prefixItems":[{"$ref":"#/components/schemas/Tool.Content"}],"minItems":1,"items":{"$ref":"#/components/schemas/Tool.Content"}})
        {
            return Err("tool content schema changed; review nonempty adapter".into());
        }
    }
    Ok(())
}
