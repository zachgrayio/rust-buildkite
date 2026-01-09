mod pipeline_test_common;

use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestTextFieldRequired {
    required: TextFieldRequired,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_field_required_string() {
        let val = TestTextFieldRequired {
            required: TextFieldRequired::String(TextFieldRequiredString::True),
        };
        check_result(val, r#"{"required":"true"}"#);
    }

    #[test]
    fn test_text_field_required_bool() {
        let val = TestTextFieldRequired {
            required: TextFieldRequired::Boolean(true),
        };
        check_result(val, r#"{"required":true}"#);
    }

    #[test]
    fn test_text_field_default() {
        let default_val = "default".to_string();
        let val = TextField {
            default: Some(default_val),
            format: None,
            hint: None,
            key: "key".parse().unwrap(),
            required: ::std::default::Default::default(),
            text: None,
        };
        check_result(val, r#"{"default":"default","key":"key","required":true}"#);
    }

    #[test]
    fn test_text_field_format() {
        let format = "format".to_string();
        let val = TextField {
            default: None,
            format: Some(format),
            hint: None,
            key: "key".parse().unwrap(),
            required: ::std::default::Default::default(),
            text: None,
        };
        check_result(val, r#"{"format":"format","key":"key","required":true}"#);
    }

    #[test]
    fn test_text_field_hint() {
        let hint = "hint".to_string();
        let val = TextField {
            default: None,
            format: None,
            hint: Some(hint),
            key: "key".parse().unwrap(),
            required: ::std::default::Default::default(),
            text: None,
        };
        check_result(val, r#"{"hint":"hint","key":"key","required":true}"#);
    }

    #[test]
    fn test_text_field_key() {
        let key = "key".to_string();
        let val = TextField {
            default: None,
            format: None,
            hint: None,
            key: key.parse().unwrap(),
            required: ::std::default::Default::default(),
            text: None,
        };
        check_result(val, r#"{"key":"key","required":true}"#);
    }

    #[test]
    fn test_text_field_required() {
        let required = TextFieldRequiredString::True;
        let val = TextField {
            default: None,
            format: None,
            hint: None,
            key: "key".parse().unwrap(),
            required: TextFieldRequired::String(required),
            text: None,
        };
        check_result(val, r#"{"key":"key","required":"true"}"#);
    }

    #[test]
    fn test_text_field_text() {
        let text = "text".to_string();
        let val = TextField {
            default: None,
            format: None,
            hint: None,
            key: "key".parse().unwrap(),
            required: ::std::default::Default::default(),
            text: Some(text),
        };
        check_result(val, r#"{"key":"key","required":true,"text":"text"}"#);
    }

    #[test]
    fn test_text_field_all() {
        let default_val = "default".to_string();
        let format = "format".to_string();
        let hint = "hint".to_string();
        let key = "key".to_string();
        let required = TextFieldRequiredString::True;
        let text = "text".to_string();
        let val = TextField {
            default: Some(default_val),
            format: Some(format),
            hint: Some(hint),
            key: key.parse().unwrap(),
            required: TextFieldRequired::String(required),
            text: Some(text),
        };
        check_result(
            val,
            r#"{"default":"default","format":"format","hint":"hint","key":"key","required":"true","text":"text"}"#,
        );
    }
}
