
use rust_buildkite::*;

#[test]
fn test_notify_github_commit_status() {
    let notify = NotifyGithubCommitStatus {
        context: Some("ci/buildkite".to_string()),
        github_commit_status: None,
    };
    
    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""context":"ci/buildkite""#));
}
