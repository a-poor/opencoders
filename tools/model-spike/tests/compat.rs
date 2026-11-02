use opencoders_model_spike::{
    compat::{NonNullValue, SessionTitlePatch},
    events::{Event, decode_event_data},
};
use serde_json::{Value, json};

#[test]
fn non_null_dynamic_payload_rejects_null() {
    assert!(serde_json::from_value::<NonNullValue>(Value::Null).is_err());
}

#[test]
fn non_null_dynamic_payload_preserves_all_other_json_kinds() {
    for value in [json!(false), json!(0), json!(""), json!([]), json!({})] {
        let payload: NonNullValue = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(payload).unwrap(), value);
    }
}

#[test]
fn update_adapter_preserves_omitted_null_and_value() {
    for value in [
        json!({}),
        json!({"title": null}),
        json!({"title": "New title"}),
    ] {
        let patch: SessionTitlePatch = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(patch).unwrap(), value);
    }
}

#[test]
fn event_data_is_decoded_once_and_dispatched_by_type() {
    let event =
        decode_event_data(r#"{"id":"evt_test","type":"server.connected","data":{}}"#).unwrap();
    assert!(matches!(event, Event::Connected(_)));
    assert!(decode_event_data("not JSON").is_err());
    assert!(decode_event_data(r#""{\"type\":\"future.event\"}""#).is_err());
}

#[test]
fn unknown_event_is_preserved_without_masking_malformed_known_events() {
    let value = json!({"type": "future.event", "data": {"new": [1, 2, 3]}});
    let Event::Unknown(raw) = decode_event_data(&value.to_string()).unwrap() else {
        panic!("expected unknown event")
    };
    assert_eq!(raw, value);
    assert!(decode_event_data(r#"{"type":"session.text.delta","data":{}}"#).is_err());
    for invalid in [json!({}), json!({"type": 3}), json!(null), json!([])] {
        assert!(decode_event_data(&invalid.to_string()).is_err());
    }
}
