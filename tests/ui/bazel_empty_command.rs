use rust_buildkite::pipeline;

fn main() {
    let _pipeline = pipeline! {
        steps: [
            command {
                command: bazel!(""),
                label: "test"
            }
        ]
    };
}
