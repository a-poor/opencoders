use opencoders_model_spike::workspace::{generate_ptys, generate_workspace};
use serde_json::{Value, json};

fn spec() -> Value {
    serde_json::from_str(include_str!("../../../openapi/opencode-v2.json")).unwrap()
}

#[test]
fn worktree_mutations_require_review_when_nullability_force_or_success_contract_changes() {
    for (pointer, replacement) in [
        (
            "/components/schemas/Worktree.CreateInput/properties/branch",
            json!({"type":["string","null"]}),
        ),
        (
            "/components/schemas/Worktree.RemoveInput/required",
            json!(["projectID", "directory"]),
        ),
        (
            "/paths/~1api~1worktree/post/responses/200/content/application~1json/schema",
            json!({"type":"boolean"}),
        ),
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(generate_workspace(&changed).is_err(), "{pointer}");
    }
}

#[test]
fn workspace_adapters_require_review_of_nullable_base_and_integer_bounds() {
    for (pointer, replacement) in [
        (
            "/components/schemas/Vcs.FileStatus/properties/additions/maximum",
            json!(100),
        ),
        (
            "/paths/~1api~1vcs~1base/get/responses/200/content/application~1json/schema/required",
            json!(["location"]),
        ),
        (
            "/paths/~1api~1fs~1find/get/responses/200/content/application~1json/schema/properties/data",
            json!({"type":"string"}),
        ),
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(generate_workspace(&changed).is_err(), "{pointer}");
    }
}

#[test]
fn pty_adapters_require_review_of_strict_inputs_and_numeric_constraints() {
    for (pointer, replacement) in [
        ("/components/schemas/Pty/properties/pid/maximum", json!(100)),
        (
            "/components/schemas/PtyTicket.ConnectToken/properties/expires_in/exclusiveMinimum",
            json!(-1),
        ),
        (
            "/paths/~1api~1pty/post/requestBody/content/application~1json/schema/properties/command",
            json!({"type":["string","null"]}),
        ),
        (
            "/paths/~1api~1pty~1{ptyID}/put/requestBody/content/application~1json/schema/properties/size/properties/rows/exclusiveMinimum",
            json!(1),
        ),
    ] {
        let mut changed = spec();
        *changed.pointer_mut(pointer).unwrap() = replacement;
        assert!(generate_ptys(&changed).is_err(), "{pointer}");
    }
}
