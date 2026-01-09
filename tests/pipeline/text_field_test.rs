
use rust_buildkite::*;

#[test]
fn test_text_field() {
    let field = TextField {
        key: Some("message".to_string()),
        hint: Some("Enter your message".to_string()),
        ..Default::default()
    };
    
    let json = serde_json::to_string(&field).unwrap();
    assert!(json.contains(r#""key":"message""#));
    assert!(json.contains(r#""hint":"Enter your message""#));
}
