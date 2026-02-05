/// Set up test environment before any tests run.
/// This skips runtime validation since tests don't have real workspaces.
#[ctor::ctor]
fn init_test_env() {
    unsafe {
        std::env::set_var("BUILDKITE_SKIP_RUNTIME_VALIDATION", "1");
    }
}

pub fn check_result<T: serde::Serialize>(value: T, expected: &str) {
    let result = serde_json::to_string(&value).unwrap();
    assert_eq!(
        expected, result,
        "\nExpected: {}\nGot:      {}",
        expected, result
    );
}
