use opencoders::models::{SessionMessageInfo, SessionMessagesResponse};
use serde_json::{Value, json};

fn message(kind: &str) -> Value {
    let extra = match kind {
        "agent-switched" => json!({"agent": "build"}),
        "model-switched" => json!({"model": {"id": "model", "providerID": "provider"}}),
        "location-switched" => {
            json!({"projectID": "project", "subpath": "", "location": {"directory": "/remote"}})
        }
        "user" | "synthetic" | "system" => json!({"text": "hello"}),
        "skill" => json!({"skill": "skill", "name": "review", "text": "instructions"}),
        "shell" => {
            json!({"shellID": "sh_test", "command": "echo", "status": "running", "output": {"output": "hello", "cursor": 0, "size": 5, "truncated": false}})
        }
        "assistant" => {
            json!({"agent": "build", "model": {"id": "model", "providerID": "provider"}, "content": [{"type": "text", "text": "hello", "future": true}]})
        }
        "compaction" => json!({"status": "running", "reason": "auto", "summary": ""}),
        "idle" => json!({"outcome": "succeeded"}),
        _ => unreachable!(),
    };
    let mut value = json!({"type": kind, "id": "msg_test", "time": {"created": 1, "futureTime": true}, "future": 1});
    value
        .as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    value
}

#[test]
fn every_message_kind_round_trips_additive_fields_and_has_a_shared_id() {
    for kind in [
        "agent-switched",
        "model-switched",
        "location-switched",
        "user",
        "synthetic",
        "system",
        "skill",
        "shell",
        "assistant",
        "compaction",
        "idle",
    ] {
        let raw = message(kind);
        let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
        let _: &opencoders::models::SessionMessageId = parsed.id();
        assert_eq!(parsed.message_type().to_string(), kind);
        assert_eq!(serde_json::to_value(parsed).unwrap(), raw, "{kind}");
    }
}

#[test]
fn unknown_tags_and_malformed_known_messages_are_decoding_errors() {
    for raw in [
        json!({"type":"future"}),
        json!({"type":"user","id":"msg_a"}),
        json!({"type":3}),
    ] {
        assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
    }
    let mut raw = message("user");
    raw["id"] = json!("invalid");
    assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
}

#[test]
fn numbers_and_arbitrary_metadata_retain_precision_and_nested_values() {
    let mut raw = message("assistant");
    raw["time"]["created"] = json!(9_007_199_254_740_993_u64);
    raw["metadata"] = json!({"private": [null, {"n": u64::MAX}]});
    raw["tokens"] =
        json!({"input": u64::MAX, "output": 2, "reasoning": 0, "cache": {"read": 0, "write": 0}});
    raw["native"] = json!({"provider": {"arbitrary": true}});
    raw["cost"] = json!(0.0123);
    let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
}

#[test]
fn message_pages_preserve_envelope_and_cursor_extensions() {
    let raw = json!({"data": [message("user")], "cursor": {"next":"opaque+/?&雪", "future":1}, "future":2});
    let page: SessionMessagesResponse = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(page.cursor.next.as_deref(), Some("opaque+/?&雪"));
    assert_eq!(serde_json::to_value(page).unwrap(), raw);
    for cursor in [
        json!({}),
        json!({"next":null,"previous":null}),
        json!({"next":""}),
    ] {
        let page: SessionMessagesResponse =
            serde_json::from_value(json!({"data":[],"cursor":cursor})).unwrap();
        assert_eq!(page.cursor.next.as_deref(), cursor["next"].as_str());
    }
}

#[test]
fn message_numeric_constraints_reject_invalid_values() {
    for field in ["cursor", "size"] {
        for invalid in [
            json!(-1),
            json!(1.5),
            json!(9_007_199_254_740_992_u64),
            json!(null),
        ] {
            let mut raw = message("shell");
            raw["output"][field] = invalid;
            assert!(
                serde_json::from_value::<SessionMessageInfo>(raw).is_err(),
                "{field}"
            );
        }
    }
    for (field, invalid) in [
        ("status", json!(99)),
        ("status", json!(600)),
        ("retryAfterMs", json!(-1)),
        ("retryAfterMs", json!(9_007_199_254_740_992_u64)),
        ("status", json!(null)),
    ] {
        let mut raw = message("idle");
        raw["error"] = json!({"type":"example","message":"details"});
        raw["error"][field] = invalid;
        assert!(
            serde_json::from_value::<SessionMessageInfo>(raw).is_err(),
            "{field}"
        );
    }
    for invalid in [
        json!(0),
        json!(-1),
        json!(1.5),
        json!(9_007_199_254_740_992_u64),
    ] {
        let mut raw = message("assistant");
        raw["retry"] =
            json!({"attempt":invalid,"at":1,"error":{"type":"example","message":"details"}});
        assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
    }
}

#[test]
fn tool_states_and_compaction_are_dispatched_by_their_tags() {
    for state in [
        json!({"status":"streaming","input":"{"}),
        json!({"status":"running","input":{"path":"a"},"metadata":{"progress":1}}),
        json!({"status":"completed","input":{},"content":[{"type":"text","text":"result","extra":1},{"type":"file","uri":"file:///remote/a","mime":"text/plain"}]}),
        json!({"status":"error","input":{},"error":{"type":"failure","message":"failed","status":500,"retryAfterMs":0}}),
    ] {
        let mut raw = message("assistant");
        raw["content"] = json!([
            {"type":"reasoning","text":"thinking","time":{"created":1,"completed":2},"extra":true},
            {"type":"tool","id":"call","name":"read","state":state,"time":{"created":1},"extra":true}
        ]);
        let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    }
    for state in [
        json!({"status":"future","input":{}}),
        json!({"status":"completed","input":{},"content":[]}),
        json!({"status":"error","input":{},"error":{"type":"x","message":"m"},"content":[]}),
    ] {
        let mut raw = message("assistant");
        raw["content"] =
            json!([{ "type":"tool","id":"call","name":"read","time":{"created":1},"state":state}]);
        assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
    }
    for status in ["running", "completed", "failed"] {
        let mut raw = message("compaction");
        raw["status"] = json!(status);
        if status == "failed" {
            raw.as_object_mut().unwrap().remove("summary");
            raw["error"] = json!({"type":"x","message":"m"});
        }
        let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    }
    let mut raw = message("compaction");
    raw["status"] = json!("future");
    assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
}

#[test]
fn valid_numeric_boundaries_and_attachment_payloads_decode() {
    let mut raw = message("assistant");
    raw["retry"] = json!({"attempt":9_007_199_254_740_991_u64,"at":1,"error":{"type":"x","message":"m","status":599,"retryAfterMs":9_007_199_254_740_991_u64}});
    let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    let mut raw = message("shell");
    raw["output"]["size"] = json!(9_007_199_254_740_991_u64);
    raw["exit"] = json!("NaN");
    assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_ok());
    let mut raw = message("user");
    raw["files"] = json!([{"data":"aGk=","mime":"text/plain","source":{"type":"uri","uri":"file:///a","future":1},"mention":{"start":0,"end":2,"text":"hi"},"future":true}]);
    raw["agents"] = json!([{"name":"build","future":true}]);
    raw["skills"] = json!([{"id":"skill","name":"review","text":"instructions"}]);
    let parsed: SessionMessageInfo = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    raw["files"][0]["data"] = json!("not base64!");
    assert!(serde_json::from_value::<SessionMessageInfo>(raw).is_err());
}
