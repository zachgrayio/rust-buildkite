
use rust_buildkite::*;

#[test]
fn test_input_step_with_input() {
    let input = InputStepInput("Deploy to production?".to_string());
    let step = InputStep {
        input: Some(input),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""input":"Deploy to production?""#));
}

#[test]
fn test_input_step_with_key() {
    let key = Key("deploy-gate".to_string());
    let step = InputStep {
        key: Some(key),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"deploy-gate""#));
}
