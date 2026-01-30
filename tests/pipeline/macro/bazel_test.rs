#![cfg(feature = "bazel")]
#![allow(unused_imports)]

use rust_buildkite::{bazel, comptime, pipeline, runtime};

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

    #[test]
    fn dynamic_target_from_variable() {
        let targets = "//foo:bar //baz:qux";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: runtime!(targets),
                    label: "build dynamic targets",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
        assert!(yaml.contains("//baz:qux"));
    }

    #[test]
    fn dynamic_target_from_format() {
        let package = "foo";
        let target = "bar";
        let targets = format!("//{}:{}", package, target);
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: runtime!(targets),
                    label: "build formatted target",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
    }

    #[test]
    fn dynamic_target_with_static_flags() {
        let targets = "//...";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: runtime!(targets),
                    flags: "--test_output=errors",
                    label: "test with dynamic targets",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//..."));
        assert!(yaml.contains("--test_output=errors"));
    }

    #[test]
    fn both_dynamic_flags_and_targets() {
        let targets = "//foo:bar";
        let flags = "--jobs=4";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: runtime!(targets),
                    flags: runtime!(flags),
                    label: "build with both dynamic",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
        assert!(yaml.contains("--jobs=4"));
    }

    #[test]
    fn comptime_target_from_const() {
        const TARGETS: &str = "//foo:bar //baz:qux";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(TARGETS),
                    label: "build comptime targets",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
        assert!(yaml.contains("//baz:qux"));
    }

    #[test]
    fn comptime_target_from_static() {
        static TARGETS: &str = "//lib:core";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: comptime!(TARGETS),
                    label: "test comptime targets",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//lib:core"));
    }

    #[test]
    fn comptime_with_concat() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(concat!("//foo", ":bar")),
                    label: "build concat target",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//foo:bar"));
    }

    #[test]
    fn comptime_flags_from_const() {
        const FLAGS: &str = "--jobs=8 --verbose_failures";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: comptime!(FLAGS),
                    label: "build with comptime flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=8"));
        assert!(yaml.contains("--verbose_failures"));
    }

    #[test]
    fn comptime_both_targets_and_flags() {
        const TARGETS: &str = "//app:main //lib:core";
        const FLAGS: &str = "--compilation_mode=opt";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(TARGETS),
                    flags: comptime!(FLAGS),
                    label: "build optimized",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//app:main"));
        assert!(yaml.contains("//lib:core"));
        assert!(yaml.contains("--compilation_mode=opt"));
    }

    #[test]
    fn mixed_comptime_targets_runtime_flags() {
        const TARGETS: &str = "//...";
        let jobs = "4";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(TARGETS),
                    flags: runtime!(format!("--jobs={}", jobs)),
                    label: "mixed comptime/runtime",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//..."));
        assert!(yaml.contains("--jobs=4"));
    }
}

mod comptime_flags {
    use super::*;

    #[test]
    fn flags_from_const() {
        const FLAGS: &str = "--jobs=8 --verbose_failures";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: comptime!(FLAGS),
                    label: "build with comptime flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=8"));
        assert!(yaml.contains("--verbose_failures"));
    }

    #[test]
    fn flags_from_concat() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: comptime!(concat!("--jobs=", "4")),
                    label: "build with concat flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=4"));
    }

    #[test]
    fn both_comptime() {
        const TARGETS: &str = "//app:main //lib:core";
        const FLAGS: &str = "--compilation_mode=opt";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(TARGETS),
                    flags: comptime!(FLAGS),
                    label: "build optimized",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//app:main"));
        assert!(yaml.contains("//lib:core"));
        assert!(yaml.contains("--compilation_mode=opt"));
    }

    #[test]
    fn mixed_comptime_runtime() {
        const TARGETS: &str = "//...";
        let jobs = "4";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime!(TARGETS),
                    flags: runtime!(format!("--jobs={}", jobs)),
                    label: "mixed comptime/runtime",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//..."));
        assert!(yaml.contains("--jobs=4"));
    }
}

mod comptime_shell_tests {
    use super::*;
    use rust_buildkite::comptime_shell;

    #[test]
    fn shell_echo_targets() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: comptime_shell!("echo '//tests:unit //tests:e2e'"),
                    label: "test shell targets",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//tests:unit"));
        assert!(yaml.contains("//tests:e2e"));
    }

    #[test]
    fn shell_flags() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: comptime_shell!("echo '--jobs=8 --verbose_failures'"),
                    label: "build with shell flags",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs=8"));
        assert!(yaml.contains("--verbose_failures"));
    }

    #[test]
    fn shell_computed_value() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    flags: comptime_shell!("echo --jobs=$(nproc 2>/dev/null || sysctl -n hw.ncpu)"),
                    label: "build with computed parallelism",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("--jobs="));
    }

    #[test]
    fn shell_both_targets_and_flags() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: comptime_shell!("echo '//app:main //lib:core'"),
                    flags: comptime_shell!("echo '--keep_going'"),
                    label: "full shell example",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("//app:main"));
        assert!(yaml.contains("--keep_going"));
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
    fn with_env_comptime() {
        const COMPILER: &str = "clang-15";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    env: {
                        CC: comptime!(COMPILER)
                    }
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("CC: clang-15"));
    }

    #[test]
    fn with_env_runtime() {
        let compiler = "gcc".to_string();
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    env: {
                        CC: runtime!(compiler)
                    }
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("CC: gcc"));
    }

    #[test]
    fn with_env_mixed() {
        const VERSION: &str = "18";
        let debug_level = "2";
        let p = pipeline! {
            env: {},
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build",
                    env: {
                        CC: "clang",
                        CC_VERSION: comptime!(VERSION),
                        DEBUG_LEVEL: runtime!(debug_level)
                    }
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("CC: clang"));
        assert!(yaml.contains("CC_VERSION: '18'"));
        assert!(yaml.contains("DEBUG_LEVEL: '2'"));
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

    #[test]
    fn with_plugins() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    plugins: [
                        { "test-collector#v1.10.1": {
                            files: "bazel-testlogs/**/test.xml",
                            format: "junit"
                        }}
                    ]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("plugins:"));
        assert!(yaml.contains("test-collector#v1.10.1"));
        assert!(yaml.contains("files: bazel-testlogs/**/test.xml"));
        assert!(yaml.contains("format: junit"));
    }

    #[test]
    fn with_multiple_plugins() {
        let p = pipeline! {
            env: {},
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test",
                    plugins: [
                        { "test-collector#v1.10.1": {
                            files: "bazel-testlogs/**/test.xml",
                            format: "junit"
                        }},
                        { "junit-annotate#v2.4.1": {
                            artifacts: "test-results/*.xml"
                        }}
                    ]
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("test-collector#v1.10.1"));
        assert!(yaml.contains("junit-annotate#v2.4.1"));
    }
}

