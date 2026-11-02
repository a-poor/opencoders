use opencoders::{
    RequestField,
    models::{SessionCreateRequest, SessionModelInput},
};
use serde_json::{Value, json};

#[test]
fn create_request_preserves_omitted_null_and_values() {
    assert_eq!(
        serde_json::to_value(SessionCreateRequest::default()).unwrap(),
        json!({})
    );
    let names = [
        "id",
        "parentID",
        "title",
        "agent",
        "model",
        "location",
        "metadata",
        "permissions",
    ];
    for name in names {
        let wire = json!({name: null});
        let request: SessionCreateRequest = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(serde_json::to_value(request).unwrap(), wire);
    }
    let wire = json!({
        "id": "ses_create", "parentID": "ses_parent", "title": "", "agent": "build",
        "model": {"providerID": "provider", "id": "model", "variant": ""},
        "location": {"directory": "/server/a +雪"},
        "metadata": {"nested": {"null": null, "list": [1, false]}},
        "permissions": [{"action": "read", "resource": "*", "effect": "ask"}]
    });
    let mut request: SessionCreateRequest = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(&request).unwrap(), wire);
    request.title = RequestField::Omitted;
    request.permissions = RequestField::Value(vec![].into());
    let value = serde_json::to_value(request).unwrap();
    assert!(value.get("title").is_none());
    assert_eq!(value["permissions"], json!([]));
}

#[test]
fn create_inputs_reject_unknown_fields_and_invalid_shapes() {
    for wire in [
        json!({"extra": true}),
        json!({"id": "wrong-prefix"}),
        json!({"title": 3}),
        json!({"metadata": []}),
        json!({"location": {"directory": "/work", "extra": true}}),
        json!({"model": {"providerID": "p", "id": "m", "variant": null}}),
        json!({"model": {"providerID": "p", "id": "m", "extra": true}}),
        json!({"permissions": [{"action": "read", "resource": "*", "effect": "unknown"}]}),
        json!({"permissions": [{"action": "read", "resource": "*", "effect": "allow", "extra": true}]}),
    ] {
        assert!(
            serde_json::from_value::<SessionCreateRequest>(wire.clone()).is_err(),
            "{wire}"
        );
    }
    let input = SessionModelInput::new("p".to_owned().into(), "m");
    assert_eq!(
        serde_json::to_value(input).unwrap(),
        json!({"providerID": "p", "id": "m"})
    );
}

#[test]
fn omitted_fields_must_be_skipped_and_null_remains_explicit() {
    assert!(serde_json::to_value(RequestField::<String>::Omitted).is_err());
    assert_eq!(
        serde_json::to_value(RequestField::<String>::Null).unwrap(),
        Value::Null
    );
    assert_eq!(
        serde_json::from_value::<RequestField<String>>(Value::Null).unwrap(),
        RequestField::Null
    );
}
