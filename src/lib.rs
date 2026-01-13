//! A Rust library for the Buildkite API.
//!
//! This library provides a client for the Buildkite API with typed requests and responses.
//!
//! # Pipeline DSL
//!
//! Use the `pipeline!` macro for declarative, type-safe pipeline definitions:
//!
//! ```ignore
//! use rust_buildkite::pipeline;
//!
//! let p = pipeline! {
//!     env: { CI: "true" },
//!     steps: [
//!         command("echo hello").label("Say Hello").key("hello"),
//!         command("npm test").key("tests").depends_on("hello"),
//!         wait,
//!         block("Deploy?"),
//!         command("./deploy.sh").depends_on("tests")
//!     ]
//! };
//! ```

// Re-export the pipeline macro
pub use rust_buildkite_macros::pipeline;

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
