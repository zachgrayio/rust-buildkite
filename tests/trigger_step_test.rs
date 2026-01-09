mod pipeline_test_common;
use pipeline_test_common::check_result;
use rust_buildkite::*;

#[test]
fn test_trigger_step_allow_dependency_failure() {
    let allow_dependency_failure = true;
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"allow_dependency_failure":true,"async":false,"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_async_string() {
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::String(TriggerStepAsyncString::True),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":"true","trigger":""}"#);
}

#[test]
fn test_trigger_step_async_bool() {
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(true),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":true,"trigger":""}"#);
}

#[test]
fn test_trigger_step_branches() {
    let branches = "branch".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: Some(Branches::String(branches)),
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"branches":"branch","trigger":""}"#);
}

#[test]
fn test_trigger_step_build_branch() {
    let branch = "branch".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch,
            commit: String::new(),
            env: None,
            message: String::new(),
            meta_data: serde_json::Map::new(),
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"branch","commit":"","message":""},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_build_commit() {
    let commit = "commit".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch: String::new(),
            commit,
            env: None,
            message: String::new(),
            meta_data: serde_json::Map::new(),
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"","commit":"commit","message":""},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_build_env() {
    let mut env = serde_json::Map::new();
    env.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch: String::new(),
            commit: String::new(),
            env: Some(Env(env)),
            message: String::new(),
            meta_data: serde_json::Map::new(),
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"","commit":"","env":{"foo":"bar"},"message":""},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_build_message() {
    let message = "message".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch: String::new(),
            commit: String::new(),
            env: None,
            message,
            meta_data: serde_json::Map::new(),
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"","commit":"","message":"message"},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_build_metadata() {
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch: String::new(),
            commit: String::new(),
            env: None,
            message: String::new(),
            meta_data: metadata,
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"","commit":"","message":"","meta_data":{"foo":"bar"}},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_build_all() {
    let branch = "branch".to_string();
    let commit = "commit".to_string();
    let mut env = serde_json::Map::new();
    env.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );
    let message = "message".to_string();
    let mut metadata = serde_json::Map::new();
    metadata.insert(
        "foo".to_string(),
        serde_json::Value::String("bar".to_string()),
    );

    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: Some(TriggerStepBuild {
            branch,
            commit,
            env: Some(Env(env)),
            message,
            meta_data: metadata,
        }),
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"build":{"branch":"branch","commit":"commit","env":{"foo":"bar"},"message":"message","meta_data":{"foo":"bar"}},"trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_depends_on() {
    let depends_on = "step".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: Some(DependsOn::String(depends_on)),
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"depends_on":"step","trigger":""}"#);
}

#[test]
fn test_trigger_step_if() {
    let if_value = "if".to_string();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: Some(If(if_value)),
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"if":"if","trigger":""}"#);
}

#[test]
fn test_trigger_step_if_changed() {
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: Some(IfChanged::String("ifChanged".to_string())),
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"if_changed":"ifChanged","trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_key() {
    let key = Key::try_from("key".to_string()).unwrap();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: Some(key),
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"key":"key","trigger":""}"#);
}

#[test]
fn test_trigger_step_identifier() {
    let key = Key::try_from("identifier".to_string()).unwrap();
    let identifier = TriggerStepKey(key);
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: Some(identifier),
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(
        val,
        r#"{"async":false,"identifier":"identifier","trigger":""}"#,
    );
}

#[test]
fn test_trigger_step_id() {
    let key = Key::try_from("id".to_string()).unwrap();
    let id = TriggerStepKey(key);
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: Some(id),
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"id":"id","trigger":""}"#);
}

#[test]
fn test_trigger_step_label() {
    let label = Label("label".to_string());
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: Some(label),
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"label":"label","trigger":""}"#);
}

#[test]
fn test_trigger_step_name() {
    let name = "name".parse::<TriggerStepLabel>().unwrap();
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: Some(name),
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"name":"name","trigger":""}"#);
}

