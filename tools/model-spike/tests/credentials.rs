use opencoders_model_spike::credentials::generate_credentials;
use serde_json::{Value, json};

#[test]
fn credential_generation_requires_review_of_scope_input_unions_and_expiry() {
    for (pointer, value) in [
        (
            "/paths/~1api~1credential/get/parameters",
            json!([{"name":"location"}]),
        ),
        (
            "/paths/~1api~1credential/post/responses/200/content/application~1json/schema/required",
            json!([]),
        ),
        (
            "/paths/~1api~1credential~1{credentialID}/patch/requestBody/content/application~1json/schema/required",
            json!([]),
        ),
        (
            "/paths/~1api~1integration~1{integrationID}~1connect~1key/post/requestBody/content/application~1json/schema/properties/key",
            json!({"type":"number"}),
        ),
        (
            "/components/schemas/Credential.CreateInput/properties/activate",
            json!({"type":"boolean","default":true}),
        ),
        (
            "/components/schemas/Credential.OAuth/properties/expires/maximum",
            json!(10),
        ),
        ("/components/schemas/Credential.Value/anyOf", json!([])),
        (
            "/components/schemas/Credential.Key/properties/metadata",
            json!({"type":"string"}),
        ),
        (
            "/components/schemas/Form.Answer/additionalProperties",
            json!({"type":"string"}),
        ),
    ] {
        let mut spec: Value =
            serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
        *spec.pointer_mut(pointer).unwrap() = value;
        assert!(generate_credentials(&spec).is_err(), "{pointer}");
    }
}