mod validation_options {
    use super::*;

    #[test]
    fn omit_target_patterns_with_custom_verb() {
        let p = pipeline! {
            custom_verbs: ["configure"],
            steps: [
                bazel_configure {
                    label: "configure",
                    key: "configure"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel configure"));
        assert!(!yaml.contains("//"));
    }

    #[test]
    fn omit_target_patterns_with_info() {
        let p = pipeline! {
            steps: [
                bazel_command {
                    verb: "info",
                    label: "bazel info",
                    key: "info"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel info"));
    }

    #[test]
    fn omit_target_patterns_with_flags() {
        let p = pipeline! {
            custom_verbs: ["configure"],
            steps: [
                bazel_configure {
                    flags: "--enable_bzlmod",
                    label: "configure",
                    key: "configure"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("command: bazel configure --enable_bzlmod"));
    }

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

mod pipeline_properties {
    use super::*;

    #[test]
    fn with_notify_slack() {
        let p = pipeline! {
            notify: [{ slack: "#bazel-builds" }],
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("slack"));
        assert!(yaml.contains("#bazel-builds"));
    }

    #[test]
    fn with_notify_slack_conditional() {
        let p = pipeline! {
            notify: [
                { slack: "#builds" },
                { slack: "#alerts", r#if: "build.state == 'failed'" }
            ],
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test",
                    key: "test"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("#builds"));
        assert!(yaml.contains("#alerts"));
        assert!(yaml.contains("if:"));
    }

    #[test]
    fn with_notify_email() {
        let p = pipeline! {
            notify: [{ email: "team@example.com" }],
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("email"));
    }

    #[test]
    fn with_notify_webhook() {
        let p = pipeline! {
            notify: [{ webhook: "https://hooks.example.com/notify" }],
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("webhook"));
    }

    #[test]
    fn with_pipeline_agents() {
        let p = pipeline! {
            agents: { queue: "bazel-runners" },
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("agents:"));
        assert!(yaml.contains("queue: bazel-runners"));
    }

    #[test]
    fn with_image() {
        let p = pipeline! {
            image: "gcr.io/bazel-public/bazel:6.0.0",
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("image:"));
        assert!(yaml.contains("gcr.io/bazel-public/bazel:6.0.0"));
    }

    #[test]
    fn with_secrets_array() {
        let p = pipeline! {
            secrets: ["REMOTE_CACHE_KEY", "GCP_CREDENTIALS"],
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("secrets:"));
        assert!(yaml.contains("REMOTE_CACHE_KEY"));
        assert!(yaml.contains("GCP_CREDENTIALS"));
    }

    #[test]
    fn with_secrets_object() {
        let p = pipeline! {
            secrets: { CACHE_KEY: "remote-cache-secret" },
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("secrets:"));
        assert!(yaml.contains("CACHE_KEY"));
    }

    #[test]
    fn with_priority() {
        let p = pipeline! {
            priority: 10,
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build",
                    key: "build"
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("priority: 10"));
    }

    #[test]
    fn full_pipeline_with_all_properties() {
        let p = pipeline! {
            env: {
                BAZEL_REMOTE_CACHE: "grpc://cache.example.com:9092"
            },
            agents: { queue: "bazel-linux", os: "linux" },
            notify: [
                { slack: "#ci-builds" },
                { email: "oncall@example.com", r#if: "build.state == 'failed'" }
            ],
            image: "gcr.io/bazel-public/bazel:latest",
            secrets: ["REMOTE_CACHE_KEY"],
            priority: 5,
            custom_verbs: ["lint"],
            steps: [
                bazel_lint {
                    target_patterns: "//...",
                    label: "lint",
                    key: "lint"
                },
                bazel_build {
                    target_patterns: "//...",
                    flags: "--verbose_failures",
                    label: "build",
                    key: "build",
                    depends_on: ["lint"]
                },
                bazel_test {
                    target_patterns: "//...",
                    flags: ["--test_output=errors", "--jobs=8"],
                    label: "test",
                    key: "test",
                    depends_on: ["build"],
                    timeout_in_minutes: 60,
                    soft_fail: true
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("BAZEL_REMOTE_CACHE"));
        assert!(yaml.contains("queue: bazel-linux"));
        assert!(yaml.contains("slack"));
        assert!(yaml.contains("email"));
        assert!(yaml.contains("image:"));
        assert!(yaml.contains("secrets:"));
        assert!(yaml.contains("priority: 5"));
        assert!(yaml.contains("bazel lint"));
        assert!(yaml.contains("bazel build"));
        assert!(yaml.contains("bazel test"));
    }
}
