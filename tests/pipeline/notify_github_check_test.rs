use rust_buildkite::*;

#[test]
fn test_notify_github_check() {
    let mut github_check_map = serde_json::Map::new();
    github_check_map.insert(
        "context".to_string(),
        serde_json::Value::String("my-check".to_string()),
    );

    let notify = NotifyGithubCheck {
        github_check: github_check_map,
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""github_check""#) && json.contains(r#""context""#));
}
