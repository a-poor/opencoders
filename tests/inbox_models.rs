use opencoders::{
    RequestField,
    models::{
        PromptInputAgentAttachment, PromptInputFileAttachment, PromptInputFileSource,
        PromptInputMention, PromptInputSkillAttachment, SessionInboxDelivery, SessionInboxInfo,
        SessionInboxResponse, SessionInboxUpdateRequest, SessionPromptRequest,
        SessionPromptResponse,
    },
};
use serde_json::{Value, json};

fn items() -> Vec<Value> {
    serde_json::from_str(include_str!("fixtures/inbox.json")).unwrap()
}

#[test]
fn prompt_preserves_omitted_null_empty_and_false_fields() {
    let mut request = SessionPromptRequest::new("");
    assert_eq!(serde_json::to_value(&request).unwrap(), json!({"text":""}));
    request.id = RequestField::Null;
    request.delivery = RequestField::Null;
    request.resume = RequestField::Null;
    request.files = Some(vec![]);
    request.agents = Some(vec![]);
    request.skills = Some(vec![]);
    request.metadata = Some(Default::default());
    assert_eq!(
        serde_json::to_value(&request).unwrap(),
        json!({"text":"","id":null,"delivery":null,"resume":null,"files":[],"agents":[],"skills":[],"metadata":{}})
    );
    let request = request
        .id("msg_custom".parse().unwrap())
        .delivery(SessionInboxDelivery::Queue)
        .resume(false);
    let raw = serde_json::to_value(&request).unwrap();
    assert_eq!(raw["resume"], false);
    assert_eq!(raw["delivery"], "queue");
    assert_eq!(raw["id"], "msg_custom");
    assert_eq!(
        serde_json::to_value(serde_json::from_value::<SessionPromptRequest>(raw.clone()).unwrap())
            .unwrap(),
        raw
    );
}

#[test]
fn prompt_inputs_are_strict_and_nonnullable_except_reviewed_fields() {
    for raw in [
        json!({}),
        json!({"text":null}),
        json!({"text":"x","extra":1}),
        json!({"text":"x","delivery":"later"}),
        json!({"text":"x","id":"bad"}),
    ] {
        assert!(serde_json::from_value::<SessionPromptRequest>(raw).is_err());
    }
    for field in ["files", "agents", "skills", "metadata"] {
        let mut raw = json!({"text":"x"});
        raw[field] = Value::Null;
        assert!(
            serde_json::from_value::<SessionPromptRequest>(raw).is_err(),
            "{field}"
        );
    }
    for raw in [
        json!({}),
        json!({"delivery":null}),
        json!({"delivery":"later"}),
        json!({"delivery":"queue","text":"cannot-edit"}),
    ] {
        assert!(serde_json::from_value::<SessionInboxUpdateRequest>(raw).is_err());
    }
    assert_eq!(
        serde_json::to_value(SessionInboxUpdateRequest::new(SessionInboxDelivery::Steer)).unwrap(),
        json!({"delivery":"steer"})
    );
}

#[test]
fn attachment_inputs_use_uris_and_strict_nested_types() {
    for raw in [
        json!({"type":"inline","uri":null}),
        json!({"type":"inline","extra":1}),
        json!({"type":"uri","uri":"x","extra":1}),
        json!({"type":"uri"}),
        json!({"type":"future"}),
    ] {
        assert!(serde_json::from_value::<PromptInputFileSource>(raw).is_err());
    }
    let mention = PromptInputMention::new(1.into(), 4.into(), "@x");
    let mut file = PromptInputFileAttachment::new("data:text/plain;base64,aGk=");
    file.mention = Some(mention.clone());
    file.source = Some(PromptInputFileSource::uri("file:///server/note.txt"));
    let mut agent = PromptInputAgentAttachment::new("build");
    agent.mention = Some(mention.clone());
    let mut skill = PromptInputSkillAttachment::new("skill_test".to_owned().into());
    skill.mention = Some(mention);
    let request = SessionPromptRequest::new("hello")
        .files(vec![file])
        .agents(vec![agent])
        .skills(vec![skill]);
    let raw = serde_json::to_value(request).unwrap();
    assert_eq!(
        serde_json::to_value(serde_json::from_value::<SessionPromptRequest>(raw.clone()).unwrap())
            .unwrap(),
        raw
    );
    assert!(matches!(
        serde_json::from_value::<PromptInputFileSource>(json!({"type":"inline"})).unwrap(),
        PromptInputFileSource::Inline
    ));
    assert_eq!(
        raw["files"][0]["source"],
        json!({"type":"uri","uri":"file:///server/note.txt"})
    );
    assert!(raw["files"][0].get("data").is_none());
    for (field, inner) in [
        ("files", json!({"uri":"x","source":null})),
        (
            "files",
            json!({"uri":"x","mention":{"start":0,"end":1,"text":"x","extra":1}}),
        ),
        ("files", json!({"uri":"x","name":null})),
        ("agents", json!({"name":"build","extra":1})),
        ("skills", json!({"id":"skill_x","mention":null})),
    ] {
        let mut raw = json!({"text":"x"});
        raw[field] = json!([inner]);
        assert!(serde_json::from_value::<SessionPromptRequest>(raw).is_err());
    }
}

#[test]
fn inbox_dispatches_all_kinds_retains_order_extensions_and_numeric_precision() {
    let raw = json!({"data":items(),"future":true});
    let response: SessionInboxResponse = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(&response.data[0], SessionInboxInfo::User(_)));
    assert!(matches!(&response.data[1], SessionInboxInfo::Synthetic(_)));
    assert!(matches!(&response.data[2], SessionInboxInfo::Compaction(_)));
    assert!(matches!(&response.data[3], SessionInboxInfo::Move(_)));
    for item in &response.data {
        assert_eq!(item.session_id().as_str(), "ses_test");
        assert_eq!(item.delivery(), SessionInboxDelivery::Queue);
    }
    assert_eq!(response.data[0].id().as_str(), "msg_user");
    assert_eq!(serde_json::to_value(response).unwrap(), raw);
    let raw = json!({"data":items()[0],"future":true});
    let admitted: SessionPromptResponse = serde_json::from_value(raw.clone()).unwrap();
    let _: &opencoders::models::SessionMessageId = &admitted.data.id;
    let _: &opencoders::models::PromptFileAttachment = &admitted.data.payload.files[0];
    assert_eq!(serde_json::to_value(admitted).unwrap(), raw);
}

#[test]
fn inbox_rejects_unknown_or_malformed_kinds_and_null_compaction() {
    for raw in [
        json!({"type":"future"}),
        json!({"type":"user"}),
        json!({"id":"msg_x"}),
    ] {
        assert!(serde_json::from_value::<SessionInboxInfo>(raw).is_err());
    }
    for value in [
        json!({}),
        json!([]),
        json!(false),
        json!(1),
        json!("opaque"),
    ] {
        let mut raw = items()[2].clone();
        raw["payload"] = value;
        assert_eq!(
            serde_json::to_value(serde_json::from_value::<SessionInboxInfo>(raw.clone()).unwrap())
                .unwrap(),
            raw
        );
    }
    for (index, field, value) in [
        (2, "payload", json!(null)),
        (0, "delivery", json!("later")),
        (0, "id", json!("bad")),
        (0, "sessionID", json!("bad")),
    ] {
        let mut raw = items()[index].clone();
        raw[field] = value;
        assert!(serde_json::from_value::<SessionInboxInfo>(raw).is_err());
    }
}
