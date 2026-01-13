use rust_buildkite::*;
use std::str::FromStr;

#[test]
fn test_select_field() {
    let field = SelectField {
        default: None,
        hint: None,
        key: SelectFieldKey::from_str("environment").unwrap(),
        multiple: SelectFieldMultiple::default(),
        options: vec![],
        required: SelectFieldRequired::default(),
        select: None,
    };

    let json = serde_json::to_string(&field).unwrap();
    assert!(json.contains(r#""key":"environment""#));
}
