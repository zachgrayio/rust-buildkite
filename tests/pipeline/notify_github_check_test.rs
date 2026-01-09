
use rust_buildkite::*;

#[test]
fn test_notify_github_check() {
    let notify = NotifyGithubCheck {
        context: Some("my-check".to_string()),
        github_check: None,
    };
    
    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""context":"my-check""#));
}
