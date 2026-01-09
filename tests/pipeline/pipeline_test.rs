
use rust_buildkite::*;

#[test]
fn test_pipeline_with_steps() {
    let pipeline = JsonSchemaForBuildkitePipelineConfigurationFiles {
        steps: Some(PipelineSteps(vec![])),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&pipeline).unwrap();
    assert!(json.contains(r#""steps":[]"#));
}

#[test]
fn test_pipeline_with_env() {
    let mut env_map = serde_json::Map::new();
    env_map.insert("KEY".to_string(), serde_json::Value::String("value".to_string()));
    
    let pipeline = JsonSchemaForBuildkitePipelineConfigurationFiles {
        env: Some(Env(env_map)),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&pipeline).unwrap();
    assert!(json.contains(r#""env""#) && json.contains(r#""KEY""#));
}
