use opencoders_model_spike::workflows::generate_workflows;
use serde_json::{Value, json};
#[test]
fn workflow_generation_requires_review_of_nullable_inputs_shared_envelopes_and_attachments() {
    for (pointer, value) in [
        (
            "/paths/~1api~1session~1{sessionID}~1command/post/requestBody/content/application~1json/schema/properties/delivery",
            json!({"type":"string"}),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1revert~1stage/post/requestBody/content/application~1json/schema/required",
            json!(["messageID", "files"]),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1context/get/responses/200/content/application~1json/schema/properties/data",
            json!({"type":"array","items":{"type":"string"}}),
        ),
        (
            "/components/schemas/PromptInput.FileAttachment/additionalProperties",
            json!(true),
        ),
    ] {
        let mut spec: Value =
            serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
        *spec.pointer_mut(pointer).unwrap() = value;
        assert!(generate_workflows(&spec).is_err(), "{pointer}");
    }
}
