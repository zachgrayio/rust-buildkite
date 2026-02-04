use rust_buildkite::pipeline;

fn main() {
    let _pipeline = pipeline! {
        steps: [
            bazel_build {
                target_patterns: "not-a-valid-target",
                label: "build",
                key: "build"
            }
        ]
    };
}
