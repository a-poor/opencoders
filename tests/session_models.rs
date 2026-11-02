use opencoders::models::{SessionId, SessionInfo};
use serde_json::{Value, json};

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/session.json")).unwrap()
}

#[test]
fn sessions_round_trip_nested_extensions_and_reuse_shared_types() {
    let raw = fixture();
    let session: SessionInfo = serde_json::from_value(raw.clone()).unwrap();
    let _: &opencoders::models::LocationPublicRef = &session.location;
    let _: &opencoders::models::ProjectId = &session.project_id;
    let _: &opencoders::models::ModelRef = session.model.as_ref().unwrap();
    let _: &opencoders::models::PermissionRuleset = session.permissions.as_ref().unwrap();
    assert_eq!(serde_json::to_value(session).unwrap(), raw);
}

#[test]
fn session_numbers_preserve_integers_without_float_rounding() {
    let mut raw = fixture();
    raw["time"]["created"] = json!(9_007_199_254_740_993_u64);
    raw["tokens"]["input"] = json!(u64::MAX);
    raw["cost"] = json!(0.12345);
    let parsed: SessionInfo = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
}

#[test]
fn session_ids_enforce_the_documented_prefix_only() {
    assert!("other".parse::<SessionId>().is_err());
    for id in ["ses", "ses_parent +?&", "ses_123"] {
        assert_eq!(id.parse::<SessionId>().unwrap().as_str(), id);
    }
    let mut raw = fixture();
    raw["id"] = json!("invalid");
    assert!(serde_json::from_value::<SessionInfo>(raw).is_err());
}

#[test]
fn fork_boundaries_validate_tags_and_keep_added_fields() {
    for tag in ["before", "through"] {
        let mut raw = fixture();
        raw["fork"]["boundary"]["type"] = json!(tag);
        let parsed: SessionInfo = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(parsed).unwrap(), raw);
    }
    for boundary in [
        json!({"type": "after", "messageID": "msg_1"}),
        json!({"type": "before"}),
        json!({"type": "through", "messageID": "bad"}),
    ] {
        let mut raw = fixture();
        raw["fork"]["boundary"] = boundary;
        assert!(serde_json::from_value::<SessionInfo>(raw).is_err());
    }
}

#[test]
fn file_diff_counts_obey_nonnegative_safe_integer_bounds() {
    for field in ["additions", "deletions"] {
        for count in [json!(0), json!(9_007_199_254_740_991_u64)] {
            let mut raw = fixture();
            raw["revert"]["files"][0][field] = count;
            assert!(serde_json::from_value::<SessionInfo>(raw).is_ok());
        }
        for count in [json!(-1), json!(1.5), json!(9_007_199_254_740_992_u64)] {
            let mut raw = fixture();
            raw["revert"]["files"][0][field] = count;
            assert!(
                serde_json::from_value::<SessionInfo>(raw).is_err(),
                "{field} must obey schema bounds"
            );
        }
    }
}

#[test]
fn file_diff_more_allows_absence_or_true_only() {
    let mut raw = fixture();
    raw["revert"]["files"][0]
        .as_object_mut()
        .unwrap()
        .remove("more");
    assert!(serde_json::from_value::<SessionInfo>(raw).is_ok());
    for more in [json!(false), json!(null), json!(1)] {
        let mut raw = fixture();
        raw["revert"]["files"][0]["more"] = more;
        assert!(serde_json::from_value::<SessionInfo>(raw).is_err());
    }
}

#[test]
fn required_session_shapes_and_known_enum_values_are_checked() {
    for field in ["id", "projectID", "cost", "tokens", "time", "location"] {
        let mut raw = fixture();
        raw.as_object_mut().unwrap().remove(field);
        assert!(
            serde_json::from_value::<SessionInfo>(raw).is_err(),
            "{field}"
        );
    }
    for (pointer, value) in [
        ("/outcome", json!("future")),
        ("/tokens/cache/read", json!("bad")),
        ("/time/created", json!(null)),
        ("/location/directory", json!(3)),
        ("/metadata", json!([])),
    ] {
        let mut raw = fixture();
        *raw.pointer_mut(pointer).unwrap() = value;
        assert!(
            serde_json::from_value::<SessionInfo>(raw).is_err(),
            "{pointer}"
        );
    }
}
