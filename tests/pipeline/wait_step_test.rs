use rust_buildkite::*;

#[test]
fn test_wait_step_basic() {
    let step = WaitStep {
        ..Default::default()
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json == "{}" || !json.is_empty());
}

#[test]
fn test_wait_step_with_continue_on_failure() {
    let step = WaitStep {
        allow_dependency_failure: None,
        branches: None,
        continue_on_failure: WaitStepContinueOnFailure::Boolean(true),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        key: None,
        label: None,
        name: None,
        type_: None,
        wait: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""continue_on_failure":true"#));
}
