mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;

#[test]
fn test_notify_github_check() {
    let mut github_check = serde_json::Map::new();
    github_check.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );

    let val = NotifyGithubCheck { github_check };
    check_result(val, r#"{"github_check":{"foo":"bar"}}"#);
}
