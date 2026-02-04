#![allow(unused_imports)]

use rust_buildkite::{pipeline, runtime};

fn main() {
    let targets = "this-is-not-a-valid-target-pattern";
    let _pipeline = pipeline! {
        steps: [
            bazel_build {
                target_patterns: runtime!(targets),
                label: "build",
                key: "build"
            }
        ]
    };
}
