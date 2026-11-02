use opencoders_model_spike::discovery::generate_discovery;
use serde_json::{Value, json};

#[test]
fn discovery_adapters_require_review_when_upstream_constraints_change() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for (pointer, replacement, message) in [
        (
            "/components/schemas/Agent.Info/properties/steps/maximum",
            json!(100),
            "steps",
        ),
        (
            "/components/schemas/Provider.Settings/properties/timeout",
            json!({"type": "boolean"}),
            "timeout",
        ),
        (
            "/components/schemas/Provider.CacheWarming/anyOf/1/properties/interval",
            json!({"type": "integer"}),
            "interval",
        ),
    ] {
        let mut changed = spec.clone();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            generate_discovery(&changed)
                .unwrap_err()
                .to_string()
                .contains(message)
        );
    }
}
