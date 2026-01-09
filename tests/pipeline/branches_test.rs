
use rust_buildkite::*;

#[test]
fn test_branches_string() {
    let branches = Branches::String("main".to_string());
    
    let json = serde_json::to_string(&branches).unwrap();
    assert_eq!(json, r#""main""#);
}

#[test]
fn test_branches_list() {
    let branches = Branches::List(vec!["main".to_string(), "develop".to_string()]);
    
    let json = serde_json::to_string(&branches).unwrap();
    assert!(json.contains(r#""main""#) && json.contains(r#""develop""#));
}
