use rust_buildkite::*;

#[test]
fn test_block_step_with_block() {
    let step = BlockStep {
        allow_dependency_failure: None,
        allowed_teams: None,
        block: Some("Release Gate".to_string()),
        blocked_state: BlockStepBlockedState::default(),
        branches: None,
        depends_on: None,
        fields: None,
        id: None,
        identifier: None,
        if_: None,
        key: None,
        label: None,
        name: None,
        prompt: None,
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""block":"Release Gate""#));
}

#[test]
fn test_block_step_with_key() {
    let key: Key = "release-gate".to_string().try_into().unwrap();
    let step = BlockStep {
        allow_dependency_failure: None,
        allowed_teams: None,
        block: None,
        blocked_state: BlockStepBlockedState::default(),
        branches: None,
        depends_on: None,
        fields: None,
        id: None,
        identifier: None,
        if_: None,
        key: Some(key),
        label: None,
        name: None,
        prompt: None,
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"release-gate""#));
}

#[test]
fn test_block_step_with_blocked_state() {
    let step = BlockStep {
        allow_dependency_failure: None,
        allowed_teams: None,
        block: None,
        blocked_state: BlockStepBlockedState::Passed,
        branches: None,
        depends_on: None,
        fields: None,
        id: None,
        identifier: None,
        if_: None,
        key: None,
        label: None,
        name: None,
        prompt: None,
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""blocked_state":"passed""#));
}
