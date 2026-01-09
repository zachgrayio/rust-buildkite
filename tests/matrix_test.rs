mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
struct TestMatrixSetup {
    setup: MatrixSetup,
}

#[test]
fn test_matrix_setup_element_list() {
    let value = "value".to_string();
    let list = MatrixElementList(vec![MatrixElement::String(value)]);
    let val = TestMatrixSetup {
        setup: MatrixSetup::MatrixElementList(list),
    };
    check_result(val, r#"{"setup":["value"]}"#);
}

#[test]
fn test_matrix_setup_object() {
    let element_value = "bar".to_string();
    let mut value = HashMap::new();
    value.insert(
        MatrixSetupObjectKey::try_from("foo").unwrap(),
        vec![MatrixElement::String(element_value)],
    );
    let val = TestMatrixSetup {
        setup: MatrixSetup::Object(value),
    };
    check_result(val, r#"{"setup":{"foo":["bar"]}}"#);
}

#[derive(Serialize)]
struct TestMatrixAdjustments {
    adjustments: MatrixAdjustments,
}

#[test]
fn test_matrix_adjustments_with_element_list() {
    let value = "value".to_string();
    let list = MatrixElementList(vec![MatrixElement::String(value)]);
    let val = TestMatrixAdjustments {
        adjustments: MatrixAdjustments {
            skip: None,
            soft_fail: None,
            with: MatrixAdjustmentsWith::ElementList(list),
        },
    };
    check_result(val, r#"{"adjustments":{"with":["value"]}}"#);
}

#[test]
fn test_matrix_adjustments_with_object() {
    let mut value = HashMap::new();
    value.insert("foo".to_string(), "bar".to_string());
    let val = TestMatrixAdjustments {
        adjustments: MatrixAdjustments {
            skip: None,
            soft_fail: None,
            with: MatrixAdjustmentsWith::AdjustmentsWithObject(MatrixAdjustmentsWithObject(value)),
        },
    };
    check_result(val, r#"{"adjustments":{"with":{"foo":"bar"}}}"#);
}

#[test]
fn test_matrix_adjustments_skip() {
    let skip = "skip".parse::<SkipString>().unwrap();
    let val = TestMatrixAdjustments {
        adjustments: MatrixAdjustments {
            skip: Some(Skip::String(skip)),
            soft_fail: None,
            with: MatrixAdjustmentsWith::ElementList(MatrixElementList(vec![])),
        },
    };
    check_result(val, r#"{"adjustments":{"skip":"skip","with":[]}}"#);
}

#[test]
fn test_matrix_adjustments_soft_fail() {
    let soft_fail = true;
    let val = TestMatrixAdjustments {
        adjustments: MatrixAdjustments {
            skip: None,
            soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::Boolean(soft_fail))),
            with: MatrixAdjustmentsWith::ElementList(MatrixElementList(vec![])),
        },
    };
    check_result(val, r#"{"adjustments":{"soft_fail":true,"with":[]}}"#);
}

#[test]
fn test_matrix_adjustments_all() {
    let skip = "skip".parse::<SkipString>().unwrap();
    let soft_fail = true;
    let mut with = HashMap::new();
    with.insert("foo".to_string(), "bar".to_string());
    let val = TestMatrixAdjustments {
        adjustments: MatrixAdjustments {
            skip: Some(Skip::String(skip)),
            soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::Boolean(soft_fail))),
            with: MatrixAdjustmentsWith::AdjustmentsWithObject(MatrixAdjustmentsWithObject(with)),
        },
    };
    check_result(
        val,
        r#"{"adjustments":{"skip":"skip","soft_fail":true,"with":{"foo":"bar"}}}"#,
    );
}

#[test]
fn test_matrix_object_setup() {
    let element_value = "bar".to_string();
    let mut value = HashMap::new();
    value.insert(
        MatrixSetupObjectKey::try_from("foo").unwrap(),
        vec![MatrixElement::String(element_value)],
    );
    let val = MatrixObject {
        adjustments: vec![],
        setup: MatrixSetup::Object(value),
    };
    check_result(val, r#"{"setup":{"foo":["bar"]}}"#);
}

#[test]
fn test_matrix_object_adjustments() {
    let mut value = HashMap::new();
    value.insert("foo".to_string(), "bar".to_string());
    let val = MatrixObject {
        adjustments: vec![MatrixAdjustments {
            skip: None,
            soft_fail: None,
            with: MatrixAdjustmentsWith::AdjustmentsWithObject(MatrixAdjustmentsWithObject(value)),
        }],
        setup: MatrixSetup::MatrixElementList(MatrixElementList(vec![])),
    };
    check_result(
        val,
        r#"{"adjustments":[{"with":{"foo":"bar"}}],"setup":[]}"#,
    );
}

#[test]
fn test_matrix_object_both() {
    let element_value = "bar".to_string();
    let mut setup = HashMap::new();
    setup.insert(
        MatrixSetupObjectKey::try_from("foo").unwrap(),
        vec![MatrixElement::String(element_value)],
    );
    let mut adjustments_with = HashMap::new();
    adjustments_with.insert("foo".to_string(), "bar".to_string());
    let val = MatrixObject {
        setup: MatrixSetup::Object(setup),
        adjustments: vec![MatrixAdjustments {
            skip: None,
            soft_fail: None,
            with: MatrixAdjustmentsWith::AdjustmentsWithObject(MatrixAdjustmentsWithObject(
                adjustments_with,
            )),
        }],
    };
    check_result(
        val,
        r#"{"adjustments":[{"with":{"foo":"bar"}}],"setup":{"foo":["bar"]}}"#,
    );
}

#[derive(Serialize)]
struct TestMatrix {
    matrix: Matrix,
}

#[test]
fn test_matrix_element_list() {
    let element = "value".to_string();
    let list = MatrixElementList(vec![MatrixElement::String(element)]);
    let val = TestMatrix {
        matrix: Matrix::ElementList(list),
    };
    check_result(val, r#"{"matrix":["value"]}"#);
}

#[test]
fn test_matrix_object() {
    let element_value = "bar".to_string();
    let mut value = HashMap::new();
    value.insert(
        MatrixSetupObjectKey::try_from("foo").unwrap(),
        vec![MatrixElement::String(element_value)],
    );
    let val = TestMatrix {
        matrix: Matrix::Object(MatrixObject {
            adjustments: vec![],
            setup: MatrixSetup::Object(value),
        }),
    };
    check_result(val, r#"{"matrix":{"setup":{"foo":["bar"]}}}"#);
}
