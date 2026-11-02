use opencoders::models::{AgentInfo, AgentListResponse, ProviderRequest};
use serde_json::{Value, json};

fn agent() -> Value {
    json!({"id": "build", "name": "Build", "mode": "primary", "hidden": false,
        "request": {"settings": {}, "headers": {}, "body": {}}, "permissions": []})
}

#[test]
fn optional_steps_accepts_absence_and_positive_safe_integers() {
    assert!(
        serde_json::from_value::<AgentInfo>(agent())
            .unwrap()
            .steps
            .is_none()
    );
    for steps in [1, 9_007_199_254_740_991_u64] {
        let mut value = agent();
        value["steps"] = json!(steps);
        assert_eq!(
            serde_json::from_value::<AgentInfo>(value).unwrap().steps,
            Some(steps)
        );
    }
    for steps in [
        json!(0),
        json!(-1),
        json!(1.5),
        json!(null),
        json!(9_007_199_254_740_992_u64),
    ] {
        let mut value = agent();
        value["steps"] = steps;
        assert!(serde_json::from_value::<AgentInfo>(value).is_err());
    }
}

#[test]
fn cache_warming_intervals_and_tagged_compaction_round_trip() {
    for interval in ["auto", "30s"] {
        for compaction in ["summary", "native"] {
            let mut value = agent();
            value["request"]["settings"] = json!({
                "cache": {"ttl": "5m", "warming": {"interval": interval, "duration": "1h", "future": true}, "future": 1},
                "compaction": {"type": compaction, "future": 2}, "encoding": "gzip", "transport": "http"
            });
            let parsed: AgentInfo = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), value);
        }
    }
}

#[test]
fn provider_timeouts_accept_numbers_or_false_but_reject_true() {
    for field in ["timeout", "headerTimeout", "chunkTimeout"] {
        for timeout in [json!(false), json!(2500), json!(0.5)] {
            let mut value = agent();
            value["request"]["settings"][field] = timeout;
            let parsed: AgentInfo = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(serde_json::to_value(parsed).unwrap(), value);
        }
        let mut value = agent();
        value["request"]["settings"][field] = json!(true);
        assert!(
            serde_json::from_value::<AgentInfo>(value).is_err(),
            "{field} accepts only false, not true"
        );
    }
}

#[test]
fn request_debug_does_not_expose_provider_credentials() {
    let request: ProviderRequest = serde_json::from_value(json!({
        "headers": {"authorization": "private-key"}, "body": {"key": "private-body"},
        "settings": {"credential": "private-setting"}, "future": "private-extra"
    }))
    .unwrap();
    assert!(!format!("{request:?}").contains("private-"));
    assert_eq!(request.headers["authorization"], "private-key");
}

#[test]
fn malformed_known_models_and_unknown_enum_values_fail_decoding() {
    for (pointer, replacement) in [
        ("/mode", json!("future-mode")),
        ("/request/body", json!([])),
        ("/request/headers", json!({"bad": 1})),
        ("/request/settings", json!({"cache": {"ttl": "garbage"}})),
        (
            "/permissions",
            json!([{"action": "read", "resource": "*", "effect": "future-policy"}]),
        ),
    ] {
        let mut value = agent();
        *value.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            serde_json::from_value::<AgentInfo>(value).is_err(),
            "{pointer}"
        );
    }
}

#[test]
fn empty_agent_lists_preserve_the_resolved_location() {
    let value = json!({"location": {"directory": "/remote"}, "data": []});
    let response: AgentListResponse = serde_json::from_value(value.clone()).unwrap();
    assert!(response.data.is_empty());
    assert_eq!(serde_json::to_value(response).unwrap(), value);
}
