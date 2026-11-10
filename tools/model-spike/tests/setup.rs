use opencoders_model_spike::setup::generate_setup;
use serde_json::{Value, json};

#[test]
fn setup_generation_requires_review_of_envelopes_bounds_and_timeout_unions() {
    for (pointer, value) in [
        ("/components/schemas/Config.Entry/anyOf", json!([])),
        (
            "/paths/~1api~1config/get/responses/200/content/application~1json/schema",
            json!({"type":"object"}),
        ),
        (
            "/paths/~1api~1provider/get/responses/200/content/application~1json/schema/required",
            json!(["data"]),
        ),
        (
            "/components/schemas/Project.Time/properties/active/maximum",
            json!(100),
        ),
        (
            "/components/schemas/Config.Provider.Settings/properties/timeout",
            json!({"type":"boolean"}),
        ),
        (
            "/components/schemas/Config.Provider.Settings/allOf",
            json!([{"type":"object","additionalProperties":{"type":"string"}}]),
        ),
        (
            "/components/schemas/Config.WarmingEncoded/properties/interval",
            json!({"type":"integer"}),
        ),
    ] {
        let mut spec: Value =
            serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
        *spec.pointer_mut(pointer).unwrap() = value;
        assert!(generate_setup(&spec).is_err(), "{pointer}");
    }
}
