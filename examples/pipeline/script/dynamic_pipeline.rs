#!/usr/bin/env rust-script
//! Example: Building a Buildkite pipeline as a standalone script using rust-script
//!
//! This demonstrates how to use rust-script to run pipeline generation as a simple
//! script file, similar to `go run .buildkite/pipeline.go` in Go projects.
//!
//! ## Usage
//!
//! 1. Install rust-script: `cargo install rust-script`
//! 2. Run directly: `rust-script pipeline_examples/rust_script/dynamic_pipeline.rs`
//! 3. Or make executable: `chmod +x dynamic_pipeline.rs && ./dynamic_pipeline.rs`
//!
//! In a Buildkite pipeline:
//! ```yaml
//! steps:
//!   - label: ":pipeline: Generate pipeline"
//!     command: rust-script .buildkite/pipeline.rs | buildkite-agent pipeline upload
//! ```
//!
//! ```cargo
//! [dependencies]
//! # When rust-buildkite is published to crates.io, use:
//! # rust-buildkite = "0.1"
//!
//! # change this to a real version when published to crates.io
//! rust-buildkite = { path = "../.." }
//! serde_json = "1.0"
//! serde_yaml = "0.9"
//! ```

use rust_buildkite::pipeline;

fn main() {
    let pipeline = pipeline! {
        env: {
            CI: "true",
            RUST_BACKTRACE: "1"
        },
        steps: [
            command("cargo fmt --check && cargo clippy -- -D warnings")
                .label("Lint")
                .key("lint"),
            command("cargo test --all-features")
                .label("Test")
                .key("test"),
            wait,
            block("Deploy to Production?"),
            command("./deploy.sh production")
                .label("Deploy")
                .depends_on("test")
        ]
    };
    println!("{}", serde_yaml::to_string(&pipeline).expect("Failed to serialize pipeline"));
}
