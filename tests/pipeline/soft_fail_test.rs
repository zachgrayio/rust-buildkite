use rust_buildkite::*;

#[test]
fn test_soft_fail_boolean() {
    let soft_fail = SoftFail::Variant0(SoftFailVariant0::Boolean(true));

    let json = serde_json::to_string(&soft_fail).unwrap();
    assert_eq!(json, "true");
}
