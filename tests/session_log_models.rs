use opencoders::models::SessionLogItem;
use serde_json::{Value, json};

#[test]
fn watermark_preserves_optional_sequence_and_extensions_without_inventing_zero() {
    for seq in [None, Some(0), Some(9007199254740991_u64)] {
        let mut raw = json!({"type":"log.synced","aggregateID":"ses_log","future":{"ok":true}});
        if let Some(seq) = seq {
            raw["seq"] = json!(seq);
        }
        let item: SessionLogItem = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(item.sequence(), seq);
        assert_eq!(item.aggregate_id(), Some("ses_log"));
        assert_eq!(serde_json::to_value(item).unwrap(), raw);
    }
    for seq in [
        Value::Null,
        json!(-1),
        json!(0.5),
        json!(9007199254740992_u64),
    ] {
        assert!(
            serde_json::from_value::<SessionLogItem>(
                json!({"type":"log.synced","aggregateID":"ses_log","seq":seq})
            )
            .is_err()
        );
    }
    for raw in [json!({"type":"log.synced"}), json!({}), json!({"type":1})] {
        assert!(serde_json::from_value::<SessionLogItem>(raw).is_err());
    }
}

#[test]
fn log_dispatch_reuses_durable_events_and_rejects_known_transient_events() {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/events.json")).unwrap();
    let mut count = 0;
    for raw in fixtures {
        let item = serde_json::from_value::<SessionLogItem>(raw.clone());
        if raw.get("durable").is_some() {
            count += 1;
            let item = item.unwrap();
            assert!(matches!(&item, SessionLogItem::Event(_)));
            assert_eq!(item.sequence(), raw["durable"]["seq"].as_u64());
            assert_eq!(serde_json::to_value(item).unwrap(), raw);
        } else {
            assert!(item.is_err(), "{}", raw["type"]);
        }
    }
    assert_eq!(count, 47);
    let raw = json!({"type":"future.durable","data":{"future":true}});
    let item: SessionLogItem = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(&item, SessionLogItem::Unknown(_)));
    assert_eq!(serde_json::to_value(item).unwrap(), raw);
    assert!(serde_json::from_value::<SessionLogItem>(json!({"type":"session.renamed"})).is_err());
}

#[test]
fn newly_reviewed_durable_payloads_keep_typed_content_and_numeric_constraints() {
    let fixtures: Vec<Value> = serde_json::from_str(include_str!("fixtures/events.json")).unwrap();
    let find = |kind: &str| fixtures.iter().find(|v| v["type"] == kind).unwrap().clone();
    let mut edited = find("session.message.content.updated");
    edited["data"]["content"] = json!([{"type":"text","text":"complete","future":1}]);
    let item: SessionLogItem = serde_json::from_value(edited.clone()).unwrap();
    assert_eq!(serde_json::to_value(item).unwrap(), edited);
    edited["data"]["content"][0] = json!({"type":"text"});
    assert!(serde_json::from_value::<SessionLogItem>(edited).is_err());
    for bad in [Value::Null, json!(-1), json!(9007199254740992_u64)] {
        let mut shell = find("session.shell.started");
        shell["data"]["shell"]["pid"] = bad;
        assert!(serde_json::from_value::<SessionLogItem>(shell).is_err());
    }
    let mut shell = find("session.shell.ended");
    shell["data"]["output"]["cursor"] = json!(0.5);
    assert!(serde_json::from_value::<SessionLogItem>(shell).is_err());
}