#[test]
fn test_trigger_step_type() {
    let type_value = TriggerStepType::Trigger;
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: Some(type_value),
    };
    check_result(val, r#"{"async":false,"trigger":"","type":"trigger"}"#);
}

#[test]
fn test_trigger_step_trigger() {
    let trigger = "trigger".to_string();
    let val = TriggerStep {
        trigger,
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"trigger":"trigger"}"#);
}

#[test]
fn test_trigger_step_skip() {
    let skip = true;
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: Some(Skip::Boolean(skip)),
        soft_fail: None,
        type_: None,
    };
    check_result(val, r#"{"async":false,"skip":true,"trigger":""}"#);
}

#[test]
fn test_trigger_step_soft_fail() {
    let soft_fail = SoftFailVariant0String::True;
    let val = TriggerStep {
        trigger: String::new(),
        async_: TriggerStepAsync::Boolean(false),
        allow_dependency_failure: None,
        branches: None,
        build: None,
        depends_on: None,
        id: None,
        identifier: None,
        if_: None,
        if_changed: None,
        key: None,
        label: None,
        name: None,
        skip: None,
        soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::String(soft_fail))),
        type_: None,
    };
    check_result(val, r#"{"async":false,"soft_fail":"true","trigger":""}"#);
}

#[test]
fn test_trigger_step_all() {
    let allow_dependency_failure = true;
    let branches = "branch".to_string();
    let build_commit = "commit".to_string();
    let depends_on = "step".to_string();
    let if_value = "if".to_string();
    let key = Key::try_from("key".to_string()).unwrap();
    let identifier = TriggerStepKey(Key::try_from("identifier".to_string()).unwrap());
    let id = TriggerStepKey(Key::try_from("id".to_string()).unwrap());
    let label = Label("label".to_string());
    let name = "name".parse::<TriggerStepLabel>().unwrap();
    let type_value = TriggerStepType::Trigger;
    let trigger = "trigger".to_string();
    let skip = true;
    let soft_fail = SoftFailVariant0String::True;

    let val = TriggerStep {
        allow_dependency_failure: Some(AllowDependencyFailure::Boolean(allow_dependency_failure)),
        async_: TriggerStepAsync::Boolean(true),
        branches: Some(Branches::String(branches)),
        build: Some(TriggerStepBuild {
            branch: String::new(),
            commit: build_commit,
            env: None,
            message: String::new(),
            meta_data: serde_json::Map::new(),
        }),
        depends_on: Some(DependsOn::String(depends_on)),
        if_: Some(If(if_value)),
        if_changed: Some(IfChanged::String("ifChanged".to_string())),
        key: Some(key),
        identifier: Some(identifier),
        id: Some(id),
        label: Some(label),
        name: Some(name),
        type_: Some(type_value),
        trigger,
        skip: Some(Skip::Boolean(skip)),
        soft_fail: Some(SoftFail::Variant0(SoftFailVariant0::String(soft_fail))),
    };
    check_result(
        val,
        r#"{"allow_dependency_failure":true,"async":true,"branches":"branch","build":{"branch":"","commit":"commit","message":""},"depends_on":"step","id":"id","identifier":"identifier","if":"if","if_changed":"ifChanged","key":"key","label":"label","name":"name","skip":true,"soft_fail":"true","trigger":"trigger","type":"trigger"}"#,
    );
}

#[test]
fn test_nested_trigger_step() {
    let trigger = "trigger".to_string();
    let val = NestedTriggerStep {
        trigger: Some(TriggerStep {
            trigger,
            async_: TriggerStepAsync::Boolean(false),
            allow_dependency_failure: None,
            branches: None,
            build: None,
            depends_on: None,
            id: None,
            identifier: None,
            if_: None,
            if_changed: None,
            key: None,
            label: None,
            name: None,
            skip: None,
            soft_fail: None,
            type_: None,
        }),
    };
    check_result(val, r#"{"trigger":{"async":false,"trigger":"trigger"}}"#);
}
