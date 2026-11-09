use opencoders::models::{
    FileListResponse, PtyConnectTokenResponse, PtyCreateRequest, PtyInfo, PtyUpdateRequest,
    VcsBaseResponse, VcsStatusResponse,
};
use serde_json::{Value, json};

#[test]
fn workspace_envelopes_preserve_extensions_order_and_required_nullable_data() {
    let value = json!({"location":{"directory":"/work","new":1},"data":[{"path":"z/","type":"directory","new":true},{"path":"a","type":"file"}],"future":2});
    let response: FileListResponse = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(response).unwrap(), value);
    let value = json!({"location":{"directory":"/work"},"data":null});
    let response: VcsBaseResponse = serde_json::from_value(value.clone()).unwrap();
    assert!(response.data.is_none());
    assert_eq!(serde_json::to_value(response).unwrap(), value);
    assert!(
        serde_json::from_value::<VcsBaseResponse>(json!({"location":{"directory":"/work"}}))
            .is_err()
    );
}

#[test]
fn workspace_counts_and_pty_numbers_enforce_reviewed_bounds() {
    for number in [
        json!(-1),
        json!(1.5),
        json!(9_007_199_254_740_992_u64),
        Value::Null,
    ] {
        assert!(serde_json::from_value::<VcsStatusResponse>(json!({"location":{"directory":"/work"},"data":[{"file":"a","status":"added","additions":number,"deletions":0}]})).is_err());
        assert!(serde_json::from_value::<PtyInfo>(json!({"id":"pty_test","args":[],"command":"sh","cwd":"/work","status":"running","title":"T","pid":number})).is_err());
    }
    let running: PtyInfo=serde_json::from_value(json!({"id":"pty_test","args":[],"command":"sh","cwd":"/work","status":"running","title":"T","pid":0})).unwrap();
    assert!(running.exit_code.is_none());
}

#[test]
fn pty_inputs_preserve_omission_and_empty_values_and_reject_null_and_unknown_fields() {
    let omitted: PtyCreateRequest = serde_json::from_value(json!({})).unwrap();
    assert_eq!(serde_json::to_value(omitted).unwrap(), json!({}));
    let empty: PtyCreateRequest =
        serde_json::from_value(json!({"args":[],"env":{},"title":""})).unwrap();
    assert_eq!(
        serde_json::to_value(empty).unwrap(),
        json!({"args":[],"env":{},"title":""})
    );
    for field in ["args", "env", "title", "command", "cwd", "unexpected"] {
        assert!(serde_json::from_value::<PtyCreateRequest>(json!({field:null})).is_err());
    }
    for body in [
        json!({"size":null}),
        json!({"size":{"rows":0,"cols":80}}),
        json!({"size":{"rows":24,"cols":80,"extra":1}}),
        json!({"title":null}),
    ] {
        assert!(serde_json::from_value::<PtyUpdateRequest>(body).is_err());
    }
}

#[test]
fn ticket_and_environment_debug_output_omit_secrets() {
    let ticket: PtyConnectTokenResponse=serde_json::from_value(json!({"location":{"directory":"/work"},"data":{"ticket":"secret-ticket","expires_in":60,"secret":"secret-extra"}})).unwrap();
    assert!(!format!("{ticket:?}").contains("secret-"));
    let input: PtyCreateRequest =
        serde_json::from_value(json!({"env":{"KEY":"secret-env"},"args":["secret-arg"]})).unwrap();
    assert!(!format!("{input:?}").contains("secret-"));
}
