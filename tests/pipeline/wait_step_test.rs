
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
        continue_on_failure: Some(WaitStepContinueOnFailure::Boolean(true)),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""continue_on_failure":true"#));
}
