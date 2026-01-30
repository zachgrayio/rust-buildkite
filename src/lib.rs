//! A Rust library for the Buildkite API.
//!
//! This library provides a client for the Buildkite API with typed requests and responses.
//!
//! # Pipeline DSL
//!
//! Use the `pipeline!` macro for declarative, type-safe pipeline definitions with
//! compile-time command validation:
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     env: { CI: "true" },
//!     allow_missing_paths: ["./deploy.sh"],
//!     steps: [
//!         command(cmd!("echo hello")).label("Say Hello").key("hello"),
//!         command(cmd!("cat README.md")).key("tests").depends_on("hello"),
//!         wait,
//!         block("Deploy?"),
//!         command(cmd!("./deploy.sh")).depends_on("tests")
//!     ]
//! };
//! ```
//!
//! # Type-Checked Commands
//!
//! All commands must use `cmd!("...")` for compile-time validation:
//!
//! - **bashrs validation**: Shell syntax and anti-patterns are checked at compile time
//! - **Allowlist validation**: By default, commands are checked against host PATH
//! - **Path validation**: `./script.sh` and `/path/to/cmd` are validated to exist
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     steps: [
//!         // Commands are validated against host PATH (the default)
//!         command(cmd!("echo install")).key("install"),
//!         command(cmd!("cargo build --release")).key("build"),
//!     ]
//! };
//! ```
//!
//! ## Custom Allowed Commands
//!
//! Override the default host PATH discovery with an explicit list:
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     // Only these commands are allowed (overrides host PATH default)
//!     allowed_commands: ["echo", "cargo"],
//!     steps: [
//!         command(cmd!("echo install")).key("install"),
//!         command(cmd!("cargo build")).key("build"),
//!     ]
//! };
//! ```
//!
//! ## Path Validation
//!
//! Path-based commands (`./script.sh`, `/usr/bin/env`) are validated to exist
//! at compile time. For paths that only exist at runtime, use `allow_missing_paths`:
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     // Skip validation for paths that don't exist on the build machine
//!     allow_missing_paths: ["./deploy.sh", "./scripts/setup.sh"],
//!     steps: [
//!         command(cmd!("./deploy.sh production")).key("deploy"),
//!     ]
//! };
//! ```
//!
//! # Command Validation
//!
//! By default, `pipeline!` validates commands against the host PATH at compile time.
//! You can override this with an explicit `allowed_commands` list:
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! // Default: uses host PATH commands (echo, cargo, etc. must be on build machine)
//! let p = pipeline! {
//!     steps: [
//!         command(cmd!("echo install")).key("install"),
//!     ]
//! };
//!
//! // Explicit: only allow specific commands (useful for CI/CD consistency)
//! let p = pipeline! {
//!     allowed_commands: ["echo", "cargo"],
//!     steps: [
//!         command(cmd!("echo install")).key("install"),
//!     ]
//! };
//! ```
//!
//! # Conditional Expressions
//!
//! Buildkite `if` conditions are validated at compile time:
//!
//! ```no_run
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     steps: [
//!         // Branch conditions
//!         command(cmd!("echo deploy"))
//!             .label("Deploy")
//!             .key("deploy")
//!             .r#if("build.branch == 'main'"),
//!
//!         // Regex matching
//!         command(cmd!("echo feature"))
//!             .key("feature")
//!             .r#if("build.branch =~ /^feature\\//"),
//!
//!         // Logical operators
//!         command(cmd!("echo prod"))
//!             .key("prod")
//!             .r#if("build.branch == 'main' && build.state == 'passed'"),
//!
//!         // Environment functions
//!         command(cmd!("echo ci"))
//!             .key("ci")
//!             .r#if("env('CI') == 'true'"),
//!     ]
//! };
//! ```
//!
//! Invalid conditions produce compile errors:
//! - Unknown references (e.g., `unknown.field`)
//! - Invalid regex patterns
//! - Syntax errors in expressions
//!

// nb: re-export required because the pipeline macro needs this for the unconstrainted raw json
// allowed in fields like env, retry, plugins, etc. maybe can clean those types up later.
pub use serde_json;

pub use rust_buildkite_macros::{cmd, pipeline};

/// Discovers all available commands from the host machine's PATH at compile time.
///
/// Note: The `pipeline!` macro automatically uses host PATH discovery by default,
/// so you typically don't need to call this macro directly. It's provided for
/// advanced use cases where you need direct access to the host commands list.
#[allow(unused)]
#[crabtime::function]
pub fn host_path_commands() {
    use std::collections::HashSet;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let mut commands = HashSet::new();
    if let Ok(path) = std::env::var("PATH") {
        for dir in path.split(':') {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Ok(metadata) = path.metadata() {
                            let mode = metadata.permissions().mode();
                            if mode & 0o111 != 0 {
                                if let Some(name) = path.file_name() {
                                    if let Some(name_str) = name.to_str() {
                                        commands.insert(name_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut sorted: Vec<_> = commands.into_iter().collect();
    sorted.sort();
    let items: Vec<String> = sorted
        .iter()
        .map(|s| format!("\"{}\".to_string()", s))
        .collect();

    let output = format!("vec![{}]", items.join(", "));
    crabtime::output_str!("{}", output);
}

#[allow(
    dead_code,
    clippy::derivable_impls,
    clippy::must_use_candidate,
    clippy::infallible_try_from
)]
mod codegen {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}
pub use codegen::*;

pub mod error;
pub use error::*;

pub mod client;
pub mod services;
pub mod types;
pub mod webhook;

pub use client::{Client, ClientBuilder, Response, ResponseExt};
pub use types::*;
pub use webhook::*;
