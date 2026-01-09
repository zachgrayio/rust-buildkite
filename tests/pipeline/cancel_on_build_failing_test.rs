
use rust_buildkite::*;

#[test]
fn test_cancel_on_build_failing_boolean() {
    let cancel = CancelOnBuildFailing::Boolean(true);
    
    let json = serde_json::to_string(&cancel).unwrap();
    assert_eq!(json, "true");
}
