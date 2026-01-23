use rust_buildkite::*;

#[test]
fn test_trigger_step_with_trigger() {
    let step = TriggerStep {
        allow_dependency_failure: None,
        async_: false,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        trigger: "deploy-pipeline".to_string(),
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""trigger":"deploy-pipeline""#));
}

#[test]
fn test_trigger_step_with_label() {
    let label = Label("Deploy".to_string());
    let step = TriggerStep {
        allow_dependency_failure: None,
        async_: false,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: Some(label),
        name: None,
        skip: None,
        soft_fail: None,
        trigger: "deploy-pipeline".to_string(),
        type_: None,
    };

    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""label":"Deploy""#));
}
