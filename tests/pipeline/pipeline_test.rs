use rust_buildkite::*;

#[test]
fn test_pipeline_with_steps() {
    let pipeline = JsonSchemaForBuildkitePipelineConfigurationFiles {
        agents: None,
        env: None,
        image: None,
        notify: None,
        priority: None,
        secrets: None,
        steps: PipelineSteps(vec![]),
    };

    let json = serde_json::to_string(&pipeline).unwrap();
    assert!(json.contains(r#""steps":[]"#));
}

#[test]
fn test_pipeline_with_env() {
    let mut env_map = serde_json::Map::new();
    env_map.insert(
        "KEY".to_string(),
        serde_json::Value::String("value".to_string()),
    );

    let pipeline = JsonSchemaForBuildkitePipelineConfigurationFiles {
        agents: None,
        env: Some(Env(env_map)),
        image: None,
        notify: None,
        priority: None,
        secrets: None,
        steps: PipelineSteps(vec![]),
    };

    let json = serde_json::to_string(&pipeline).unwrap();
    assert!(json.contains(r#""env""#) && json.contains(r#""KEY""#));
}
