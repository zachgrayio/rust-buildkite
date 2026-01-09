mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestSelectFieldDefault {
    default: SelectFieldDefault,
}

#[derive(Serialize)]
struct TestSelectFieldMultiple {
    multiple: SelectFieldMultiple,
}

#[derive(Serialize)]
struct TestSelectFieldRequired {
    required: SelectFieldRequired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_field_default_string() {
        let val = TestSelectFieldDefault {
            default: SelectFieldDefault::String("string".to_string()),
        };
        check_result(val, r#"{"default":"string"}"#);
    }

    #[test]
    fn test_select_field_default_string_array() {
        let val = TestSelectFieldDefault {
            default: SelectFieldDefault::Array(vec!["one".to_string(), "two".to_string()]),
        };
        check_result(val, r#"{"default":["one","two"]}"#);
    }

    #[test]
    fn test_select_field_multiple_string() {
        let val = TestSelectFieldMultiple {
            multiple: SelectFieldMultiple::String(SelectFieldMultipleString::True),
        };
        check_result(val, r#"{"multiple":"true"}"#);
    }

    #[test]
    fn test_select_field_multiple_bool() {
        let val = TestSelectFieldMultiple {
            multiple: SelectFieldMultiple::Boolean(true),
        };
        check_result(val, r#"{"multiple":true}"#);
    }

    #[test]
    fn test_select_field_required_string() {
        let val = TestSelectFieldRequired {
            required: SelectFieldRequired::String(SelectFieldRequiredString::True),
        };
        check_result(val, r#"{"required":"true"}"#);
    }

    #[test]
    fn test_select_field_required_bool() {
        let val = TestSelectFieldRequired {
            required: SelectFieldRequired::Boolean(true),
        };
        check_result(val, r#"{"required":true}"#);
    }

    #[test]
    fn test_select_field_hint() {
        let hint = "hint".to_string();
        let val = SelectField {
            default: None,
            hint: Some(hint),
            key: "key".parse().unwrap(),
            multiple: ::std::default::Default::default(),
            options: vec![],
            required: ::std::default::Default::default(),
            select: None,
        };
        check_result(
            val,
            r#"{"hint":"hint","key":"key","multiple":false,"options":[],"required":true}"#,
        );
    }

    #[test]
    fn test_select_field_key() {
        let key = "key".to_string();
        let val = SelectField {
            default: None,
            hint: None,
            key: key.parse().unwrap(),
            multiple: ::std::default::Default::default(),
            options: vec![],
            required: ::std::default::Default::default(),
            select: None,
        };
        check_result(
            val,
            r#"{"key":"key","multiple":false,"options":[],"required":true}"#,
        );
    }

    #[test]
    fn test_select_field_select() {
        let select_val = "select".to_string();
        let val = SelectField {
            default: None,
            hint: None,
            key: "key".parse().unwrap(),
            multiple: ::std::default::Default::default(),
            options: vec![],
            required: ::std::default::Default::default(),
            select: Some(select_val),
        };
        check_result(
            val,
            r#"{"key":"key","multiple":false,"options":[],"required":true,"select":"select"}"#,
        );
    }

    #[test]
    fn test_select_field_all() {
        let default_val = "value".to_string();
        let hint = "hint".to_string();
        let key = "key".to_string();
        let select_val = "select".to_string();
        let multiple = false;
        let required = false;
        let option_val = "optionValue".to_string();
        let options = vec![SelectFieldOption {
            hint: None,
            label: "".to_string(),
            required: ::std::default::Default::default(),
            value: option_val,
        }];

        let val = SelectField {
            default: Some(SelectFieldDefault::String(default_val)),
            hint: Some(hint),
            key: key.parse().unwrap(),
            multiple: SelectFieldMultiple::Boolean(multiple),
            options,
            required: SelectFieldRequired::Boolean(required),
            select: Some(select_val),
        };
        check_result(
            val,
            r#"{"default":"value","hint":"hint","key":"key","multiple":false,"options":[{"label":"","required":true,"value":"optionValue"}],"required":false,"select":"select"}"#,
        );
    }
}
