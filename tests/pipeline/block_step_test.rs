
use rust_buildkite::*;

#[test]
fn test_block_step_with_block() {
    let block = BlockStepBlock("Release Gate".to_string());
    let step = BlockStep {
        block: Some(block),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""block":"Release Gate""#));
}

#[test]
fn test_block_step_with_key() {
    let key = Key("release-gate".to_string());
    let step = BlockStep {
        key: Some(key),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""key":"release-gate""#));
}

#[test]
fn test_block_step_with_blocked_state() {
    let step = BlockStep {
        blocked_state: Some(BlockStepBlockedState::Passed),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&step).unwrap();
    assert!(json.contains(r#""blocked_state":"passed""#));
}
