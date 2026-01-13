use rust_buildkite::*;

#[test]
fn test_notify_github_commit_status() {
    let notify = NotifyGithubCommitStatus {
        github_commit_status: Some(NotifyGithubCommitStatusGithubCommitStatus {
            context: Some("ci/buildkite".to_string()),
        }),
        if_: None,
    };

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains(r#""context":"ci/buildkite""#));
}
