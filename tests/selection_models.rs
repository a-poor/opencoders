use opencoders::models::{
    ModelDefaultResponse, ModelInfo, ModelType, SessionModelInput, SessionSwitchAgentRequest,
    SessionSwitchModelRequest,
};
use serde_json::{Value, json};

fn model() -> Value {
    serde_json::from_str(include_str!("fixtures/model.json")).unwrap()
}

#[test]
fn catalog_round_trips_extensions_open_strings_and_shared_types() {
    let mut raw = model();
    raw["type"] = json!("vendor-type");
    raw["time"]["released"] = json!(9_007_199_254_740_993_u64);
    let parsed: ModelInfo = serde_json::from_value(raw.clone()).unwrap();
    let _: &opencoders::models::ProviderId = &parsed.provider_id;
    assert_eq!(parsed.type_.as_ref().unwrap().as_str(), "vendor-type");
    assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    for kind in ["chat", "image", "decision", "vendor-type", ""] {
        let parsed: ModelType = kind.parse().unwrap();
        assert_eq!(parsed.as_str(), kind);
    }
}

#[test]
fn model_limits_follow_signed_safe_integer_bounds_and_optionality() {
    for pointer in [
        "/limit/context",
        "/limit/input",
        "/limit/output",
        "/cost/0/tier/size",
    ] {
        for valid in [-9_007_199_254_740_991_i64, 0, 9_007_199_254_740_991] {
            let mut raw = model();
            *raw.pointer_mut(pointer).unwrap() = json!(valid);
            let parsed: ModelInfo = serde_json::from_value(raw.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
        }
        for invalid in [
            json!(-9_007_199_254_740_992_i64),
            json!(9_007_199_254_740_992_i64),
            json!(1.5),
            json!(null),
        ] {
            let mut raw = model();
            *raw.pointer_mut(pointer).unwrap() = invalid;
            assert!(
                serde_json::from_value::<ModelInfo>(raw).is_err(),
                "{pointer}"
            );
        }
    }
    let mut raw = model();
    raw["limit"].as_object_mut().unwrap().remove("input");
    assert!(serde_json::from_value::<ModelInfo>(raw).is_ok());
}

#[test]
fn default_model_is_nullable_but_the_data_field_is_required() {
    for data in [json!(null), model()] {
        let raw = json!({"location":{"directory":"/remote","future":1},"data":data,"future":2});
        let parsed: ModelDefaultResponse = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(parsed.data.is_none(), data.is_null());
        assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    }
    assert!(
        serde_json::from_value::<ModelDefaultResponse>(json!({"location":{"directory":"/remote"}}))
            .is_err()
    );
}

#[test]
fn catalog_debug_redacts_provider_configuration_without_changing_data() {
    let parsed: ModelInfo = serde_json::from_value(model()).unwrap();
    assert!(!format!("{parsed:?} {:?}", parsed.variants[0]).contains("private-"));
    assert_eq!(parsed.headers["authorization"], "private-header");
    assert_eq!(serde_json::to_value(parsed).unwrap(), model());
}

#[test]
fn selection_inputs_are_required_nonnullable_and_do_not_echo_response_fields() {
    let agent = SessionSwitchAgentRequest::new("build".to_owned().into());
    let request = SessionSwitchModelRequest::new(
        SessionModelInput::new("provider".to_owned().into(), "public-name").variant(""),
    );
    assert_eq!(
        serde_json::to_value(agent).unwrap(),
        json!({"agent":"build"})
    );
    assert_eq!(
        serde_json::to_value(request).unwrap(),
        json!({"model":{"id":"public-name","providerID":"provider","variant":""}})
    );
    for raw in [
        json!({}),
        json!({"agent":null}),
        json!({"agent":"build","extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionSwitchAgentRequest>(raw).is_err());
    }
    for raw in [
        json!({}),
        json!({"model":null}),
        json!({"model":{"id":"x","providerID":"p","variant":null}}),
        json!({"model":{"id":"x","providerID":"p","extra":1}}),
    ] {
        assert!(serde_json::from_value::<SessionSwitchModelRequest>(raw).is_err());
    }
}
