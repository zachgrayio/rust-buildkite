
use rust_buildkite::*;

#[test]
fn test_automatic_retry_with_limit() {
    let retry = AutomaticRetry {
        exit_status: None,
        limit: Some(3),
        signal_reason: None,
    };
    
    let json = serde_json::to_string(&retry).unwrap();
    assert!(json.contains(r#""limit":3"#));
}

#[test]
fn test_automatic_retry_with_exit_status() {
    let retry = AutomaticRetry {
        exit_status: Some(AutomaticRetryExitStatus::Number(255)),
        limit: None,
        signal_reason: None,
    };
    
    let json = serde_json::to_string(&retry).unwrap();
    assert!(json.contains(r#""exit_status":255"#));
}
