
use rust_buildkite::*;

#[test]
fn test_group_step_with_group() {
    let group = GroupStepGroup(Some("Test Group".to_string()));
    let step = GroupStep {
        group: Some(group),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""group":"Test Group""#));
}

#[test]
fn test_group_step_with_key() {
    let key = Key("test-group".to_string());
    let step = GroupStep {
        key: Some(key),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"test-group""#));
}
