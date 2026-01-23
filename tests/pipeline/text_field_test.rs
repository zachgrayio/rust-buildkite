use rust_buildkite::*;
use std::str::FromStr;

#[test]
fn test_text_field() {
    let field = TextField {
        default: None,
        format: None,
        hint: Some("Enter your message".to_string()),
        key: TextFieldKey::from_str("message").unwrap(),
        required: true,
        text: None,
    };

    let json = serde_json::to_string(&field).unwrap();
    assert!(json.contains(r#""key":"message""#));
    assert!(json.contains(r#""hint":"Enter your message""#));
}
