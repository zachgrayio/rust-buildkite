use rust_buildkite::*;

#[test]
fn test_group_step_with_group() {
    let step = GroupStep {
        allow_dependency_failure: None,
        depends_on: None,
        group: Some("Test Group".to_string()),
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        notify: None,
        skip: None,
        steps: GroupSteps(vec![]),
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""group":"Test Group""#));
}

#[test]
fn test_group_step_with_key() {
    let key: Key = "test-group".to_string().try_into().unwrap();
    let step = GroupStep {
        allow_dependency_failure: None,
        depends_on: None,
        group: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: Some(key),
        label: None,
        name: None,
        notify: None,
        skip: None,
        steps: GroupSteps(vec![]),
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"test-group""#));
}
