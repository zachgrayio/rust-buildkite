mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestSelectFieldOptionRequired {
    required: SelectFieldOptionRequired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_field_option_required_string() {
        let val = TestSelectFieldOptionRequired {
            required: SelectFieldOptionRequired::String(SelectFieldOptionRequiredString::True),
        };
        check_result(val, r#"{"required":"true"}"#);
    }

    #[test]
    fn test_select_field_option_required_bool() {
        let val = TestSelectFieldOptionRequired {
            required: SelectFieldOptionRequired::Boolean(true),
        };
        check_result(val, r#"{"required":true}"#);
    }

    #[test]
    fn test_select_field_option_hint() {
        let hint = "hint".to_string();
        let val = SelectFieldOption {
            hint: Some(hint),
            label: "".to_string(),
            required: ::std::default::Default::default(),
            value: "".to_string(),
        };
        check_result(
            val,
            r#"{"hint":"hint","label":"","required":true,"value":""}"#,
        );
    }

    #[test]
    fn test_select_field_option_label() {
        let label = "label".to_string();
        let val = SelectFieldOption {
            hint: None,
            label,
            required: ::std::default::Default::default(),
            value: "".to_string(),
        };
        check_result(val, r#"{"label":"label","required":true,"value":""}"#);
    }

    #[test]
    fn test_select_field_option_required() {
        let required = true;
        let val = SelectFieldOption {
            hint: None,
            label: "".to_string(),
            required: SelectFieldOptionRequired::Boolean(required),
            value: "".to_string(),
        };
        check_result(val, r#"{"label":"","required":true,"value":""}"#);
    }

    #[test]
    fn test_select_field_option_value() {
        let value = "value".to_string();
        let val = SelectFieldOption {
            hint: None,
            label: "".to_string(),
            required: ::std::default::Default::default(),
            value,
        };
        check_result(val, r#"{"label":"","required":true,"value":"value"}"#);
    }

    #[test]
    fn test_select_field_option_all() {
        let hint = "hint".to_string();
        let label = "label".to_string();
        let required = true;
        let value = "value".to_string();
        let val = SelectFieldOption {
            hint: Some(hint),
            label,
            required: SelectFieldOptionRequired::Boolean(required),
            value,
        };
        check_result(
            val,
            r#"{"hint":"hint","label":"label","required":true,"value":"value"}"#,
        );
    }
}
