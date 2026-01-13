use rust_buildkite::*;

#[test]
fn test_notify_email() {
    let notify = NotifyEmail {
        email: Some("dev@example.com".to_string()),
        if_: None,
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""email":"dev@example.com""#));
}
