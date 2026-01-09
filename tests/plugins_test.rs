mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestPluginsList {
    plugins: PluginsList,
}

#[test]
fn test_plugins_list_string() {
    let value = "string".to_string();
    let val = TestPluginsList {
        plugins: PluginsList(vec![PluginsListItem::String(value)]),
    };
    check_result(val, r#"{"plugins":["string"]}"#);
}

#[test]
fn test_plugins_list_object() {
    let mut plugin_obj = serde_json::Map::new();
    let mut name_config = serde_json::Map::new();
    name_config.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    plugin_obj.insert("name".to_string(), serde_json::Value::Object(name_config));

    let val = TestPluginsList {
        plugins: PluginsList(vec![PluginsListItem::Object(plugin_obj)]),
    };
    check_result(val, r#"{"plugins":[{"name":{"foo":"bar"}}]}"#);
}

#[derive(Serialize)]
struct TestPlugins {
    plugins: Plugins,
}

#[test]
fn test_plugins_list() {
    let plugin = "docker".to_string();
    let list = PluginsList(vec![PluginsListItem::String(plugin)]);
    let val = TestPlugins {
        plugins: Plugins::List(list),
    };
    check_result(val, r#"{"plugins":["docker"]}"#);
}

#[test]
fn test_plugins_object() {
    let mut plugin_obj = serde_json::Map::new();
    let mut name_config = serde_json::Map::new();
    name_config.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    plugin_obj.insert("name".to_string(), serde_json::Value::Object(name_config));

    let val = TestPlugins {
        plugins: Plugins::Object(PluginsObject(plugin_obj)),
    };
    check_result(val, r#"{"plugins":{"name":{"foo":"bar"}}}"#);
}
