
use rust_buildkite::*;

#[test]
fn test_depends_on_string() {
    let depends = DependsOn::String("step1".to_string());
    
    let json = serde_json::to_string(&depends).unwrap();
    assert_eq!(json, r#""step1""#);
}

#[test]
fn test_depends_on_list() {
    let depends = DependsOn::List(DependsOnList(vec![
        DependsOnListItem::String("step1".to_string()),
    ]));
    
    let json = serde_json::to_string(&depends).unwrap();
    assert!(json.contains(r#""step1""#));
}
