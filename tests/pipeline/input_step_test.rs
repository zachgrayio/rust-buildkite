use rust_buildkite::*;

#[test]
fn test_input_step_with_input() {
    let step = InputStep {
        allow_dependency_failure: None,
        allowed_teams: None,
        blocked_state: InputStepBlockedState::default(),
        branches: None,
        depends_on: None,
        fields: None,
        id: None,
        identifier: None,
        if_: None,
        input: Some("Deploy to production?".to_string()),
        key: None,
        label: None,
        name: None,
        prompt: None,
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""input":"Deploy to production?""#));
}

#[test]
fn test_input_step_with_key() {
    let key: Key = "deploy-gate".to_string().try_into().unwrap();
    let step = InputStep {
        allow_dependency_failure: None,
        allowed_teams: None,
        blocked_state: InputStepBlockedState::default(),
        branches: None,
        depends_on: None,
        fields: None,
        id: None,
        identifier: None,
        if_: None,
        input: None,
        key: Some(key),
        label: None,
        name: None,
        prompt: None,
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"deploy-gate""#));
}
