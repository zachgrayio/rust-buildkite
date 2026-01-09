
use rust_buildkite::*;

#[test]
fn test_select_field() {
    let field = SelectField {
        key: Some("environment".to_string()),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&field).unwrap();
    assert!(json.contains(r#""key":"environment""#));
}
