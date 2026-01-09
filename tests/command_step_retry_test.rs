mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestCommandStepAutomaticRetry {
    retry: CommandStepAutomaticRetry,
}

#[test]
fn test_command_step_automatic_retry() {
    {
        let val = TestCommandStepAutomaticRetry {
            retry: CommandStepAutomaticRetry::Variant0(CommandStepAutomaticRetryVariant0::Boolean(
                true,
            )),
        };
        check_result(val, r#"{"retry":true}"#);
    }

    {
        let val = TestCommandStepAutomaticRetry {
            retry: CommandStepAutomaticRetry::Variant0(CommandStepAutomaticRetryVariant0::String(
                CommandStepAutomaticRetryVariant0String::True,
            )),
        };
        check_result(val, r#"{"retry":"true"}"#);
    }

    {
        let val = TestCommandStepAutomaticRetry {
            retry: CommandStepAutomaticRetry::Variant1(AutomaticRetry {
                limit: Some(1),
                exit_status: None,
                signal: None,
                signal_reason: None,
            }),
        };
        check_result(val, r#"{"retry":{"limit":1}}"#);
    }

    {
        let val = TestCommandStepAutomaticRetry {
            retry: CommandStepAutomaticRetry::Variant2(AutomaticRetryList(vec![AutomaticRetry {
                limit: Some(1),
                exit_status: None,
                signal: None,
                signal_reason: None,
            }])),
        };
        check_result(val, r#"{"retry":[{"limit":1}]}"#);
    }
}

#[derive(Serialize)]
struct TestCommandStepManualRetryObject {
    retry: CommandStepManualRetryObject,
}

#[test]
fn test_command_step_manual_retry_object() {
    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: CommandStepManualRetryObjectAllowed::Boolean(true),
                permit_on_passed: ::std::default::Default::default(),
                reason: None,
            },
        };
        check_result(val, r#"{"retry":{"allowed":true,"permit_on_passed":true}}"#);
    }

    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: CommandStepManualRetryObjectAllowed::String(
                    CommandStepManualRetryObjectAllowedString::True,
                ),
                permit_on_passed: ::std::default::Default::default(),
                reason: None,
            },
        };
        check_result(
            val,
            r#"{"retry":{"allowed":"true","permit_on_passed":true}}"#,
        );
    }

    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: ::std::default::Default::default(),
                permit_on_passed: CommandStepManualRetryObjectPermitOnPassed::Boolean(true),
                reason: None,
            },
        };
        check_result(val, r#"{"retry":{"allowed":true,"permit_on_passed":true}}"#);
    }

    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: ::std::default::Default::default(),
                permit_on_passed: CommandStepManualRetryObjectPermitOnPassed::String(
                    CommandStepManualRetryObjectPermitOnPassedString::True,
                ),
                reason: None,
            },
        };
        check_result(
            val,
            r#"{"retry":{"allowed":true,"permit_on_passed":"true"}}"#,
        );
    }

    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: ::std::default::Default::default(),
                permit_on_passed: ::std::default::Default::default(),
                reason: Some("reason".to_string()),
            },
        };
        check_result(
            val,
            r#"{"retry":{"allowed":true,"permit_on_passed":true,"reason":"reason"}}"#,
        );
    }

    {
        let val = TestCommandStepManualRetryObject {
            retry: CommandStepManualRetryObject {
                allowed: CommandStepManualRetryObjectAllowed::Boolean(true),
                permit_on_passed: CommandStepManualRetryObjectPermitOnPassed::String(
                    CommandStepManualRetryObjectPermitOnPassedString::False,
                ),
                reason: Some("reason".to_string()),
            },
        };
        check_result(
            val,
            r#"{"retry":{"allowed":true,"permit_on_passed":"false","reason":"reason"}}"#,
        );
    }
}

#[derive(Serialize)]
struct TestCommandStepManualRetry {
    retry: CommandStepManualRetry,
}

#[test]
fn test_command_step_manual_retry() {
    {
        let val = TestCommandStepManualRetry {
            retry: CommandStepManualRetry::Variant0(CommandStepManualRetryVariant0::Boolean(true)),
        };
        check_result(val, r#"{"retry":true}"#);
    }

    {
        let val = TestCommandStepManualRetry {
            retry: CommandStepManualRetry::Variant0(CommandStepManualRetryVariant0::String(
                CommandStepManualRetryVariant0String::True,
            )),
        };
        check_result(val, r#"{"retry":"true"}"#);
    }

    {
        let val = TestCommandStepManualRetry {
            retry: CommandStepManualRetry::Variant1(CommandStepManualRetryObject {
                allowed: CommandStepManualRetryObjectAllowed::Boolean(true),
                permit_on_passed: CommandStepManualRetryObjectPermitOnPassed::String(
                    CommandStepManualRetryObjectPermitOnPassedString::False,
                ),
                reason: Some("reason".to_string()),
            }),
        };
        check_result(
            val,
            r#"{"retry":{"allowed":true,"permit_on_passed":"false","reason":"reason"}}"#,
        );
    }
}
