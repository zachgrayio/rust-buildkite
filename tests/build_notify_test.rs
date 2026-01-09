mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestBuildNotify {
    notify: BuildNotify,
}

#[test]
fn test_notify_simple() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Simple(NotifySimple::GithubCheck)]),
    };
    check_result(val, r#"{"notify":["github_check"]}"#);
}

#[test]
fn test_notify_email() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Email(NotifyEmail {
            email: Some("email".to_string()),
            if_: None,
        })]),
    };
    check_result(val, r#"{"notify":[{"email":"email"}]}"#);
}

#[test]
fn test_notify_basecamp() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Basecamp(NotifyBasecamp {
            basecamp_campfire: Some("string".to_string()),
            if_: None,
        })]),
    };
    check_result(val, r#"{"notify":[{"basecamp_campfire":"string"}]}"#);
}

#[test]
fn test_notify_slack() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Slack(NotifySlack {
            slack: Some(NotifySlackSlack::String("#general".to_string())),
            if_: None,
        })]),
    };
    check_result(val, r##"{"notify":[{"slack":"#general"}]}"##);
}

#[test]
fn test_notify_webhook() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Webhook(NotifyWebhook {
            webhook: Some("url".to_string()),
            if_: None,
        })]),
    };
    check_result(val, r#"{"notify":[{"webhook":"url"}]}"#);
}

#[test]
fn test_notify_pagerduty() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::Pagerduty(NotifyPagerduty {
            pagerduty_change_event: Some("event".to_string()),
            if_: None,
        })]),
    };
    check_result(val, r#"{"notify":[{"pagerduty_change_event":"event"}]}"#);
}

#[test]
fn test_notify_github_commit_status() {
    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::GithubCommitStatus(
            NotifyGithubCommitStatus {
                github_commit_status: Some(NotifyGithubCommitStatusGithubCommitStatus {
                    context: Some("ctx".to_string()),
                }),
                if_: None,
            },
        )]),
    };
    check_result(
        val,
        r#"{"notify":[{"github_commit_status":{"context":"ctx"}}]}"#,
    );
}

#[test]
fn test_notify_github_check() {
    let mut github_check = serde_json::Map::new();
    github_check.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );

    let val = TestBuildNotify {
        notify: BuildNotify(vec![BuildNotifyItem::GithubCheck(NotifyGithubCheck {
            github_check,
        })]),
    };
    check_result(val, r#"{"notify":[{"github_check":{"foo":"bar"}}]}"#);
}
