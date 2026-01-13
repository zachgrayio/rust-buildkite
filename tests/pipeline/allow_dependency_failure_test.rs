use rust_buildkite::*;

#[test]
fn test_allow_dependency_failure_boolean() {
    let allow = AllowDependencyFailure::Boolean(true);

    let json = serde_json::to_string(&allow).unwrap();
    assert_eq!(json, "true");
}

#[test]
fn test_allow_dependency_failure_string() {
    let allow = AllowDependencyFailure::String(AllowDependencyFailureString::True);

    let json = serde_json::to_string(&allow).unwrap();
    assert_eq!(json, r#""true""#);
}
