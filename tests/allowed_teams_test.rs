mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestAllowedTeams {
    allowed_teams: AllowedTeams,
}

#[test]
fn test_allowed_teams_string() {
    let val = "string".to_string();
    let test_val = TestAllowedTeams {
        allowed_teams: AllowedTeams::String(val),
    };
    check_result(test_val, r#"{"allowed_teams":"string"}"#);
}

#[test]
fn test_allowed_teams_string_array() {
    let val = vec!["string".to_string()];
    let test_val = TestAllowedTeams {
        allowed_teams: AllowedTeams::Array(val),
    };
    check_result(test_val, r#"{"allowed_teams":["string"]}"#);
}
