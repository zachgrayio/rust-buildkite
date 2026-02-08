#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../..", features = ["bazel"] }
//! serde_yaml = "0.9"
//! ```
//!
//! Dynamic patterns for Bazel pipelines:
//! - `label: expr` - Any expression for labels
//! - `key: "literal"` - Validated keys
//! - `key: runtime!(expr)` - Dynamic keys (skip validation)
//! - `args: ["--flag", "value"]` - Program args for bazel run
//! - `comptime_shell!("cmd")` - Shell at compile time
//! - `comptime!(CONST)` - Const at compile time
//! - `runtime!(expr)` - Runtime expression (skip validation)

#![allow(unused_imports)]
use rust_buildkite::{comptime, comptime_shell, pipeline, runtime};

const STATIC_TARGETS: &str = "//app:main //lib:core";

fn get_targets_from_env() -> String {
    std::env::var("BAZEL_TARGETS").unwrap_or_else(|_| "//...".to_string())
}

fn main() {
    let app_name = "myapp";
    let dynamic_flags = format!("--jobs={}", num_cpus());

    let pipeline = pipeline! {
        env: { CI: "true" },
        agents: { queue: "bazel-runners" },
        notify: [{ slack: "#builds" }],
        image: "gcr.io/bazel-public/bazel:latest",
        secrets: ["CACHE_KEY"],
        priority: 5,

        steps: [
            // Dynamic label with format!
            command {
                command: bazel!("info"),
                label: format!("{} info", app_name),
                key: "info"
            },

            // Dynamic key with runtime!
            bazel_build {
                target_patterns: "//app:main",
                label: format!("{} build", app_name),
                key: runtime!(format!("{}_build", app_name))
            },

            command {
                commands: [
                    cmd!("echo 'Starting pipeline'"),
                    bazel_build {
                        target_patterns: "//app:main",
                        config: ["ci", "remote"],
                        compilation_mode: "opt"
                    },
                    bazel_test {
                        target_patterns: "//...",
                        test_tag_filters: ["-foo", "-bar"],
                        config: "ci"
                    },
                    bazel_run {
                        target_patterns: "//tools/ci:publish",
                        args: ["--bucket", "artifacts", "--region", "us-west-2"]
                    }
                ],
                label: "Build, Test, and Publish",
                key: "build_test"
            },

            // Flag shorthands
            bazel_build {
                target_patterns: "//...",
                config: ["ci", "remote"],
                compilation_mode: "opt",
                build_tag_filters: ["-foo"],
                label: "build with shorthands",
                key: "build_shorthands"
            },

            // Compile-time expressions
            bazel_build {
                target_patterns: comptime!(STATIC_TARGETS),
                label: "comptime const",
                key: "build_const"
            },

            bazel_build {
                target_patterns: comptime_shell!("echo '//cpp:hello'"),
                label: "comptime shell",
                key: "build_shell"
            },

            // Runtime expressions
            bazel_build {
                target_patterns: runtime!(get_targets_from_env()),
                flags: runtime!(dynamic_flags),
                label: "runtime targets",
                key: "build_dynamic"
            },

            bazel_run {
                target_patterns: "//tools:deploy",
                config: "ci",
                args: ["--env", "prod", "--verbose"],
                label: "deploy",
                key: "deploy"
            },

            // Custom verb
            bazel_command {
                verb: "query",
                target_patterns: "//...",
                label: "query",
                key: "query"
            },
        ]
    };

    println!("{}", serde_yaml::to_string(&pipeline).expect("yaml"));
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4)
}
