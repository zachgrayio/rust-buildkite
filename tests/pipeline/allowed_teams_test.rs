
use rust_buildkite::*;

#[test]
fn test_allowed_teams() {
    let teams = AllowedTeams::List(vec!["team1".to_string(), "team2".to_string()]);
    
    let json = serde_json::to_string(&teams).unwrap();
    assert!(json.contains(r#""team1""#) && json.contains(r#""team2""#));
}
