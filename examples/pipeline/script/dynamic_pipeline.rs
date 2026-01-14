#!/usr/bin/env rust-script

//! This is an example dynamic pipeline script meant to be directly invoked with something
//! like:
//!
//!   ./dynamic_pipeline.rs | buildkite-agent pipeline upload
//!
//! When executed on a CI worker, the pipeline will be validated at compile time,
//! including all commands and environment variables.
//!
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../.." }
//! serde_yaml = "0.9"
//! ```

use rust_buildkite::pipeline;

fn main() {
    let pipeline = pipeline! {
        // Buildkite pipeline-level environment variables
        // these are available to all steps, and references to them are validated at compile
        // time.
        env: {
            CI: "true",
            RUST_BACKTRACE: "1",
            CARGO_TERM_COLOR: "always"
        },

        // additional runtime environment variables that commands are allowed to reference,
        // aside from those in the env block.
        //  - these are validated at compile time - undefined vars cause errors.
        //  - if not supplied, defaults to host env; including these here allows this to build
        //    on non-CI machines.
        //  - SHELL_ENV, BUILDKITE_ENV, CI_ENV are keywords that expand to known values for
        //    convenience.
        // nb: this could (and should) be omitted entirely to validate against the host env
        // during compilation!
        runtime_env: [SHELL_ENV, BUILDKITE_ENV],

        // commands use host PATH by default for compile time validation of commands;
        // you can override this with an explicit allowed_commands list:
        // allowed_commands: ["cargo", "npm", "docker"],

        // declare paths that don't exist at compile time to allow them to be used at runtime
        // again these will be validated at runtime on the CI worker, but we need it here to
        // allow the pipeline to build on non-CI machines
        allow_missing_paths: ["./scripts/deploy.sh"],

        steps: [
            command {
                // shell syntax and executable references are validated at compile time.
                // nb: the cmd! macro is required here, and raw string commands are not
                // allowed.
                // this is what enables compile time validation of commands.
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
                // these env vars are all checked at compile time, along with shell syntax.
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
                // builtins (cd, source, export) are always allowed.
                // bash is always used for shell syntax validation, and bash
                // builtins will be allowed by default in addition to POSIX standard;
                // this may change in the future
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
