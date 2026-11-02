use opencoders_model_spike::generated::*;
use serde_json::{Value, json};

fn delta() -> Value {
    json!({
        "id": "evt_probe", "created": 1.0, "type": "session.text.delta",
        "data": {"sessionID": "ses_probe", "assistantMessageID": "msg_probe",
                 "ordinal": 0, "delta": "hello"}
    })
}

#[test]
fn message_variants_keep_their_type_and_content() {
    for kind in ["user", "system"] {
        let value = json!({"id": "msg_probe", "type": kind, "text": "hello",
                           "time": {"created": 1.0}});
        let message: MessageProbe = serde_json::from_value(value.clone()).unwrap();
        match (&message, kind) {
            (MessageProbe::User(user), "user") => assert_eq!(user.text, "hello"),
            (MessageProbe::System(system), "system") => assert_eq!(system.text, "hello"),
            _ => panic!("wrong variant"),
        }
        assert_eq!(serde_json::to_value(message).unwrap(), value);
    }
}

#[test]
fn errors_decode_by_their_wire_tag() {
    let error: ErrorProbe = serde_json::from_value(json!({
        "_tag": "SessionNotFoundError", "sessionID": "ses_missing", "message": "missing"
    }))
    .unwrap();
    assert!(matches!(error, ErrorProbe::SessionNotFoundErrorEncoded(_)));
    assert!(
        serde_json::from_value::<ErrorProbe>(json!({
            "_tag": "SomeFutureError", "message": "unknown"
        }))
        .is_err()
    );
}

#[test]
fn text_delta_decodes_to_a_typed_event() {
    let EventProbe::SessionTextDelta(event) = serde_json::from_value(delta()).unwrap() else {
        panic!("wrong event variant")
    };
    assert_eq!(event.data.delta, "hello");
    assert_eq!(event.data.session_id.to_string(), "ses_probe");
    assert_eq!(event.data.ordinal, 0);
}

#[test]
fn malformed_known_event_is_rejected() {
    let mut value = delta();
    value["data"].as_object_mut().unwrap().remove("delta");
    assert!(serde_json::from_value::<EventProbe>(value).is_err());
    let mut value = delta();
    value["id"] = json!("invalid-prefix");
    assert!(serde_json::from_value::<EventProbe>(value).is_err());
}

#[test]
fn connection_marker_requires_non_null_data() {
    let value = json!({"id": "evt_connected", "type": "server.connected", "data": {}});
    assert!(serde_json::from_value::<EventProbe>(value).is_ok());
    let value = json!({"id": "evt_connected", "type": "server.connected", "data": null});
    assert!(serde_json::from_value::<EventProbe>(value).is_err());
}

#[test]
fn recursive_model_uses_indirection_and_round_trips() {
    let value = json!({"value": "parent", "next": {"value": "child"}});
    let tree: RecursiveProbe = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(tree.next.as_ref().unwrap().value, "child");
    assert_eq!(serde_json::to_value(tree).unwrap(), value);
}

#[test]
fn log_watermark_sequence_is_optional_and_preserved_when_present() {
    for value in [
        json!({"type": "log.synced", "aggregateID": "ses_probe"}),
        json!({"type": "log.synced", "aggregateID": "ses_probe", "seq": 12}),
    ] {
        let marker: EventLogSynced = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(marker).unwrap(), value);
    }
}

// Characterization tests: these gaps require adapters before production use.
#[test]
fn raw_generation_collapses_omitted_and_null_update_fields() {
    for value in [json!({}), json!({"title": null})] {
        let request: SessionUpdateInput = serde_json::from_value(value).unwrap();
        assert_eq!(serde_json::to_value(request).unwrap(), json!({}));
    }
}

#[test]
fn raw_response_models_reject_additive_fields() {
    let mut value = delta();
    value["newField"] = json!(true);
    assert!(serde_json::from_value::<EventProbe>(value).is_err());
}

#[test]
fn raw_sse_content_schema_is_only_a_string() {
    let encoded: SseDataProbe = serde_json::from_value(json!("not JSON")).unwrap();
    assert_eq!(encoded.to_string(), "not JSON");
}

#[test]
fn raw_numbers_do_not_enforce_schema_bounds() {
    // Event.Seq has minimum 0; generated deserialization currently accepts -1.
    assert!(serde_json::from_value::<EventSeq>(json!(-1)).is_ok());
}
