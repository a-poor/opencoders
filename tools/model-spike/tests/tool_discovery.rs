use opencoders_model_spike::tool_discovery::generate_tool_discovery;
use serde_json::{Value, json};
#[test]
fn discovery_generation_requires_review_of_envelopes_variants_forms_and_literals() {
    for (pointer, value) in [
        (
            "/paths/~1api~1integration/get/responses/200/content/application~1json/schema/required",
            json!(["data"]),
        ),
        ("/components/schemas/Integration.Method/anyOf", json!([])),
        (
            "/components/schemas/Mcp.Status.Failed/required",
            json!(["error"]),
        ),
        (
            "/components/schemas/Plugin.Features/properties/server",
            json!({"type":"boolean"}),
        ),
        (
            "/components/schemas/Plugin.Source/anyOf/1/properties/outdated",
            json!({"type":"boolean"}),
        ),
        ("/components/schemas/Form.Fields/minItems", json!(0)),
        (
            "/components/schemas/IntegrationNotFoundErrorEncoded/required",
            json!(["_tag", "message"]),
        ),
    ] {
        let mut spec: Value =
            serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
        *spec.pointer_mut(pointer).unwrap() = value;
        assert!(generate_tool_discovery(&spec).is_err(), "{pointer}");
    }
}
