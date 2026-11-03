use opencoders::models::Event;
use serde_json::{Value, json};

#[test]
fn event_envelopes_retain_ids_location_durable_metadata_and_extensions() {
    let raw = json!({"id":"evt_test","created":9007199254740993_u64,"type":"session.renamed","metadata":{"x":1},"location":{"directory":"/remote","future":true},"durable":{"aggregateID":"ses_test","seq":9007199254740991_u64,"version":1,"future":2},"data":{"sessionID":"ses_test","title":"renamed","future":3},"future":4});
    let event: Event = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(event.kind(), "session.renamed");
    assert!(matches!(&event, Event::SessionRenamed(_)));
    assert_eq!(serde_json::to_value(event).unwrap(), raw);
}

#[test]
fn event_numeric_constraints_and_known_payloads_are_validated() {
    let raw = json!({"id":"evt_test","created":1,"type":"session.text.delta","data":{"sessionID":"ses_test","assistantMessageID":"msg_test","ordinal":0,"delta":"雪"}});
    assert!(serde_json::from_value::<Event>(raw.clone()).is_ok());
    for value in [
        json!(-1),
        json!(9007199254740992_u64),
        json!(0.5),
        Value::Null,
    ] {
        let mut invalid = raw.clone();
        invalid["data"]["ordinal"] = value;
        assert!(serde_json::from_value::<Event>(invalid).is_err());
    }
    for field in ["id", "created", "data"] {
        let mut invalid = raw.clone();
        invalid.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<Event>(invalid).is_err());
    }
    for raw in [
        json!({}),
        json!({"type":1}),
        json!({"type":"server.connected","id":"evt_test","data":null}),
    ] {
        assert!(serde_json::from_value::<Event>(raw).is_err());
    }
}

#[test]
fn rpc_events_have_a_reviewed_shape_and_unimplemented_kinds_keep_json() {
    let raw = json!({"id":"evt_rpc","created":1,"type":"rpc.plugin.changed","location":{"directory":"/remote"},"data":{"nested":{"a":1}},"extra":true});
    let event: Event = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(&event, Event::Rpc(_)));
    assert_eq!(event.kind(), "rpc.plugin.changed");
    assert_eq!(serde_json::to_value(event).unwrap(), raw);
    assert!(serde_json::from_value::<Event>(json!({"type":"rpc.plugin.changed"})).is_err());
    let raw = json!({"type":"plugin.updated","future":true});
    let event: Event = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(&event, Event::Unknown(_)));
    assert_eq!(serde_json::to_value(event).unwrap(), raw);
}

#[test]
fn every_reviewed_event_fixture_decodes_and_roundtrips() {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/events.json")).unwrap();
    assert_eq!(fixtures.len(), 52);
    for mut raw in fixtures {
        raw["future"] = json!({"extension":true});
        let event: Event =
            serde_json::from_value(raw.clone()).unwrap_or_else(|e| panic!("{}: {e}", raw["type"]));
        assert!(!matches!(&event, Event::Unknown(_)), "{}", raw["type"]);
        assert_eq!(event.kind(), raw["type"].as_str().unwrap());
        assert_eq!(serde_json::to_value(event).unwrap(), raw);
    }
}

#[test]
fn durable_versions_nonempty_content_and_payload_unions_are_strict() {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/events.json")).unwrap();
    let find = |kind: &str| fixtures.iter().find(|v| v["type"] == kind).unwrap().clone();
    for bad in [
        json!(-1),
        json!(9007199254740992_u64),
        json!(0.5),
        Value::Null,
    ] {
        let mut v = find("session.renamed");
        v["durable"]["seq"] = bad;
        assert!(serde_json::from_value::<Event>(v).is_err());
    }
    for bad in [json!(0), json!(2), Value::Null] {
        let mut v = find("session.renamed");
        v["durable"]["version"] = bad;
        assert!(serde_json::from_value::<Event>(v).is_err());
    }
    for kind in ["session.tool.success", "form.created"] {
        let mut v = find(kind);
        let array = if kind == "form.created" {
            &mut v["data"]["form"]["fields"]
        } else {
            &mut v["data"]["content"]
        };
        *array = json!([]);
        assert!(serde_json::from_value::<Event>(v).is_err(), "{kind}");
    }
    for (kind, pointer) in [
        ("session.inbox.enqueued", "/data/item"),
        ("session.status", "/data/status"),
    ] {
        let mut v = find(kind);
        *v.pointer_mut(pointer).unwrap() = json!({"type":"future"});
        assert!(serde_json::from_value::<Event>(v).is_err(), "{kind}");
    }
    let mut v = find("session.status");
    v["data"]["status"] = json!({"type":"retry"});
    assert!(serde_json::from_value::<Event>(v).is_err());
    let mut v = find("session.tool.failed");
    v["data"]["content"] = Value::Null;
    assert!(serde_json::from_value::<Event>(v).is_err());
}

#[test]
fn nested_inbox_status_and_form_tags_preserve_extensions_and_required_fields() {
    use opencoders::models::events::{EventFormField, EventInboxItem, EventStatusPayload};
    for raw in [
        json!({"type":"user","payload":{"text":"hello","future":1},"delivery":"steer","extra":2}),
        json!({"type":"synthetic","payload":{"text":"hello"},"delivery":"queue","extra":2}),
        json!({"type":"compaction","payload":{"opaque":true},"delivery":"queue","extra":2}),
        json!({"type":"move","payload":{"location":{"directory":"/remote","workspaceID":"wrk_fixture"},"projectID":"project"},"delivery":"steer","extra":2}),
    ] {
        let v: EventInboxItem = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
        let mut invalid = raw;
        invalid.as_object_mut().unwrap().remove("payload");
        assert!(serde_json::from_value::<EventInboxItem>(invalid).is_err());
    }
    for raw in [
        json!({"type":"idle","extra":1}),
        json!({"type":"busy","extra":2}),
        json!({"type":"retry","attempt":0,"next":9007199254740991_u64,"message":"wait","extra":3}),
    ] {
        let v: EventStatusPayload = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
    }
    for kind in [
        "string",
        "number",
        "integer",
        "boolean",
        "multiselect",
        "external",
    ] {
        let mut raw = json!({"type":kind,"key":"answer","future":true});
        if kind == "external" {
            raw["url"] = json!("https://example.invalid");
        }
        if kind == "multiselect" {
            raw["options"] = json!([]);
        }
        let v: EventFormField = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
        raw.as_object_mut().unwrap().remove("key");
        assert!(serde_json::from_value::<EventFormField>(raw).is_err());
    }
    assert!(
        serde_json::from_value::<EventInboxItem>(
            json!({"type":"compaction","payload":null,"delivery":"queue"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<EventFormField>(
            json!({"type":"multiselect","key":"a","options":[],"minItems":null})
        )
        .is_err()
    );
}
