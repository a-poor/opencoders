//! Reviewed stable command, skill and session workflow contracts.
use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

type GenerationResult = Result<String, Box<dyn std::error::Error>>;
fn request(spec: &Value, suffix: &str) -> Value {
    spec["paths"][format!("/api/session/{{sessionID}}/{suffix}")]["post"]["requestBody"]["content"]
        ["application/json"]["schema"]
        .clone()
}
fn response(spec: &Value, path: &str, method: &str) -> Value {
    spec["paths"][path][method]["responses"]["200"]["content"]["application/json"]["schema"].clone()
}

pub fn generate_workflows(spec: &Value) -> GenerationResult {
    // The inbox target owns the shared strict attachment contracts.
    crate::inbox::guard(spec)?;
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let nullable = |v: Value| json!({"anyOf":[v,{"type":"null"}]});
    let array = |name: &str| json!({"type":"array","items":reference(name)});
    let command = request(spec, "command");
    let compact = request(spec, "compact");
    let stage = request(spec, "revert/stage");
    if command
        != json!({"type":"object","properties":{
        "name":{"type":"string"},"text":{"type":"string"},
        "files":array("PromptInput.FileAttachment"),"agents":array("Prompt.AgentAttachment"),
        "skills":array("PromptInput.SkillAttachment"),"delivery":nullable(reference("Session.Inbox.Delivery"))
    },"required":["name","text"],"additionalProperties":false})
        || compact
            != json!({"type":"object","properties":{
            "id":nullable(reference("Session.Message.ID")),"delivery":nullable(reference("Session.Inbox.Delivery"))
        },"additionalProperties":false})
        || stage
            != json!({"type":"object","properties":{
            "messageID":reference("Session.Message.ID"),"files":nullable(json!({"type":"boolean"}))
        },"required":["messageID"],"additionalProperties":false})
    {
        return Err(
            "session workflow input contract changed; review strict fields and omission/null"
                .into(),
        );
    }
    let mut spec = spec.clone();
    let roots = [
        (
            "CommandListResponse",
            "/api/command",
            "get",
            "Command.Info",
            true,
            true,
        ),
        (
            "SkillListResponse",
            "/api/skill",
            "get",
            "Skill.Info",
            true,
            true,
        ),
        (
            "SessionContextResponse",
            "/api/session/{sessionID}/context",
            "get",
            "Session.Message.Info",
            true,
            false,
        ),
        (
            "SessionDiffResponse",
            "/api/session/{sessionID}/diff",
            "get",
            "FileDiff.Info",
            true,
            false,
        ),
        (
            "SessionCompactResponse",
            "/api/session/{sessionID}/compact",
            "post",
            "Session.Inbox.Compaction",
            false,
            false,
        ),
        (
            "SessionRevertResponse",
            "/api/session/{sessionID}/revert/stage",
            "post",
            "Session.Revert",
            false,
            false,
        ),
    ];
    for (name, path, method, wire, is_array, location) in roots {
        let r = response(&spec, path, method);
        let data = if is_array {
            array(wire)
        } else {
            reference(wire)
        };
        let required = if location {
            json!(["location", "data"])
        } else {
            json!(["data"])
        };
        if r["properties"]["data"] != data
            || r["required"] != required
            || (location && r["properties"]["location"] != reference("Location.PublicRef"))
        {
            return Err(format!("{name} envelope changed; review shared response types").into());
        }
        spec["components"]["schemas"][name] = r;
    }
    // These types are generated/adapted by existing checked targets. Treat their
    // definitions as external leaves, so this target cannot emit duplicate wire types.
    let shared = [
        (
            "Location.PublicRef",
            "LocationPublicRef",
            "LocationPublicRef",
        ),
        ("AbsolutePath", "AbsolutePath", "AbsolutePath"),
        ("Skill.ID", "SkillId", "SkillId"),
        ("Skill.Name", "SkillName", "SkillName"),
        (
            "Session.Message.Info",
            "SessionMessageInfo",
            "SessionMessageInfo",
        ),
        ("Session.Message.ID", "SessionMessageId", "SessionMessageId"),
        ("FileDiff.Info", "FileDiffInfo", "FileDiffInfo"),
        ("Session.Revert", "SessionRevert", "SessionRevert"),
        (
            "Session.Inbox.Compaction",
            "SessionInboxCompaction",
            "SessionInboxCompaction",
        ),
        (
            "Session.Inbox.Delivery",
            "SessionInboxDelivery",
            "SessionInboxDelivery",
        ),
        (
            "PromptInput.FileAttachment",
            "PromptInputFileAttachment",
            "PromptInputFileAttachment",
        ),
        (
            "Prompt.AgentAttachment",
            "PromptAgentAttachment",
            "PromptInputAgentAttachment",
        ),
        (
            "PromptInput.SkillAttachment",
            "PromptInputSkillAttachment",
            "PromptInputSkillAttachment",
        ),
    ];
    let mut settings = typify::TypeSpaceSettings::default();
    for (wire, name, rust) in shared {
        if spec["components"]["schemas"].get(wire).is_none() {
            return Err(format!("missing shared schema {wire}").into());
        }
        spec["components"]["schemas"][wire] = json!({});
        settings.with_replacement(name, format!("super::{rust}"), std::iter::empty());
    }
    let names: Vec<_> = roots.iter().map(|(name, ..)| *name).collect();
    let mut schema = select_schemas(&spec, &names)?;
    allow_additive_fields(&mut schema);
    for (name, value) in [
        ("SessionCommandRequest", command),
        ("SessionCompactRequest", compact),
        ("SessionRevertStageRequest", stage),
    ] {
        spec["components"]["schemas"][name] = value;
    }
    let inputs = select_schemas(
        &spec,
        &[
            "SessionCommandRequest",
            "SessionCompactRequest",
            "SessionRevertStageRequest",
        ],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut adapted = 0;
    for item in &mut syntax.items {
        if let Item::Struct(item) = item {
            if !matches!(item.fields, syn::Fields::Named(_)) {
                continue;
            }
            item.attrs.push(parse_quote!(#[non_exhaustive]));
            let name = item.ident.to_string();
            for field in &mut item.fields {
                let key = field.ident.as_ref().unwrap().to_string();
                let nullable = match name.as_str() {
                    "SessionCommandRequest" => key == "delivery",
                    "SessionCompactRequest" => true,
                    "SessionRevertStageRequest" => key == "files",
                    _ => false,
                };
                let optional = name == "SessionCommandRequest"
                    && matches!(key.as_str(), "files" | "agents" | "skills");
                if !nullable && !optional {
                    continue;
                }
                let inner =
                    crate::inbox::option_inner(&field.ty).unwrap_or_else(|| field.ty.clone());
                field.attrs.retain(|a| !a.path().is_ident("serde"));
                if nullable {
                    field.ty = parse_quote!(super::RequestField<#inner>);
                    field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="super::RequestField::is_omitted")]));
                } else {
                    field.ty = parse_quote!(Option<#inner>);
                    field.attrs.push(parse_quote!(#[serde(default,skip_serializing_if="Option::is_none",deserialize_with="super::deserialize_optional_value")]));
                }
                adapted += 1;
            }
        }
    }
    if adapted != 7 {
        return Err("workflow optional fields changed".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --workflows.\n// Do not edit manually; adaptations live in tools/model-spike/src/workflows.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}
