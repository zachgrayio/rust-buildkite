//! Compile-fail tests for the pipeline! and cmd! macros
//!
//! These tests use trybuild to verify that certain invalid usages
//! produce the expected compile errors.

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();

    t.compile_fail("tests/ui/cmd_not_in_allowlist.rs");
    t.compile_fail("tests/ui/missing_absolute_path.rs");
    t.compile_fail("tests/ui/missing_relative_path.rs");
    t.compile_fail("tests/ui/raw_string_rejected.rs");
    t.compile_fail("tests/ui/undefined_env_var.rs");
    t.compile_fail("tests/ui/undefined_variable.rs");

    #[cfg(feature = "bazel")]
    {
        t.compile_fail("tests/ui/bazel_empty_command.rs");
        t.compile_fail("tests/ui/bazel_invalid_verb.rs");
        t.compile_fail("tests/ui/bazel_invalid_target_pattern.rs");
        t.pass("tests/ui/bazel_comptime_const.rs");
        t.pass("tests/ui/bazel_runtime_skips_validation.rs");
    }
}
