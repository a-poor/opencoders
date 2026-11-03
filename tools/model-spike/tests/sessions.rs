use opencoders_model_spike::sessions::generate_sessions;
use serde_json::{Value, json};

#[test]
fn fork_adapters_require_nullable_message_boundary_and_shared_response_envelope() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    let route = "/paths/~1api~1session~1{sessionID}~1fork/post";
    let input = format!("{route}/requestBody/content/application~1json/schema");
    for (key, value) in [
        ("required", json!(["before"])),
        ("additionalProperties", json!(true)),
        (
            "properties",
            json!({"before":{"anyOf":[{"type":"string"},{"type":"null"}]}}),
        ),
    ] {
        let mut changed = spec.clone();
        changed.pointer_mut(&input).unwrap()[key] = value;
        assert!(generate_sessions(&changed).is_err(), "{key}");
    }
    let mut changed = spec;
    changed
        .pointer_mut(&format!(
            "{route}/responses/200/content/application~1json/schema"
        ))
        .unwrap()["required"] = json!([]);
    assert!(generate_sessions(&changed).is_err());
}

#[test]
fn session_adapters_require_review_when_file_diff_constraints_change() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for (pointer, replacement, message) in [
        (
            "/components/schemas/FileDiff.Info/properties/additions/maximum",
            json!(100),
            "additions",
        ),
        (
            "/components/schemas/FileDiff.Info/properties/deletions/minimum",
            json!(-1),
            "deletions",
        ),
        (
            "/components/schemas/FileDiff.Info/properties/more/enum",
            json!([false]),
            "more",
        ),
    ] {
        let mut changed = spec.clone();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(
            generate_sessions(&changed)
                .unwrap_err()
                .to_string()
                .contains(message)
        );
    }
}

#[test]
fn create_adapters_require_optional_nullable_fields_and_matching_response_envelopes() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    let request = "/paths/~1api~1session/post/requestBody/content/application~1json/schema";
    for pointer in [
        format!("{request}/properties/title"),
        format!("{request}/required"),
    ] {
        let mut changed = spec.clone();
        if pointer.ends_with("required") {
            changed.pointer_mut(request).unwrap()["required"] = json!(["title"]);
        } else {
            *changed.pointer_mut(&pointer).unwrap() = json!({"type": "string"});
        }
        assert!(generate_sessions(&changed).is_err());
    }
    let mut changed = spec.clone();
    *changed.pointer_mut("/paths/~1api~1session~1{sessionID}/get/responses/200/content/application~1json/schema/required").unwrap() = json!([]);
    assert!(generate_sessions(&changed).is_err());
    let mut changed = spec;
    *changed
        .pointer_mut("/components/schemas/Model.Ref/properties/variant")
        .unwrap() = json!({"type": ["string", "null"]});
    assert!(generate_sessions(&changed).is_err());
}

#[test]
fn update_adapters_require_optional_nullable_fields() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    let pointer =
        "/paths/~1api~1session~1{sessionID}/patch/requestBody/content/application~1json/schema";
    for field in ["title", "metadata", "permissions", "archived"] {
        let mut changed = spec.clone();
        changed.pointer_mut(pointer).unwrap()["properties"][field] = json!({"type": "string"});
        assert!(generate_sessions(&changed).is_err());
    }
    let mut changed = spec;
    changed.pointer_mut(pointer).unwrap()["required"] = json!(["archived"]);
    assert!(generate_sessions(&changed).is_err());
}

#[test]
fn selection_inputs_require_the_reviewed_nonnullable_fields() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for route in ["agent", "model"] {
        let pointer = format!(
            "/paths/~1api~1session~1{{sessionID}}~1{route}/post/requestBody/content/application~1json/schema"
        );
        let mut changed = spec.clone();
        changed.pointer_mut(&pointer).unwrap()["required"] = json!([]);
        assert!(generate_sessions(&changed).is_err());
        let mut changed = spec.clone();
        changed.pointer_mut(&pointer).unwrap()["additionalProperties"] = json!(true);
        assert!(generate_sessions(&changed).is_err());
    }
}
