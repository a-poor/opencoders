use opencoders_model_spike::shells::generate_shells;
use serde_json::{Value, json};

#[test]
fn shell_generation_requires_review_when_inputs_shared_output_or_query_constraints_change() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for (pointer, value) in [
        (
            "/paths/~1api~1shell/post/requestBody/content/application~1json/schema/properties/timeout/maximum",
            json!(100),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1shell/post/requestBody/content/application~1json/schema/properties/id",
            json!({"type":"string"}),
        ),
        (
            "/paths/~1api~1session~1{sessionID}~1environment/put/requestBody/content/application~1json/schema/required",
            json!([]),
        ),
        (
            "/paths/~1api~1shell~1{id}~1output/get/responses/200/content/application~1json/schema/properties/data/properties/cursor/maximum",
            json!(100),
        ),
        (
            "/paths/~1api~1shell~1{id}~1output/get/parameters/2/schema/pattern",
            json!("^x"),
        ),
        (
            "/components/schemas/Shell.Info/properties/pid/maximum",
            json!(100),
        ),
    ] {
        let mut changed = spec.clone();
        *changed.pointer_mut(pointer).unwrap() = value;
        assert!(generate_shells(&changed).is_err(), "{pointer}");
    }
}
