use opencoders_model_spike::permissions::generate_permissions;
use serde_json::{Value, json};
#[test]
fn permission_input_adapters_reject_optionality_and_field_drift() {
    let spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    for route in [
        "/paths/~1api~1session~1{sessionID}~1permission/post",
        "/paths/~1api~1session~1{sessionID}~1permission~1{requestID}~1reply/post",
    ] {
        for (key, value) in [
            ("required", json!([])),
            ("additionalProperties", json!(true)),
        ] {
            let mut changed = spec.clone();
            changed
                .pointer_mut(&format!(
                    "{route}/requestBody/content/application~1json/schema"
                ))
                .unwrap()[key] = value;
            assert!(
                generate_permissions(&changed)
                    .unwrap_err()
                    .to_string()
                    .contains("permission input adapters")
            );
        }
    }
    let mut changed = spec.clone();
    *changed.pointer_mut("/paths/~1api~1session~1{sessionID}~1permission/post/requestBody/content/application~1json/schema/properties/save").unwrap()=json!({"type":["array","null"],"items":{"type":"string"}});
    assert!(generate_permissions(&changed).is_err());
}
#[test]
fn strict_permission_sources_require_review_when_union_changes() {
    let mut spec: Value =
        serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap();
    spec["components"]["schemas"]["Permission.Source"]["anyOf"]
        .as_array_mut()
        .unwrap()
        .push(json!({"type":"string"}));
    assert!(
        generate_permissions(&spec)
            .unwrap_err()
            .to_string()
            .contains("Permission.Source")
    );
}
