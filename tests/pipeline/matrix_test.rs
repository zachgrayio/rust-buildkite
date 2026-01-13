use rust_buildkite::*;

#[test]
fn test_matrix_list() {
    let matrix = Matrix::ElementList(MatrixElementList(vec![MatrixElement::String(
        "value1".to_string(),
    )]));

    let json = serde_json::to_string(&matrix).unwrap();
    assert!(json.contains(r#""value1""#));
}
