use rust_buildkite::*;

#[test]
fn test_fields_list() {
    let fields = Fields(vec![]);

    let json = serde_json::to_string(&fields).unwrap();
    assert!(json.contains("[]"));
}
