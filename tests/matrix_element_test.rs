mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestMatrixElement {
    element: MatrixElement,
}

#[test]
fn test_matrix_element_string() {
    let val = TestMatrixElement {
        element: MatrixElement::String("string".to_string()),
    };
    check_result(val, r#"{"element":"string"}"#);
}

#[test]
fn test_matrix_element_integer() {
    let val = TestMatrixElement {
        element: MatrixElement::Integer(1),
    };
    check_result(val, r#"{"element":1}"#);
}

#[test]
fn test_matrix_element_boolean() {
    let val = TestMatrixElement {
        element: MatrixElement::Boolean(true),
    };
    check_result(val, r#"{"element":true}"#);
}
