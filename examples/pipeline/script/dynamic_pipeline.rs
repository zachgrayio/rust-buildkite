#!/usr/bin/env rust-script

//! This is an example dynamic pipeline script meant to be directly invoked with something
//! like:
//!
//!   ./dynamic_pipeline.rs | buildkite-agent pipeline upload
//!
//! When executed on a CI worker, the pipeline will be validated at compile time,
//! including all commands and environment variables.
//!
//! To run locally:
//!   BUILDKITE_SKIP_RUNTIME_VALIDATION=1 cargo run --example dynamic-pipeline-script
//!
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../.." }
//! serde_yaml = "0.9"
//! ```

use rust_buildkite::pipeline;

fn main() {
    let pipeline = pipeline! {
        env: {
            CI: "true",
            RUST_BACKTRACE: "1",
            CARGO_TERM_COLOR: "always"
        },
        expect_env: [SHELL_ENV, BUILDKITE_ENV],
        expect_paths: ["./scripts/deploy.sh"],
        steps: [
            command {
                command: cmd!("cargo fmt --check"),
                label: "🎨 Format",
                key: "fmt"
            },
            command {
                command: cmd!("cargo clippy -- -D warnings"),
                label: "📎 Clippy",
                key: "clippy",
                soft_fail: true
            },
            command {
                command: cmd!("cargo test --all-features"),
                label: "🧪 Tests",
                key: "test",
                env: {
                    RUST_LOG: "debug"
                },
                parallelism: 4,
                retry: {
                    automatic: { limit: 2 }
                }
            },
            wait,
            command {
                command: cmd!(r#"echo "Building $BUILDKITE_BRANCH @ $BUILDKITE_COMMIT""#),
                label: "📋 Build Info"
            },
            command {
                command: cmd!("cargo build --release"),
                label: "🔨 Build",
                key: "build",
                artifact_paths: ["target/release/myapp"]
            },
            wait,
            block {
                block: "Deploy to Production?",
                key: "approval",
                prompt: "Are you sure?",
                branches: ["main"]
            },
            command {
                command: cmd!("cd dist && ./scripts/deploy.sh production"),
                label: "🚀 Deploy",
                key: "deploy",
                depends_on: ["build", "approval"],
                concurrency: 1,
                concurrency_group: "deploy/production"
            }
        ]
    };

    println!("{}", serde_yaml::to_string(&pipeline).expect("yaml"));
}
