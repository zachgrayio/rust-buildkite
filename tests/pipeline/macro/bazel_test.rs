#![cfg(feature = "bazel")]

use rust_buildkite::{bazel, pipeline};

mod bazel_macro {
    use super::*;

    #[test]
    fn simple_build() {
        let c = bazel!("build //...");
        assert_eq!(c, "bazel build //...");
    }

    #[test]
    fn simple_test() {
        let c = bazel!("test //...");
        assert_eq!(c, "bazel test //...");
    }

    #[test]
    fn simple_run() {
        let c = bazel!("run //foo:bar");
        assert_eq!(c, "bazel run //foo:bar");
    }

    #[test]
    fn simple_query() {
        let c = bazel!("query //...");
        assert_eq!(c, "bazel query //...");
    }

    #[test]
    fn info_command() {
        let c = bazel!("info");
        assert_eq!(c, "bazel info");
    }

    #[test]
    fn version_command() {
        let c = bazel!("version");
        assert_eq!(c, "bazel version");
    }

    #[test]
    fn build_with_flags() {
        let c = bazel!("build //... --jobs=4");
        assert_eq!(c, "bazel build //... --jobs=4");
    }

    #[test]
    fn test_with_multiple_flags() {
        let c = bazel!("test //... --test_output=errors --jobs=2");
        assert_eq!(c, "bazel test //... --test_output=errors --jobs=2");
    }

    #[test]
    fn build_specific_target() {
        let c = bazel!("build //foo/bar:baz");
        assert_eq!(c, "bazel build //foo/bar:baz");
    }

    #[test]
    fn external_repo_target() {
        let c = bazel!("build @external//pkg:target");
        assert_eq!(c, "bazel build @external//pkg:target");
    }

    #[test]
    fn cquery_command() {
        let c = bazel!("cquery //...");
        assert_eq!(c, "bazel cquery //...");
    }

    #[test]
    fn aquery_command() {
        let c = bazel!("aquery //...");
        assert_eq!(c, "bazel aquery //...");
    }

    #[test]
    fn coverage_command() {
        let c = bazel!("coverage //...");
        assert_eq!(c, "bazel coverage //...");
    }

    #[test]
    fn fetch_command() {
        let c = bazel!("fetch //...");
        assert_eq!(c, "bazel fetch //...");
    }

    #[test]
    fn clean_command() {
        let c = bazel!("clean");
        assert_eq!(c, "bazel clean");
    }

    #[test]
    fn shutdown_command() {
        let c = bazel!("shutdown");
        assert_eq!(c, "bazel shutdown");
    }
}

mod shorthand_macros {
    use super::*;

    #[test]
    fn bazel_build_basic() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build all",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel build //..."));
        assert!(yaml.contains("label: build all"));
        assert!(yaml.contains("key: build"));
    }

    #[test]
    fn bazel_test_basic() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test all",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel test //..."));
        assert!(yaml.contains("label: test all"));
    }

    #[test]
    fn bazel_run_basic() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_run {
                    target_patterns: "//foo:bar",
                    label: "run foo",
                    key: "run"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel run //foo:bar"));
    }

    #[test]
    fn bazel_query_basic() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_query {
                    target_patterns: "//...",
                    label: "query all",
                    key: "query"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel query //..."));
    }

    #[test]
    fn bazel_command_with_verb() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_command {
                    verb: "cquery",
                    target_patterns: "//...",
                    label: "cquery all",
                    key: "cquery"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel cquery //..."));
    }
}

mod flags_syntax {
    use super::*;

    #[test]
    fn flags_as_string() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: "--jobs=4 --verbose_failures",
                    label: "build with string flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=4"));
        assert!(yaml.contains("--verbose_failures"));
    }

    #[test]
    fn flags_as_array() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: ["--jobs=4", "--verbose_failures"],
                    label: "build with array flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=4"));
        assert!(yaml.contains("--verbose_failures"));
    }

    #[test]
    fn empty_flag_value() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    flags: "--disk_cache=",
                    label: "test with empty flag",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--disk_cache="));
    }

    #[test]
    fn multiple_empty_flags() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: ["--disk_cache=", "--remote_cache="],
                    label: "build with empty flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--disk_cache="));
        assert!(yaml.contains("--remote_cache="));
    }
}

mod target_patterns {
    use super::*;

    #[test]
    fn wildcard_all() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build all",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build //..."));
    }

    #[test]
    fn specific_target() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//foo:bar",
                    label: "build specific",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build //foo:bar"));
    }

    #[test]
    fn package_all_targets() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//foo:all",
                    label: "build package all",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build //foo:all"));
    }

    #[test]
    fn external_repo() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "@external//pkg:target",
                    label: "build external",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build @external//pkg:target"));
    }

    #[test]
    fn multiple_targets() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//foo:bar //baz:qux",
                    label: "build multiple",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
        assert!(yaml.contains("//baz:qux"));
    }

    #[test]
    fn subtraction_pattern() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//... -//foo:bar",
                    label: "build with subtraction",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("-- //... -//foo:bar"));
    }

    #[test]
    fn multiple_subtractions() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//... -//foo:bar -//baz:qux",
                    label: "build with multiple subtractions",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("-- //... -//foo:bar -//baz:qux"));
    }
}

