//! Reviewed prompt inputs and durable inbox responses.

use crate::{
    generate_with_settings, select_schemas, server_info::allow_additive_fields,
    sessions::rename_input_refs,
};
use serde_json::{Value, json};
use syn::{Item, parse_quote};

const PROMPT: &str = "/paths/~1api~1session~1{sessionID}~1prompt/post";
const UPDATE: &str = "/paths/~1api~1session~1{sessionID}~1inbox~1{inboxID}/patch";

pub fn generate_inbox(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    guard(spec)?;
    let mut spec = spec.clone();
    for (name,path) in [
        ("SessionPromptResponse",format!("{PROMPT}/responses/200/content/application~1json/schema")),
        ("SessionInboxResponse","/paths/~1api~1session~1{sessionID}~1inbox/get/responses/200/content/application~1json/schema".into()),
        ("SessionPromptRequest",format!("{PROMPT}/requestBody/content/application~1json/schema")),
        ("SessionInboxUpdateRequest",format!("{UPDATE}/requestBody/content/application~1json/schema")),
    ] {
        spec["components"]["schemas"][name]=spec.pointer(&path).ok_or("missing inbox endpoint schema")?.clone();
    }
    let mut schema = select_schemas(&spec, &["SessionPromptResponse", "SessionInboxResponse"])?;
    allow_additive_fields(&mut schema);
    let names = [
        ("Prompt.AgentAttachment", "PromptInputAgentAttachment"),
        ("Prompt.Mention", "PromptInputMention"),
        ("Prompt.FileSource", "PromptInputFileSource"),
    ];
    for (original, renamed) in names {
        let mut input = spec["components"]["schemas"][original].clone();
        rename_input_refs(&mut input, &names);
        spec["components"]["schemas"][renamed] = input;
    }
    for name in [
        "SessionPromptRequest",
        "PromptInput.FileAttachment",
        "PromptInput.SkillAttachment",
    ] {
        rename_input_refs(&mut spec["components"]["schemas"][name], &names);
    }
    let inputs = select_schemas(
        &spec,
        &["SessionPromptRequest", "SessionInboxUpdateRequest"],
    )?;
    schema["definitions"]
        .as_object_mut()
        .unwrap()
        .extend(inputs["definitions"].as_object().unwrap().clone());
    let mut settings = typify::TypeSpaceSettings::default();
    for name in [
        "AbsolutePath",
        "LocationPublicRef",
        "ProjectId",
        "RelativePath",
        "SessionId",
        "SessionMessageId",
        "SkillId",
        "SkillName",
        "PromptAgentAttachment",
        "PromptMention",
        "PromptFileAttachment",
        "PromptFileSource",
        "PromptBase64",
        "PromptSkillAttachment",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    settings.with_replacement(
        "SessionInboxInfo",
        "super::SessionInboxInfo",
        std::iter::empty(),
    );
    settings.with_replacement(
        "SessionInboxCompactionPayload",
        "super::NonNullValue",
        std::iter::empty(),
    );
    settings.with_conversion(
        serde_json::from_value(json!({"type":"number"}))?,
        "::serde_json::Number",
        std::iter::empty(),
    );
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    let mut nullable = 0;
    let mut optional = 0;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                let name = item.ident.to_string();
                let input = matches!(
                    name.as_str(),
                    "SessionPromptRequest"
                        | "PromptInputAgentAttachment"
                        | "PromptInputFileAttachment"
                        | "PromptInputSkillAttachment"
                );
                if !input {
                    continue;
                }
                for field in &mut item.fields {
                    let key = field.ident.as_ref().unwrap().to_string();
                    let is_nullable = name == "SessionPromptRequest"
                        && matches!(key.as_str(), "id" | "delivery" | "resume");
                    let is_optional = match name.as_str() {
                        "SessionPromptRequest" => key != "text",
                        "PromptInputFileAttachment" => key != "uri",
                        "PromptInputAgentAttachment" | "PromptInputSkillAttachment" => {
                            key == "mention"
                        }
                        _ => false,
                    };
                    if !is_optional {
                        continue;
                    }
                    let inner = option_inner(&field.ty).unwrap_or_else(|| field.ty.clone());
                    field.attrs.retain(|a| !a.path().is_ident("serde"));
                    if is_nullable {
                        if option_inner(&field.ty).is_none() {
                            return Err("nullable prompt field changed".into());
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
                if item.ident == "PromptInputFileSource" {
                    // Typify's adjacent-tag decoder accepts extra fields, including
                    // a null `uri` for the unit variant. Use a strict decoder.
                    for attr in &mut item.attrs {
                        if attr.path().is_ident("derive") {
                            *attr = parse_quote!(#[derive(::serde::Serialize, Clone, Debug)]);
                        }
                    }
                }
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
    if nullable != 3 || optional != 10 {
        return Err("prompt input fields changed; review optionality adapters".into());
    }
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --inbox.\n// Do not edit manually; adaptations live in tools/model-spike/src/inbox.rs.\n{}",
        prettyplease::unparse(&syntax)
    ))
}

pub(crate) fn option_inner(ty: &syn::Type) -> Option<syn::Type> {
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

pub(crate) fn guard(spec: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let nullable = |value: Value| json!({"anyOf":[value,{"type":"null"}]});
    let array = |name: &str| json!({"type":"array","items":reference(name)});
    let object = |properties: Value, required: Value| json!({"type":"object","properties":properties,"required":required,"additionalProperties":false});
    let string = json!({"type":"string"});
    if spec.pointer("/components/schemas/Prompt.FileSource")
        != Some(&json!({"anyOf":[
            object(json!({"type":{"type":"string","enum":["inline"]}}),json!(["type"])),
            object(json!({"type":{"type":"string","enum":["uri"]},"uri":string}),json!(["type","uri"]))
        ]}))
    {
        return Err("Prompt.FileSource changed; review strict source decoder".into());
    }
    for (pointer, expected) in [
        (
            format!("{PROMPT}/requestBody/content/application~1json/schema"),
            object(
                json!({
                    "id":nullable(reference("Session.Message.ID")),"text":string,"files":array("PromptInput.FileAttachment"),"agents":array("Prompt.AgentAttachment"),"skills":array("PromptInput.SkillAttachment"),"metadata":{"type":"object"},"delivery":nullable(reference("Session.Inbox.Delivery")),"resume":nullable(json!({"type":"boolean"}))
                }),
                json!(["text"]),
            ),
        ),
        (
            format!("{UPDATE}/requestBody/content/application~1json/schema"),
            object(
                json!({"delivery":reference("Session.Inbox.Delivery")}),
                json!(["delivery"]),
            ),
        ),
        (
            "/components/schemas/PromptInput.FileAttachment".into(),
            object(
                json!({"uri":string,"source":reference("Prompt.FileSource"),"name":string,"description":string,"mention":reference("Prompt.Mention")}),
                json!(["uri"]),
            ),
        ),
        (
            "/components/schemas/Prompt.AgentAttachment".into(),
            object(
                json!({"name":string,"mention":reference("Prompt.Mention")}),
                json!(["name"]),
            ),
        ),
        (
            "/components/schemas/PromptInput.SkillAttachment".into(),
            object(
                json!({"id":reference("Skill.ID"),"mention":reference("Prompt.Mention")}),
                json!(["id"]),
            ),
        ),
        (
            "/components/schemas/Session.Inbox.CompactionPayload".into(),
            json!({"not":{"type":"null"}}),
        ),
        (
            "/components/schemas/Session.Inbox.Info".into(),
            json!({"anyOf":[reference("Session.Inbox.User"),reference("Session.Inbox.Synthetic"),reference("Session.Inbox.Compaction"),reference("Session.Inbox.Move")]}),
        ),
    ] {
        if spec.pointer(&pointer) != Some(&expected) {
            return Err(format!("{pointer} changed; review inbox adapters").into());
        }
    }
    for (name, tag) in [
        ("User", "user"),
        ("Synthetic", "synthetic"),
        ("Compaction", "compaction"),
        ("Move", "move"),
    ] {
        let schema = &spec["components"]["schemas"][format!("Session.Inbox.{name}")];
        if schema["properties"]["type"] != json!({"type":"string","enum":[tag]})
            || !["id", "sessionID", "type", "payload", "time", "delivery"]
                .iter()
                .all(|key| {
                    schema["required"]
                        .as_array()
                        .is_some_and(|r| r.contains(&json!(key)))
                })
        {
            return Err(
                format!("inbox {name} tag/required fields changed; review dispatch").into(),
            );
        }
    }
    Ok(())
}
