#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../..", features = ["bazel"] }
//! serde_yaml = "0.9"
//! ```
//!
//! Dynamic target selection patterns for rust-script pipelines:
//!
//! - `comptime_shell!("cmd")`: Run shell at compile time, returns string literal
//! - `comptime!(CONST)`: Use a const value
//! - `runtime!(expr)`: Evaluate at runtime, skips compile-time validation

#![allow(unused_imports)]
use rust_buildkite::{comptime, comptime_shell, pipeline, runtime};

const STATIC_TARGETS: &str = "//app:main //lib:core";

fn get_targets_from_env() -> String {
    std::env::var("BAZEL_TARGETS").unwrap_or_else(|_| "//...".to_string())
}

fn main() {
    let dynamic_flags = format!("--jobs={}", num_cpus());

    let pipeline = pipeline! {
        env: {
            FOO: "bar"
        },
        agents: { queue: "bazel-runners" },
        notify: [
            { slack: "#bazel-builds" },
            { slack: "#alerts", r#if: "build.state == 'failed'" }
        ],
        image: "gcr.io/bazel-public/bazel:latest",
        secrets: ["REMOTE_CACHE_KEY"],
        priority: 5,

        steps: [
            command {
                command: bazel!("info"),
                label: "info",
                key: "info"
            },

            bazel_test {
                target_patterns: "//...",
                flags: "--jobs=4 --verbose_failures",
                label: "test all",
                key: "test"
            },

            bazel_build {
                target_patterns: comptime!(STATIC_TARGETS),
                label: "build from const",
                key: "build_const"
            },

            bazel_build {
                target_patterns: comptime_shell!("echo '//cpp:hello-world'"),
                label: "build from shell",
                key: "build_shell"
            },

            bazel_build {
                target_patterns: runtime!(get_targets_from_env()),
                flags: runtime!(dynamic_flags),
                label: "build dynamic",
                key: "build_dynamic"
            },

            bazel_command {
                verb: "query",
                target_patterns: "//...",
                label: "query targets",
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
