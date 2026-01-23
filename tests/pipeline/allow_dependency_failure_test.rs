use rust_buildkite::*;

#[test]
fn test_allow_dependency_failure_boolean() {
    let allow = AllowDependencyFailure(true);

    let json = serde_json::to_string(&allow).unwrap();
    assert_eq!(json, "true");
}

#[test]
fn test_allow_dependency_failure_false() {
    let allow = AllowDependencyFailure(false);

    let json = serde_json::to_string(&allow).unwrap();
    assert_eq!(json, "false");
}
