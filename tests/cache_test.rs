mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;
use serde::Serialize;

#[derive(Serialize)]
struct TestCache {
    cache: Cache,
}

#[test]
fn test_cache_object_empty() {
    let test_val = Cache::Object {
        paths: ::std::default::Default::default(),
        size: None,
        name: None,
    };
    check_result(test_val, r#"{"paths":[]}"#);
}

#[test]
fn test_cache_object_paths() {
    let test_val = Cache::Object {
        paths: vec!["one".to_string(), "two".to_string()],
        size: None,
        name: None,
    };
    check_result(test_val, r#"{"paths":["one","two"]}"#);
}

#[test]
fn test_cache_object_size() {
    let test_val = Cache::Object {
        paths: ::std::default::Default::default(),
        size: Some(CacheObjectSize::try_from("5g").unwrap()),
        name: None,
    };
    check_result(test_val, r#"{"paths":[],"size":"5g"}"#);
}

#[test]
fn test_cache_object_name() {
    let test_val = Cache::Object {
        paths: ::std::default::Default::default(),
        size: None,
        name: Some("name".to_string()),
    };
    check_result(test_val, r#"{"name":"name","paths":[]}"#);
}

#[test]
fn test_cache_object_all() {
    let test_val = Cache::Object {
        paths: vec!["one".to_string(), "two".to_string()],
        size: Some(CacheObjectSize::try_from("5g").unwrap()),
        name: Some("name".to_string()),
    };
    check_result(
        test_val,
        r#"{"name":"name","paths":["one","two"],"size":"5g"}"#,
    );
}

#[test]
fn test_cache_string() {
    let val = TestCache {
        cache: Cache::String("string".to_string()),
    };
    check_result(val, r#"{"cache":"string"}"#);
}

#[test]
fn test_cache_string_array() {
    let val = TestCache {
        cache: Cache::Array(vec!["one".to_string(), "two".to_string()]),
    };
    check_result(val, r#"{"cache":["one","two"]}"#);
}

#[test]
fn test_cache_object() {
    let val = TestCache {
        cache: Cache::Object {
            paths: ::std::default::Default::default(),
            size: Some(CacheObjectSize::try_from("5g").unwrap()),
            name: None,
        },
    };
    check_result(val, r#"{"cache":{"paths":[],"size":"5g"}}"#);
}
