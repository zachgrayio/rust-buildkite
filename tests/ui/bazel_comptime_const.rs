#![allow(unused_imports)]

use rust_buildkite::{comptime, pipeline};

const TARGETS: &str = "//app:main //lib:core";

fn main() {
    let _pipeline = pipeline! {
        steps: [
            bazel_build {
                target_patterns: comptime!(TARGETS),
                label: "build",
                key: "build"
            }
        ]
    };
}
