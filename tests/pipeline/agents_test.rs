
use rust_buildkite::*;

#[test]
fn test_agents_list() {
    let agents = Agents::List(AgentsList(vec!["queue=default".to_string()]));
    
    let json = serde_json::to_string(&agents).unwrap();
    assert!(json.contains(r#"["queue=default"]"#));
}

#[test]
fn test_agents_object() {
    let mut map = serde_json::Map::new();
    map.insert("queue".to_string(), serde_json::Value::String("deploy".to_string()));
    let agents = Agents::Object(AgentsObject(map));
    
    let json = serde_json::to_string(&agents).unwrap();
    assert!(json.contains(r#""queue":"deploy""#));
}
