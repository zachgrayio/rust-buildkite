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

const STATIC_TARGETS: &str = "//cpp:hello-world //cpp:hello-lib";

fn get_targets_from_env() -> String {
    std::env::var("BAZEL_TARGETS").unwrap_or_else(|_| "//...".to_string())
}

fn main() {
    let app_name = "cpp";
    let dynamic_flags = format!("--jobs={}", num_cpus());

    let pipeline = pipeline! {
        env: { CI: "true" },
        agents: { queue: "bazel-runners" },
        notify: [{ slack: "#builds" }],

        steps: [
            command {
                command: bazel!("info"),
                label: format!("{} info", app_name),
                key: "info"
            },

            bazel_build {
                target_patterns: "//cpp:hello-world",
                label: format!("{} build", app_name),
                key: runtime!(format!("{}_build", app_name))
            },

            command {
                commands: [
                    cmd!("echo 'Starting pipeline'"),
                    bazel_build {
                        target_patterns: "//cpp:hello-world",
                        config: ["ci", "remote"],
                        compilation_mode: "opt"
                    },
                    bazel_test {
                        target_patterns: "//cpp:hello-success_test",
                        test_tag_filters: ["-slow"],
                        config: "ci"
                    },
                    bazel_run {
                        target_patterns: "//cpp:hello-world",
                        args: ["--greeting", "hello"]
                    }
                ],
                label: "Build, Test, and Run",
                key: "build_test"
            },

            bazel_build {
                target_patterns: "//...",
                config: ["ci", "remote"],
                compilation_mode: "opt",
                build_tag_filters: ["-slow"],
                label: "build with shorthands",
                key: "build_shorthands"
            },

            bazel_build {
                target_patterns: comptime!(STATIC_TARGETS),
                label: "comptime const",
                key: "build_const"
            },

            bazel_build {
                target_patterns: comptime_shell!("echo '//cpp:hello-world'"),
                label: "comptime shell",
                key: "build_shell"
            },

            bazel_build {
                target_patterns: runtime!(get_targets_from_env()),
                flags: runtime!(dynamic_flags),
                label: "runtime targets",
                key: "build_dynamic"
            },

            bazel_run {
                target_patterns: "//cpp:hello-world",
                config: "ci",
                args: ["--greeting", "hello"],
                label: "run",
                key: "run"
            },

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
