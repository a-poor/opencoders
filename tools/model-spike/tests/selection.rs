use opencoders_model_spike::select_schemas;
use serde_json::json;

#[test]
fn selects_transitive_references_without_unrelated_schemas() {
    let spec = json!({"components": {"schemas": {
        "Root": {"type": "object", "properties": {
            "child": {"$ref": "#/components/schemas/Child"}
        }},
        "Child": {"type": "string"},
        "Unrelated": {"type": "integer"}
    }}});
    let selected = select_schemas(&spec, &["Root"]).unwrap();
    assert_eq!(selected["definitions"].as_object().unwrap().len(), 2);
    assert_eq!(
        selected["definitions"]["Root"]["properties"]["child"]["$ref"],
        "#/definitions/Child"
    );
}

#[test]
fn recursive_references_terminate_and_remain_references() {
    // The pinned upstream schema has no cycles; this is a synthetic capability probe.
    let spec = json!({"components": {"schemas": {
        "Node": {"type": "object", "properties": {
            "next": {"$ref": "#/components/schemas/Node"}
        }}
    }}});
    let selected = select_schemas(&spec, &["Node"]).unwrap();
    assert_eq!(selected["definitions"].as_object().unwrap().len(), 1);
    assert_eq!(
        selected["definitions"]["Node"]["properties"]["next"]["$ref"],
        "#/definitions/Node"
    );
}

#[test]
fn missing_references_fail_instead_of_generating_partial_models() {
    let spec = json!({"components": {"schemas": {
        "Root": {"$ref": "#/components/schemas/Missing"}
    }}});
    let error = select_schemas(&spec, &["Root"]).unwrap_err();
    assert!(error.contains("Missing"), "{error}");
}

#[test]
fn external_references_fail_without_network_access() {
    let spec = json!({"components": {"schemas": {
        "Root": {"$ref": "https://example.invalid/schema.json"}
    }}});
    let error = select_schemas(&spec, &["Root"]).unwrap_err();
    assert!(error.contains("unsupported reference"), "{error}");
}
