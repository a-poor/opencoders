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

#[test]
fn model_catalog_adaptations_reject_changed_bounds_optionality_and_open_strings() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for pointer in [
        "/components/schemas/Model.Info/properties/limit/properties/input/maximum",
        "/components/schemas/Model.Cost/properties/tier/properties/size/minimum",
        "/components/schemas/Model.Type/anyOf/1/type",
        "/components/schemas/Model.ReasoningField/anyOf/1/type",
    ] {
        let mut changed = spec.clone();
        *changed.pointer_mut(pointer).unwrap() = json!("changed");
        assert!(generate_discovery(&changed).is_err());
    }
    let mut changed = spec.clone();
    changed["components"]["schemas"]["Model.Info"]["properties"]["limit"]["required"]
        .as_array_mut()
        .unwrap()
        .push(json!("input"));
    assert!(generate_discovery(&changed).is_err());
    let mut changed = spec;
    *changed.pointer_mut("/paths/~1api~1model~1default/get/responses/200/content/application~1json/schema/required").unwrap()=json!(["location"]);
    assert!(generate_discovery(&changed).is_err());
}
