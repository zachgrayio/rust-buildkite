//! A Rust library for the Buildkite API.
//!
//! This library provides a client for the Buildkite API with typed requests and responses.

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