mod step_options {
    use super::*;

    #[test]
    fn with_env() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    env: {
                        CC: "clang"
                    }
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("CC: clang"));
    }

    #[test]
    fn with_depends_on() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                },
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    depends_on: ["build"]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("- build"));
    }

    #[test]
    fn with_timeout() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    timeout_in_minutes: 30
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("timeout_in_minutes: 30"));
    }

    #[test]
    fn with_soft_fail() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    soft_fail: true
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("soft_fail: true"));
    }

    #[test]
    fn with_parallelism() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    parallelism: 4
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("parallelism: 4"));
    }

    #[test]
    fn with_agents() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    agents: {
                        queue: "linux"
                    }
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("agents:"));
        assert!(yaml.contains("queue: linux"));
    }
}

mod validation_options {
    use super::*;

    #[test]
    fn validate_targets_false() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//nonexistent:target",
                    label: "build",
                    key: "build",
                    validate_targets: false
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build //nonexistent:target"));
    }

    #[test]
    fn dry_run_false() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    dry_run: false
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel build //..."));
    }

    #[test]
    fn custom_verbs_single() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_command {
                    verb: "my_custom_verb",
                    target_patterns: "//...",
                    label: "custom",
                    key: "custom",
                    custom_verbs: ["my_custom_verb"]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel my_custom_verb //..."));
    }

    #[test]
    fn custom_verbs_multiple() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_command {
                    verb: "deploy",
                    target_patterns: "//app:server",
                    label: "deploy",
                    key: "deploy",
                    custom_verbs: ["deploy", "publish", "release"]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel deploy //app:server"));
    }

    #[test]
    fn pipeline_level_custom_verbs_with_bazel_lint() {
        let p = pipeline! {
            custom_verbs: ["lint"],
            steps: [
                bazel_lint {
                    target_patterns: "//...",
                    label: "lint",
                    key: "lint"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel lint //..."));
    }

    #[test]
    fn pipeline_level_custom_verbs_with_bazel_deploy() {
        let p = pipeline! {
            custom_verbs: ["deploy"],
            steps: [
                bazel_deploy {
                    target_patterns: "//app:server",
                    label: "deploy",
                    key: "deploy"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel deploy //app:server"));
    }

    #[test]
    fn pipeline_level_multiple_custom_verbs() {
        let p = pipeline! {
            custom_verbs: ["lint", "deploy", "publish"],
            steps: [
                bazel_lint {
                    target_patterns: "//...",
                    label: "lint",
                    key: "lint"
                },
                bazel_deploy {
                    target_patterns: "//app:server",
                    label: "deploy",
                    key: "deploy",
                    depends_on: ["lint"]
                },
                bazel_publish {
                    target_patterns: "//packages/...",
                    label: "publish",
                    key: "publish",
                    depends_on: ["deploy"]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel lint //..."));
        assert!(yaml.contains("bazel deploy //app:server"));
        assert!(yaml.contains("bazel publish //packages/..."));
    }

    #[test]
    fn custom_verb_with_flags() {
        let p = pipeline! {
            custom_verbs: ["lint"],
            steps: [
                bazel_lint {
                    target_patterns: "//...",
                    flags: "--config=strict",
                    label: "lint with config",
                    key: "lint"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel lint --config=strict //..."));
    }
}

mod integration {
    use super::*;

    #[test]
    fn mixed_steps() {
        let p = pipeline! {
            env: {},
            steps: [
                command {
                    command: bazel!("info"),
                    label: "info",
                    key: "info"
                },
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                },
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    depends_on: ["build"]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("bazel info"));
        assert!(yaml.contains("bazel build //..."));
        assert!(yaml.contains("bazel test //..."));
    }

    #[test]
    fn full_pipeline() {
        let p = pipeline! {
            env: {
                BAZEL_CACHE: "/tmp/cache"
            },
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: "--verbose_failures",
                    label: "Build all targets",
                    key: "build",
                    agents: {
                        queue: "linux"
                    }
                },
                bazel_test {
                    target_patterns: "//... -//integration:tests",
                    flags: ["--test_output=errors", "--jobs=4"],
                    label: "Unit tests",
                    key: "unit-test",
                    depends_on: ["build"],
                    timeout_in_minutes: 30
                },
                bazel_test {
                    target_patterns: "//integration/...",
                    flags: "--test_output=all",
                    label: "Integration tests",
                    key: "integration-test",
                    depends_on: ["unit-test"],
                    soft_fail: true
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("BAZEL_CACHE"));
        assert!(yaml.contains("bazel build"));
        assert!(yaml.contains("bazel test"));
        assert!(yaml.contains("--test_output=errors"));
        assert!(yaml.contains("soft_fail: true"));
    }
}
