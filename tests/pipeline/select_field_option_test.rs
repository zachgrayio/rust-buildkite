
use rust_buildkite::*;

#[test]
fn test_select_field_option() {
    let option = SelectFieldOption {
        label: Some("Production".to_string()),
        value: Some("prod".to_string()),
    };
    
    let json = serde_json::to_string(&option).unwrap();
    assert!(json.contains(r#""label":"Production""#));
    assert!(json.contains(r#""value":"prod""#));
}
