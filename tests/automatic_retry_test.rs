mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestAutomaticRetry {
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_retry: Option<AutomaticRetry>,
}

#[derive(Serialize)]
struct TestAutomaticRetryExitStatus {
    status: AutomaticRetryExitStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automatic_retry_exit_status_enum() {
        let test_val = TestAutomaticRetryExitStatus {
            status: AutomaticRetryExitStatus::String(AutomaticRetryExitStatusString::X),
        };

        check_result(test_val, r#"{"status":"*"}"#);
    }

    #[test]
    fn test_automatic_retry_exit_status_int() {
        let test_val = TestAutomaticRetryExitStatus {
            status: AutomaticRetryExitStatus::Integer(1),
        };

        check_result(test_val, r#"{"status":1}"#);
    }

    #[test]
    fn test_automatic_retry_exit_status_int_array() {
        let test_val = TestAutomaticRetryExitStatus {
            status: AutomaticRetryExitStatus::Array(vec![1, 2]),
        };

        check_result(test_val, r#"{"status":[1,2]}"#);
    }

    #[test]
    fn test_automatic_retry_exit_status() {
        let test_val = TestAutomaticRetry {
            automatic_retry: Some(AutomaticRetry {
                exit_status: Some(AutomaticRetryExitStatus::Integer(1)),
                ..::std::default::Default::default()
            }),
        };

        check_result(test_val, r#"{"automatic_retry":{"exit_status":1}}"#);
    }

    #[test]
    fn test_automatic_retry_limit() {
        let test_val = TestAutomaticRetry {
            automatic_retry: Some(AutomaticRetry {
                limit: Some(1),
                ..::std::default::Default::default()
            }),
        };

        check_result(test_val, r#"{"automatic_retry":{"limit":1}}"#);
    }

    #[test]
    fn test_automatic_retry_signal() {
        let test_val = TestAutomaticRetry {
            automatic_retry: Some(AutomaticRetry {
                signal: Some("string".to_string()),
                ..::std::default::Default::default()
            }),
        };

        check_result(test_val, r#"{"automatic_retry":{"signal":"string"}}"#);
    }

    #[test]
    fn test_automatic_retry_signal_reason() {
        let test_val = TestAutomaticRetry {
            automatic_retry: Some(AutomaticRetry {
                signal_reason: Some(AutomaticRetrySignalReason::None),
                ..::std::default::Default::default()
            }),
        };

        check_result(test_val, r#"{"automatic_retry":{"signal_reason":"none"}}"#);
    }

    #[test]
    fn test_automatic_retry_all() {
        let test_val = TestAutomaticRetry {
            automatic_retry: Some(AutomaticRetry {
                exit_status: Some(AutomaticRetryExitStatus::Integer(1)),
                limit: Some(2),
                signal: Some("string".to_string()),
                signal_reason: Some(AutomaticRetrySignalReason::None),
            }),
        };

        check_result(
            test_val,
            r#"{"automatic_retry":{"exit_status":1,"limit":2,"signal":"string","signal_reason":"none"}}"#,
        );
    }
}
