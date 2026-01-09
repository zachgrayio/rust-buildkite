mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestFields {
    fields: Vec<FieldsItem>,
}

#[test]
fn test_text_field() {
    let val = TestFields {
        fields: vec![
            FieldsItem::TextField(
                TextField::builder()
                    .key("key".parse::<TextFieldKey>().unwrap())
                    .text(Some("textFieldOne".to_string()))
                    .try_into()
                    .unwrap(),
            ),
            FieldsItem::TextField(
                TextField::builder()
                    .key("key".parse::<TextFieldKey>().unwrap())
                    .text(Some("textFieldTwo".to_string()))
                    .try_into()
                    .unwrap(),
            ),
        ],
    };
    check_result(
        val,
        r#"{"fields":[{"key":"key","required":true,"text":"textFieldOne"},{"key":"key","required":true,"text":"textFieldTwo"}]}"#,
    );
}

#[test]
fn test_select_field() {
    let val = TestFields {
        fields: vec![
            FieldsItem::SelectField(
                SelectField::builder()
                    .key("key".parse::<SelectFieldKey>().unwrap())
                    .options(vec![])
                    .select(Some("selectFieldOne".to_string()))
                    .try_into()
                    .unwrap(),
            ),
            FieldsItem::SelectField(
                SelectField::builder()
                    .key("key".parse::<SelectFieldKey>().unwrap())
                    .options(vec![])
                    .select(Some("selectFieldTwo".to_string()))
                    .try_into()
                    .unwrap(),
            ),
        ],
    };
    check_result(
        val,
        r#"{"fields":[{"key":"key","multiple":false,"options":[],"required":true,"select":"selectFieldOne"},{"key":"key","multiple":false,"options":[],"required":true,"select":"selectFieldTwo"}]}"#,
    );
}
