use opencoders_model_spike::messages::generate_messages;
use serde_json::{Value, json};

fn spec() -> Value {
    serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap()
}

#[test]
fn message_generation_is_reproducible_and_uses_shared_ids() {
    let output = generate_messages(&spec()).unwrap();
    assert_eq!(output, include_str!("../../../src/models/messages.rs"));
    assert!(output.contains("pub id: super::SessionMessageId"));
    assert!(!output.contains("pub struct SessionMessageInfo"));
}

#[test]
fn changed_message_tags_or_union_members_require_review() {
    for pointer in [
        "/components/schemas/Session.Message.User/properties/type/enum/0",
        "/components/schemas/Session.Message.Compaction.Completed/properties/status/enum/0",
        "/components/schemas/Session.Message.Assistant.Text/properties/type/enum/0",
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = json!("future");
        assert!(generate_messages(&changed).is_err());
    }
    let mut changed = spec();
    changed["components"]["schemas"]["Session.Message.Info"]["anyOf"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(generate_messages(&changed).is_err());
}

#[test]
fn changed_message_numeric_or_content_constraints_require_review() {
    for pointer in [
        "/components/schemas/Session.Message.Shell/properties/output/properties/size/maximum",
        "/components/schemas/Session.StructuredError/properties/status/minimum",
        "/components/schemas/Session.StructuredError/properties/retryAfterMs/maximum",
        "/components/schemas/Session.Message.Assistant.Retry/properties/attempt/exclusiveMinimum",
        "/components/schemas/Session.Message.ToolState.Completed/properties/content/minItems",
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = json!(10);
        assert!(generate_messages(&changed).is_err());
    }
    let mut changed = spec();
    changed["components"]["schemas"]["Session.StructuredError"]["required"]
        .as_array_mut()
        .unwrap()
        .push(json!("status"));
    assert!(
        generate_messages(&changed).is_err(),
        "a required status must not acquire a default"
    );
}
