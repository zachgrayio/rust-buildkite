
use rust_buildkite::*;

#[test]
fn test_skip_boolean() {
    let skip = Skip::Boolean(true);
    
    let json = serde_json::to_string(&skip).unwrap();
    assert_eq!(json, "true");
}

#[test]
fn test_skip_string() {
    let skip = Skip::String(SkipString("Skipping this step".to_string()));
    
    let json = serde_json::to_string(&skip).unwrap();
    assert_eq!(json, r#""Skipping this step""#);
}
