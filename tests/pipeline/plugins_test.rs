use rust_buildkite::*;

#[test]
fn test_plugins_list() {
    let mut plugin_obj = serde_json::Map::new();
    plugin_obj.insert("docker".to_string(), serde_json::json!({"image": "node"}));

    let plugins = Plugins::List(PluginsList(vec![PluginsListItem::Object(plugin_obj)]));

    let json = serde_json::to_string(&plugins).unwrap();
    assert!(json.contains(r#""docker""#));
}
