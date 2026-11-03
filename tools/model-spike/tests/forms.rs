use opencoders_model_spike::{event_models::generate_events, forms::generate_forms};
use serde_json::{Value, json};
fn spec() -> Value {
    serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap()
}
#[test]
fn form_reuse_rejects_http_event_contract_drift() {
    for name in [
        "Info",
        "Fields",
        "Field",
        "StringField",
        "NumberField",
        "IntegerField",
        "BooleanField",
        "MultiselectField",
        "When",
        "Value",
        "Answer",
    ] {
        let mut s = spec();
        s["components"]["schemas"][format!("Form.{name}")] = json!({"type":"null"});
        assert!(generate_forms(&s).is_err(), "Form.{name}");
    }
}
#[test]
fn form_adapters_reject_scalar_state_optional_and_constraint_drift() {
    for (name, path, value) in [
        ("Form.CreatePayload", "/required", json!(["title"])),
        (
            "Form.CreatePayload",
            "/properties/id",
            json!({"type":"string"}),
        ),
        ("Form.Reply", "/required", json!([])),
        ("Form.State", "/anyOf/1/required", json!(["status"])),
        (
            "Form.StringField",
            "/properties/minLength/maximum",
            json!(10),
        ),
        ("Form.Value", "/anyOf/2/enum", json!(["new-number"])),
        ("Form.Fields", "/minItems", json!(0)),
    ] {
        let mut s = spec();
        *s["components"]["schemas"][name].pointer_mut(path).unwrap() = value.clone();
        if s["components"]["schemas"]
            .get(format!("{name}_1"))
            .is_some()
        {
            *s["components"]["schemas"][format!("{name}_1")]
                .pointer_mut(path)
                .unwrap() = value;
        }
        assert!(generate_forms(&s).is_err(), "{name} {path}");
    }
    let mut s = spec();
    s["components"]["schemas"]["Form.When_1"]["properties"]["value"]["anyOf"]
        .as_array_mut()
        .unwrap()
        .push(json!({"type":"null"}));
    assert!(
        generate_events(&s)
            .unwrap_err()
            .to_string()
            .contains("form scalar adapter")
    );
}
