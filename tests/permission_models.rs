use opencoders::{
    RequestField,
    models::{
        PermissionEffect, PermissionId, PermissionReply, PermissionRequest, PermissionSourceInput,
        SessionPermissionCreateRequest, SessionPermissionCreateResponse,
        SessionPermissionReplyRequest,
    },
};
use serde_json::{Value, json};

#[test]
fn create_and_reply_preserve_omitted_null_empty_and_explicit_values() {
    let mut request = SessionPermissionCreateRequest::new("", vec![]);
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"action":"","resources":[]})
    );
    request.id = RequestField::Null;
    request.agent = RequestField::Null;
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"action":"","resources":[],"id":null,"agent":null})
    );
    let wire = json!({"action":"read","resources":["a","a"],"id":"per_test","agent":"", "save":[],"metadata":{},"source":{"type":"tool","id":"","messageID":"unprefixed"}});
    let request = SessionPermissionCreateRequest::new("read", vec!["a".into(), "a".into()])
        .id("per_test".parse().unwrap())
        .agent(String::new().into())
        .save(vec![])
        .metadata(serde_json::Map::new())
        .source(PermissionSourceInput::tool("unprefixed", ""));
    assert_eq!(serde_json::to_value(&request).unwrap(), wire);
    let decoded: SessionPermissionCreateRequest = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), wire);
    for decision in [
        PermissionReply::Once,
        PermissionReply::Always,
        PermissionReply::Reject,
    ] {
        let mut reply = SessionPermissionReplyRequest::new(decision);
        assert_eq!(
            serde_json::to_value(&reply).unwrap(),
            json!({"decision":decision})
        );
        reply.message = RequestField::Null;
        assert_eq!(
            serde_json::to_value(&reply).unwrap(),
            json!({"decision":decision,"message":null})
        );
        assert_eq!(
            serde_json::to_value(reply.message("")).unwrap(),
            json!({"decision":decision,"message":""})
        );
    }
}

#[test]
fn permission_inputs_reject_unknown_fields_null_nonnullable_fields_and_malformed_sources() {
    for field in ["save", "metadata", "source", "resources", "action"] {
        let mut wire = json!({"action":"read","resources":[]});
        wire[field] = Value::Null;
        assert!(
            serde_json::from_value::<SessionPermissionCreateRequest>(wire).is_err(),
            "{field}"
        );
    }
    for source in [
        json!({"type":"tool","id":"a"}),
        json!({"type":"future","id":"a","messageID":"b"}),
        json!({"type":"tool","id":"a","messageID":"b","extra":1}),
    ] {
        assert!(
            serde_json::from_value::<SessionPermissionCreateRequest>(
                json!({"action":"read","resources":[],"source":source})
            )
            .is_err()
        );
    }
    for wire in [
        json!({"action":"read"}),
        json!({"resources":[]}),
        json!({"action":"read","resources":[],"extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionPermissionCreateRequest>(wire).is_err());
    }
    for wire in [
        json!({}),
        json!({"decision":"future"}),
        json!({"decision":null}),
        json!({"decision":"once","extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionPermissionReplyRequest>(wire).is_err());
    }
}

#[test]
fn pending_requests_preserve_extensions_and_share_ids_sources_and_decisions() {
    let wire = json!({"id":"per_test","sessionID":"ses_test","action":"read","resources":["a","a"],"save":["*"],"metadata":{"x":[null,9007199254740993_u64]},"source":{"type":"tool","id":"tool","messageID":"unprefixed","extra":true},"message":"hint","future":1});
    let request: PermissionRequest = serde_json::from_value(wire.clone()).unwrap();
    let _: &opencoders::models::events::PermissionId = &request.id;
    let _: &opencoders::models::events::PermissionSource = request.source.as_ref().unwrap();
    assert_eq!(serde_json::to_value(request).unwrap(), wire);
    for effect in [
        PermissionEffect::Allow,
        PermissionEffect::Deny,
        PermissionEffect::Ask,
    ] {
        let wire = json!({"data":{"id":"per_test","effect":effect,"extra":true},"future":1});
        let result: SessionPermissionCreateResponse = serde_json::from_value(wire.clone()).unwrap();
        assert_eq!(result.data.effect, effect);
        assert_eq!(serde_json::to_value(result).unwrap(), wire);
    }
    for id in ["per", "per/../?雪", "per_test"] {
        assert!(id.parse::<PermissionId>().is_ok());
    }
    assert!("req_wrong".parse::<PermissionId>().is_err());
    for field in ["id", "sessionID", "action", "resources"] {
        let mut missing = wire.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(serde_json::from_value::<PermissionRequest>(missing).is_err());
    }
    assert!(
        serde_json::from_value::<SessionPermissionCreateResponse>(
            json!({"data":{"id":"per_x","effect":"future"}})
        )
        .is_err()
    );
}
