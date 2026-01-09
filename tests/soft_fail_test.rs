mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestSoftFail {
    soft_fail: SoftFail,
}

#[test]
fn test_soft_fail_enum_bool() {
    let value = true;
    let val = TestSoftFail {
        soft_fail: SoftFail::Variant0(SoftFailVariant0::Boolean(value)),
    };
    check_result(val, r#"{"soft_fail":true}"#);
}

#[test]
fn test_soft_fail_enum_string() {
    let value = SoftFailVariant0String::True;
    let val = TestSoftFail {
        soft_fail: SoftFail::Variant0(SoftFailVariant0::String(value)),
    };
    check_result(val, r#"{"soft_fail":"true"}"#);
}

#[test]
fn test_soft_fail_list() {
    let exit_status = 1i64;
    let list = SoftFailList(vec![SoftFailObject {
        exit_status: Some(SoftFailObjectExitStatus::Integer(exit_status)),
    }]);
    let val = TestSoftFail {
        soft_fail: SoftFail::Variant1(list),
    };
    check_result(val, r#"{"soft_fail":[{"exit_status":1}]}"#);
}
