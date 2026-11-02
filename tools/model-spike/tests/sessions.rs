use opencoders_model_spike::sessions::generate_sessions;
use serde_json::{Value, json};

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
