use rust_buildkite::*;

#[test]
fn test_if_changed_string() {
    let if_changed = IfChanged::String("src/**".to_string());

    let json = serde_json::to_string(&if_changed).unwrap();
    assert_eq!(json, r#""src/**""#);
}
