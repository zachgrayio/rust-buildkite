use rust_buildkite::*;

#[test]
fn test_build_notify_list() {
    let notify = BuildNotify(vec![]);

    let json = serde_json::to_string(&notify).unwrap();
    assert!(json.contains("[]"));
}
