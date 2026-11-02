use opencoders_model_spike::inbox::generate_inbox;
use serde_json::{Value, json};

fn spec() -> Value {
    serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap()
}

#[test]
fn inbox_generation_is_reproducible_and_reuses_shared_models() {
    let output = generate_inbox(&spec()).unwrap();
    assert_eq!(output, include_str!("../../../src/models/inbox.rs"));
    assert!(output.contains("pub id: super::SessionMessageId"));
    assert!(output.contains("Vec<super::PromptFileAttachment>"));
    assert!(!output.contains("pub struct SessionInboxInfo"));
}

#[test]
fn inbox_dispatch_and_nonnull_payload_guards_require_review() {
    for (pointer, value) in [
        ("/components/schemas/Session.Inbox.Info/anyOf", json!([])),
        (
            "/components/schemas/Session.Inbox.User/properties/type/enum",
            json!(["future"]),
        ),
        ("/components/schemas/Session.Inbox.Move/required", json!([])),
        (
            "/components/schemas/Session.Inbox.CompactionPayload",
            json!({}),
        ),
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = value;
        assert!(generate_inbox(&changed).is_err());
    }
}

#[test]
fn input_optionality_and_file_source_shapes_require_review() {
    for (pointer, value) in [
        (
            "/paths/~1api~1session~1{sessionID}~1prompt/post/requestBody/content/application~1json/schema/required",
            json!(["text", "resume"]),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1prompt/post/requestBody/content/application~1json/schema/properties/files",
            json!({"anyOf":[{"type":"array"},{"type":"null"}]}),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1inbox~1{inboxID}/patch/requestBody/content/application~1json/schema/required",
            json!([]),
        ),
        (
            "/components/schemas/PromptInput.FileAttachment/properties/name",
            json!({"type":"number"}),
        ),
        (
            "/components/schemas/Prompt.FileSource/anyOf/0/additionalProperties",
            json!(true),
        ),
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = value;
        assert!(generate_inbox(&changed).is_err());
    }
}
