//! Production event envelopes, strict tag dispatch, and constrained payloads.
use crate::{generate_with_settings, select_schemas, server_info::allow_additive_fields};
use serde::Deserialize;
use serde_json::{Value, json};
use syn::{Item, parse_quote};

#[derive(Deserialize)]
struct Root {
    schema: String,
    variant: String,
    r#type: String,
}

pub fn generate_events(spec: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let roots: Vec<Root> = serde_json::from_str(include_str!("../event-roots.json"))?;
    let mut spec = spec.clone();
    let mut names = Vec::new();
    for root in &roots {
        let schema = spec["components"]["schemas"][&root.schema].clone();
        if !(spec["components"]["schemas"]["V2Event"]["anyOf"]
            .as_array()
            .ok_or("event union missing")?
            .contains(&json!({"$ref":format!("#/components/schemas/{}",root.schema)}))
            || spec["components"]["schemas"]["Session.Event.Durable"]["anyOf"]
                .as_array()
                .ok_or("durable event union missing")?
                .contains(&json!({"$ref":format!("#/components/schemas/{}",root.schema)})))
            || (root.r#type != "rpc.*"
                && schema["properties"]["type"] != json!({"type":"string","enum":[root.r#type]}))
        {
            return Err(format!("event root {} changed; review dispatch", root.schema).into());
        }
        let name = format!("Event{}", root.variant);
        spec["components"]["schemas"][&name] = schema;
        names.push(name);
    }
    names.push("EventLog.Synced".into());
    let mut schema = select_schemas(&spec, &names.iter().map(String::as_str).collect::<Vec<_>>())?;
    guard_constraints(&schema)?;
    crate::forms::normalize_form_values(&mut schema, "_1")?;
    normalize_instruction_delta(&mut schema)?;
    schema["definitions"]["EventLog.Synced"]["description"] = json!(
        "Captured replay watermark over retained log rows. Not proof of complete event retention or reconstructible state. The sequence may exceed emitted events or precede a requested future cursor; absence represents an empty watermark."
    );
    if spec["components"]["schemas"]["V2Event.rpc"]["properties"]["type"]
        != json!({"type":"string","pattern":"^rpc\\.[\\s\\S]*?$"})
    {
        return Err("RPC discriminator changed; review prefix dispatch".into());
    }
    let mut settings = typify::TypeSpaceSettings::default();
    // Share previously reviewed wire types; Location.Ref is deliberately distinct
    // from Location.PublicRef because its workspaceID has a different contract.
    for name in [
        "AbsolutePath",
        "AgentId",
        "LocationPublicRef",
        "ModelRef",
        "MoneyUsd",
        "PermissionRule",
        "PermissionRuleset",
        "ProjectId",
        "ProviderId",
        "RelativePath",
        "SessionId",
        "SessionMessageId",
        "SessionMetadata",
        "SessionStructuredError",
        "SessionMessageNative",
        "SessionForkBoundary",
        "SnapshotId",
        "TokenUsageInfo",
        "ToolContent",
        "SessionInboxUserPayload",
        "SessionInboxSyntheticPayload",
        "SessionInboxDelivery",
        "SkillId",
        "SkillName",
        "PromptAgentAttachment",
        "PromptMention",
        "PromptFileAttachment",
        "PromptFileSource",
        "PromptBase64",
        "PromptSkillAttachment",
        "ShellId",
        "SessionRevert",
        "SessionMessageAssistantText",
        "SessionMessageAssistantReasoning",
        "SessionMessageAssistantTool",
    ] {
        settings.with_replacement(name, format!("super::{name}"), std::iter::empty());
    }
    settings.with_replacement(
        "SessionMessageAssistantContentEncoded",
        "super::SessionMessageAssistantContentItem",
        std::iter::empty(),
    );
    for (wire, rust) in [
        (json!({"type":"number"}), "::serde_json::Number"),
        (json!({"not":{"type":"null"}}), "super::NonNullValue"),
        (
            json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64}),
            "super::EventInteger<0,9007199254740991>",
        ),
        (
            json!({"type":"integer","exclusiveMinimum":0,"maximum":9007199254740991_u64}),
            "super::EventInteger<1,9007199254740991>",
        ),
        (
            json!({"type":"integer","minimum":100,"maximum":599}),
            "super::EventInteger<100,599>",
        ),
        (
            json!({"type":"number","enum":[1]}),
            "super::EventVersion<1>",
        ),
        (
            json!({"type":"number","enum":[2]}),
            "super::EventVersion<2>",
        ),
    ] {
        settings.with_conversion(serde_json::from_value(wire)?, rust, std::iter::empty());
    }
    let mut dispatch = String::new();
    // Typify's flattened tagged unions can lose required fields. Generate explicit
    // dispatch over standalone branch structs, preserving each branch's extensions.
    for (wire, rust) in [
        ("Session.Inbox.Item", "EventInboxItem"),
        ("SessionStatus", "EventStatusPayload"),
        ("Form.Field_1", "EventFormField"),
    ] {
        let branches = schema["definitions"][wire]["anyOf"]
            .as_array()
            .ok_or("event payload union changed")?
            .clone();
        let mut variants = Vec::new();
        for branch in branches {
            let branch = if let Some(reference) = branch["$ref"].as_str() {
                schema["definitions"][reference
                    .strip_prefix("#/definitions/")
                    .ok_or("unexpected ref")?]
                .clone()
            } else {
                branch
            };
            let tag = branch["properties"]["type"]["enum"]
                .as_array()
                .filter(|v| v.len() == 1)
                .and_then(|v| v[0].as_str())
                .ok_or("payload union tag changed")?;
            if !branch["required"]
                .as_array()
                .is_some_and(|v| v.contains(&json!("type")))
            {
                return Err("missing required union tag".into());
            }
            let variant = pascal(tag);
            let name = format!("{rust}{variant}");
            schema["definitions"][&name] = branch.clone();
            variants.push((variant, name, tag.to_owned()));
        }
        // Keep the original reference name, but ask Typify to use our explicit enum.
        schema["definitions"][wire] = json!({});
        settings.with_replacement(pascal(wire), rust, std::iter::empty());
        dispatch.push_str(&enum_dispatch(rust, &variants, false));
    }
    // Nonempty homogeneous arrays are represented as a checked wrapper, not tuples.
    normalize_arrays(&mut schema, &mut settings)?;
    allow_additive_fields(&mut schema);
    let mut syntax = syn::parse_file(&generate_with_settings(schema, &settings)?)?;
    for item in &mut syntax.items {
        match item {
            Item::Struct(item) if matches!(item.fields, syn::Fields::Named(_)) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for field in &mut item.fields {
                    if optional_constraint(&field.ty) {
                        field.attrs.push(parse_quote!(#[serde(default, deserialize_with="super::deserialize_optional_value")]));
                    }
                }
            }
            Item::Enum(item) => {
                item.attrs.push(parse_quote!(#[non_exhaustive]));
                for v in &mut item.variants {
                    if matches!(v.fields, syn::Fields::Named(_)) {
                        v.attrs.push(parse_quote!(#[non_exhaustive]));
                    }
                }
            }
            _ => {}
        }
    }
    let variants = roots
        .iter()
        .map(|r| {
            (
                r.variant.clone(),
                format!("Event{}", r.variant),
                r.r#type.clone(),
            )
        })
        .collect::<Vec<_>>();
    dispatch.push_str(&enum_dispatch("Event", &variants, true));
    dispatch.push_str(&log_accessors(&spec, &roots)?);
    let mut output = prettyplease::unparse(&syntax);
    output.push_str(&prettyplease::unparse(&syn::parse_file(&dispatch)?));
    Ok(format!(
        "// Generated from openapi/opencode-v2.json by model-spike --events.\n// Do not edit manually; adaptations live in tools/model-spike/src/event_models.rs.\n{output}"
    ))
}

pub(crate) fn pascal(s: &str) -> String {
    s.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let s = if s.chars().all(|c| c.is_ascii_uppercase()) {
                s.to_ascii_lowercase()
            } else {
                s.to_owned()
            };
            format!("{}{}", s[..1].to_ascii_uppercase(), &s[1..])
        })
        .collect()
}

// These string branches are disjoint. Typify flattens anyOf into an object,
// whereas oneOf produces a checked scalar enum for the same accepted values.
fn normalize_instruction_delta(schema: &mut Value) -> Result<(), Box<dyn std::error::Error>> {
    let branches = json!([
        {"$ref":"#/definitions/Instruction.Hash"},
        {"type":"string","enum":["removed"]},
    ]);
    if schema["definitions"]["Instruction.Hash"]
        != json!({"type":"string","pattern":"^[a-f0-9]{64}$"})
    {
        return Err("instruction hash changed; review disjoint delta branches".into());
    }
    let delta = schema.pointer_mut("/definitions/EventSessionInstructionsUpdated/properties/data/properties/delta/additionalProperties")
        .ok_or("instruction delta missing")?;
    if *delta != json!({"anyOf":branches}) {
        return Err("instruction delta changed; review scalar union".into());
    }
    *delta = json!({"oneOf":branches});
    Ok(())
}

fn enum_dispatch(name: &str, variants: &[(String, String, String)], unknown: bool) -> String {
    enum_dispatch_tag(name, variants, unknown, "type")
}

pub(crate) fn enum_dispatch_tag(
    name: &str,
    variants: &[(String, String, String)],
    unknown: bool,
    tag_key: &str,
) -> String {
    let mut out = format!(
        "/// Typed wire event or payload; malformed known tags fail decoding.\n#[derive(Clone,Debug,::serde::Serialize)] #[serde(untagged)] #[non_exhaustive] pub enum {name} {{"
    );
    for (variant, ty, tag) in variants {
        out.push_str(&format!("/// Wire kind `{tag}`.\n{variant}(Box<{ty}>),"));
    }
    if unknown {
        out.push_str("/// Unimplemented or future kind, retaining the complete JSON object.\nUnknown(::serde_json::Value),");
    }
    out.push_str(&format!("}} impl<'de> ::serde::Deserialize<'de> for {name} {{fn deserialize<D: ::serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{{let raw=<::serde_json::Value as ::serde::Deserialize>::deserialize(d)?; match raw.get({tag_key:?}).and_then(::serde_json::Value::as_str){{"));
    for (variant, _, tag) in variants {
        let pattern = if tag == "rpc.*" {
            "Some(tag) if tag.starts_with(\"rpc.\")".to_owned()
        } else {
            format!("Some({tag:?})")
        };
        out.push_str(&format!(
            "{pattern}=>::serde_json::from_value(raw).map(Self::{variant}),"
        ));
    }
    if unknown {
        out.push_str("Some(_)=>Ok(Self::Unknown(raw)),");
    }
    out.push_str("_=>return Err(::serde::de::Error::custom(\"unknown or missing event/payload type\")),}.map_err(::serde::de::Error::custom)}}");
    if unknown {
        out.push_str(&format!("impl {name} {{ /// Wire discriminator, independent of the SSE event name.\npub fn kind(&self)->&str{{match self{{"));
        for (variant, _, tag) in variants {
            out.push_str(&if tag == "rpc.*" {
                format!("Self::{variant}(v)=>v.type_.as_str(),")
            } else {
                format!("Self::{variant}(_)=>{tag:?},")
            });
        }
        out.push_str("Self::Unknown(v)=>v.get(\"type\").and_then(::serde_json::Value::as_str).unwrap_or(\"\"),}}}");
    }
    out
}

fn normalize_arrays(
    value: &mut Value,
    settings: &mut typify::TypeSpaceSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        Value::Object(map) => {
            if map.contains_key("prefixItems") {
                let item = map["items"].clone();
                if map["prefixItems"] != json!([item]) || map["minItems"] != 1 {
                    return Err("nonempty event array changed".into());
                }
                map.remove("prefixItems");
                let rust = match item["$ref"].as_str() {
                    Some("#/definitions/Tool.Content") => {
                        "super::NonEmptyEventItems<super::ToolContent>"
                    }
                    Some("#/definitions/Form.Field_1") => {
                        "super::NonEmptyEventItems<EventFormField>"
                    }
                    _ => return Err("unreviewed nonempty event array".into()),
                };
                settings.with_conversion(
                    serde_json::from_value(Value::Object(map.clone()))?,
                    rust,
                    std::iter::empty(),
                );
            }
            for v in map.values_mut() {
                normalize_arrays(v, settings)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                normalize_arrays(v, settings)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn optional_constraint(ty: &syn::Type) -> bool {
    let syn::Type::Path(p) = ty else { return false };
    let Some(segment) = p.path.segments.last() else {
        return false;
    };
    if segment.ident != "Option" {
        return false;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return false;
    };
    let Some(syn::GenericArgument::Type(syn::Type::Path(inner))) = args.args.first() else {
        return false;
    };
    inner.path.segments.iter().any(|s| {
        s.ident == "EventInteger" || s.ident == "NonEmptyEventItems" || s.ident == "EventSeq"
    })
}

// Refuse new numeric/non-null shapes instead of silently losing constraints in
// Typify. The exact-match conversions above implement these reviewed contracts.
fn guard_constraints(value: &Value) -> Result<(), Box<dyn std::error::Error>> {
    match value {
        Value::Object(map) => {
            if map.get("type") == Some(&json!("integer"))
                && ![
                    json!({"type":"integer","minimum":0,"maximum":9007199254740991_u64}),
                    json!({"type":"integer","exclusiveMinimum":0,"maximum":9007199254740991_u64}),
                    json!({"type":"integer","minimum":100,"maximum":599}),
                ]
                .contains(value)
            {
                return Err("unreviewed event integer constraint".into());
            }
            if map.get("type") == Some(&json!("number"))
                && map.contains_key("enum")
                && ![
                    json!({"type":"number","enum":[1]}),
                    json!({"type":"number","enum":[2]}),
                ]
                .contains(value)
            {
                return Err("unreviewed event version constraint".into());
            }
            if map.contains_key("not") && value != &json!({"not":{"type":"null"}}) {
                return Err("unreviewed event non-null constraint".into());
            }
            for v in map.values() {
                guard_constraints(v)?;
            }
        }
        Value::Array(a) => {
            for v in a {
                guard_constraints(v)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn log_accessors(spec: &Value, roots: &[Root]) -> Result<String, Box<dyn std::error::Error>> {
    let reference = |name: &str| json!({"$ref":format!("#/components/schemas/{name}")});
    let schemas = &spec["components"]["schemas"];
    if schemas["SessionLogItem"]
        != json!({"anyOf":[reference("Session.Event.Durable"),reference("EventLog.Synced")]})
        || schemas["EventLog.Synced"]["required"] != json!(["type", "aggregateID"])
        || schemas["EventLog.Synced"]["properties"]
            != json!({"type":{"type":"string","enum":["log.synced"]},"aggregateID":{"type":"string"},"seq":reference("Event.Seq")})
    {
        return Err(
            "session log union/watermark changed; review dispatch and optional sequence".into(),
        );
    }
    if schemas["Session.Message.AssistantContent.Encoded"]
        != json!({"anyOf":[reference("Session.Message.Assistant.Text"),reference("Session.Message.Assistant.Reasoning"),reference("Session.Message.Assistant.Tool")]})
    {
        return Err(
            "encoded assistant-content union changed; review shared model replacement".into(),
        );
    }
    let members = schemas["Session.Event.Durable"]["anyOf"]
        .as_array()
        .ok_or("durable log union missing")?;
    let mut durable = Vec::new();
    for member in members {
        let root = roots
            .iter()
            .find(|r| reference(&r.schema) == *member)
            .ok_or("unreviewed durable log kind")?;
        let schema = &schemas[&root.schema];
        if !schema["required"]
            .as_array()
            .is_some_and(|r| r.contains(&json!("durable")))
            || schema["properties"]["durable"]["required"]
                != json!(["aggregateID", "seq", "version"])
            || schema["properties"]["durable"]["properties"]["seq"] != reference("Event.Seq")
            || schema["properties"]["durable"]["properties"]["aggregateID"]
                != json!({"type":"string"})
        {
            return Err("durable log envelope changed; review accessors".into());
        }
        durable.push(&root.variant);
    }
    let mut out = String::from(
        "impl Event { /// Aggregate sequence for reviewed durable session events; no cursor is inferred for unknown kinds.\npub fn durable_sequence(&self)->Option<u64>{match self{",
    );
    for variant in &durable {
        out.push_str(&format!(
            "Self::{variant}(event)=>Some(event.durable.seq.0.get()),"
        ));
    }
    out.push_str("_=>None,}} /// Aggregate ID for reviewed durable session events.\npub fn durable_aggregate_id(&self)->Option<&str>{match self{");
    for variant in &durable {
        out.push_str(&format!(
            "Self::{variant}(event)=>Some(&event.durable.aggregate_id),"
        ));
    }
    out.push_str("_=>None,}}}");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn spec() -> Value {
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap()
    }
    #[test]
    fn instruction_delta_guard_rejects_overlapping_or_changed_scalar_contracts() {
        for (pointer, value) in [
            ("/components/schemas/Instruction.Hash/pattern", json!(".*")),
            (
                "/components/schemas/session.instructions.updated/properties/data/properties/delta/additionalProperties/anyOf/1/enum/0",
                json!("deleted"),
            ),
            (
                "/components/schemas/session.instructions.updated/properties/data/properties/delta/additionalProperties",
                json!({"type":"string"}),
            ),
        ] {
            let mut spec = spec();
            *spec.pointer_mut(pointer).unwrap() = value;
            assert!(generate_events(&spec).is_err(), "{pointer}");
        }
    }
    #[test]
    fn log_generation_guards_watermark_and_durable_envelope_adapters() {
        for (pointer, value) in [
            (
                "/components/schemas/EventLog.Synced/required",
                json!(["type", "aggregateID", "seq"]),
            ),
            (
                "/components/schemas/EventLog.Synced/properties/seq",
                json!({"type":"number"}),
            ),
            ("/components/schemas/SessionLogItem/anyOf", json!([])),
            (
                "/components/schemas/session.renamed/properties/durable/required",
                json!(["aggregateID", "version"]),
            ),
            (
                "/components/schemas/Session.Message.AssistantContent.Encoded/anyOf",
                json!([]),
            ),
        ] {
            let mut spec = spec();
            *spec.pointer_mut(pointer).unwrap() = value;
            assert!(generate_events(&spec).is_err(), "{pointer}");
        }
    }
    #[test]
    fn event_generation_guards_tags_numeric_constraints_and_nonempty_arrays() {
        for (pointer, value) in [
            (
                "/components/schemas/session.text.delta/properties/type/enum/0",
                json!("new.tag"),
            ),
            (
                "/components/schemas/session.text.delta/properties/data/properties/ordinal/maximum",
                json!(10),
            ),
            (
                "/components/schemas/session.tool.success/properties/data/properties/content/minItems",
                json!(0),
            ),
            (
                "/components/schemas/V2Event.rpc/properties/type/pattern",
                json!("^other"),
            ),
            (
                "/components/schemas/SessionStatus/anyOf/0/required",
                json!([]),
            ),
        ] {
            let mut spec = spec();
            *spec.pointer_mut(pointer).unwrap() = value;
            assert!(generate_events(&spec).is_err(), "{pointer}");
        }
    }
}
