#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../..", features = ["bazel"] }
//! serde_yaml = "0.9"
//! ```

use rust_buildkite::pipeline;

fn main() {
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
                target_patterns: "//cpp:hello-world",
                label: "build hello-world",
                key: "build"
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
