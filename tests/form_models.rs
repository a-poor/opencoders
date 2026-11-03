use opencoders::{RequestField, models::*};
use serde_json::{Value, json};

#[test]
fn creation_preserves_optional_empty_false_null_and_nonempty_fields() {
    let mut field = FormStringFieldInput::new("name");
    field.required = Some(false);
    field.options = Some(vec![]);
    field.when = Some(vec![]);
    field.default = Some(String::new());
    field.min_length = Some(0.try_into().unwrap());
    let mut request = SessionFormCreateRequest::new("", FormFieldsInput::new(field));
    let expected = json!({"title":"","fields":[{"key":"name","type":"string","required":false,"options":[],"when":[],"default":"","minLength":0}]});
    assert_eq!(serde_json::to_value(&request).unwrap(), expected);
    request.id = RequestField::Null;
    request.metadata = Some(serde_json::Map::new().into());
    let mut expected = expected;
    expected["id"] = Value::Null;
    expected["metadata"] = json!({});
    assert_eq!(serde_json::to_value(&request).unwrap(), expected);
    let decoded: SessionFormCreateRequest = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), expected);
    request = request.id("frm_/../?雪".parse().unwrap());
    assert_eq!(serde_json::to_value(request).unwrap()["id"], "frm_/../?雪");
    assert!("frm".parse::<FormId>().is_err());
    assert!(FormFieldsInput::try_from(vec![]).is_err());
    assert!(EventInteger::<0, 9007199254740991>::try_from(9007199254740992).is_err());
}

#[test]
fn all_input_field_kinds_are_strict_and_round_trip_constraints() {
    for raw in [
        json!({"type":"string","key":"s","format":"email","minLength":0,"maxLength":9007199254740991_u64,"pattern":".+","placeholder":"x","custom":true,"options":[{"value":"a","label":"A","description":""}],"when":[{"key":"b","op":"neq","value":false}]}),
        json!({"type":"number","key":"n","minimum":"-Infinity","maximum":"Infinity","default":"NaN"}),
        json!({"type":"integer","key":"i","minimum":1.5,"maximum":9007199254740993_u64,"default":2}),
        json!({"type":"boolean","key":"b","default":false,"hidden":true,"required":true}),
        json!({"type":"multiselect","key":"m","options":[],"minItems":0,"maxItems":2,"default":[],"custom":false}),
        json!({"type":"external","key":"e","url":"","title":"","description":""}),
    ] {
        let v: FormFieldInput = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
        for key in raw.as_object().unwrap().keys() {
            let mut bad = raw.clone();
            bad[key] = Value::Null;
            assert!(
                serde_json::from_value::<FormFieldInput>(bad).is_err(),
                "null {key}"
            );
        }
        let mut bad = raw.clone();
        bad["future"] = json!(true);
        assert!(serde_json::from_value::<FormFieldInput>(bad).is_err());
        let mut bad = raw;
        bad.as_object_mut().unwrap().remove("key");
        assert!(serde_json::from_value::<FormFieldInput>(bad).is_err());
    }
    for raw in [
        json!({"type":"future","key":"x"}),
        json!({"type":"multiselect","key":"x"}),
        json!({"type":"external","key":"x"}),
        json!({"type":"string","key":"x","minLength":-1}),
        json!({"type":"string","key":"x","maxLength":9007199254740992_u64}),
        json!({"type":"number","key":"x","minimum":"garbage"}),
    ] {
        assert!(serde_json::from_value::<FormFieldInput>(raw).is_err());
    }
    for raw in [
        json!({"title":"x","fields":[]}),
        json!({"title":"x","fields":[{"type":"boolean","key":"x"}],"metadata":null}),
        json!({"title":"x","fields":[{"type":"boolean","key":"x"}],"extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionFormCreateRequest>(raw).is_err());
    }
}

#[test]
fn replies_states_and_response_models_preserve_values_and_extensions() {
    let answer = json!({"text":"Infinity","number":9007199254740993_u64,"bool":false,"selected":["a","a"],"ack":true});
    let reply = SessionFormReplyRequest::new(serde_json::from_value(answer.clone()).unwrap());
    assert_eq!(
        serde_json::to_value(reply).unwrap(),
        json!({"answer":answer})
    );
    for state in [
        json!({"status":"pending","extra":1}),
        json!({"status":"answered","answer":answer,"extra":1}),
        json!({"status":"cancelled","message":"","extra":1}),
    ] {
        let raw = json!({"data":{"id":"frm_x","sessionID":"unprefixed","title":"x","fields":[{"type":"boolean","key":"x","future":true}],"metadata":{"opaque":null},"state":state,"future":1},"extra":true});
        let v: SessionFormResponse = serde_json::from_value(raw.clone()).unwrap();
        let _: &opencoders::models::events::FormId = &v.data.id;
        let _: &opencoders::models::events::FormFields1 = &v.data.fields;
        assert_eq!(serde_json::to_value(v).unwrap(), raw);
    }
    for bad in [
        json!({"status":"answered"}),
        json!({"status":"future"}),
        json!({"status":"answered","answer":null}),
        json!({}),
    ] {
        assert!(serde_json::from_value::<FormState>(bad).is_err());
    }
    for bad in [
        json!({}),
        json!({"answer":null}),
        json!({"answer":{"x":null}}),
        json!({"answer":{},"extra":1}),
    ] {
        assert!(serde_json::from_value::<SessionFormReplyRequest>(bad).is_err());
    }
}
