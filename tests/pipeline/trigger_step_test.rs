
use rust_buildkite::*;

#[test]
fn test_trigger_step_with_trigger() {
    let step = TriggerStep {
        trigger: Some("deploy-pipeline".to_string()),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""trigger":"deploy-pipeline""#));
}

#[test]
fn test_trigger_step_with_label() {
    let label = Label("Deploy".to_string());
    let step = TriggerStep {
        label: Some(label),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""label":"Deploy""#));
}
