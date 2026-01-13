//! Compile-fail tests for the pipeline! and cmd! macros
//!
//! These tests use trybuild to verify that certain invalid usages
//! produce the expected compile errors.

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
