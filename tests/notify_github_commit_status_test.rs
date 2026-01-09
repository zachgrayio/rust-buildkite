mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;

#[test]
fn test_notify_github_commit_status_github_commit_status() {
    let val = NotifyGithubCommitStatusGithubCommitStatus {
        context: Some("name".to_string()),
    };
    check_result(val, r#"{"context":"name"}"#);
}

#[test]
fn test_notify_github_commit_status_with_github_commit_status() {
    let val = NotifyGithubCommitStatus {
        github_commit_status: Some(NotifyGithubCommitStatusGithubCommitStatus {
            context: Some("name".to_string()),
        }),
        if_: None,
    };
    check_result(val, r#"{"github_commit_status":{"context":"name"}}"#);
}

#[test]
fn test_notify_github_commit_status_with_if() {
    let val = NotifyGithubCommitStatus {
        github_commit_status: None,
        if_: Some(If("string".to_string())),
    };
    check_result(val, r#"{"if":"string"}"#);
}

#[test]
fn test_notify_github_commit_status_all() {
    let val = NotifyGithubCommitStatus {
        github_commit_status: Some(NotifyGithubCommitStatusGithubCommitStatus {
            context: Some("name".to_string()),
        }),
        if_: Some(If("string".to_string())),
    };
    check_result(
        val,
        r#"{"github_commit_status":{"context":"name"},"if":"string"}"#,
    );
}
