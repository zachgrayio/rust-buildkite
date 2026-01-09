
use rust_buildkite::*;

#[test]
fn test_cache_string() {
    let cache = Cache::String("node_modules".to_string());
    
    let json = serde_json::to_string(&cache).unwrap();
    assert_eq!(json, r#""node_modules""#);
}
