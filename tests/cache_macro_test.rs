use rust_buildkite::pipeline;

#[test]
fn command_with_cache_object_string_and_array() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        steps: [
            command {
                command: cmd!("echo a"),
                label: "Single path",
                cache: "target"
            },
            command {
                command: cmd!("echo b"),
                label: "Path array",
                cache: ["target", "tools/ci/target"]
            },
            command {
                command: cmd!("echo c"),
                label: "Named sized volume",
                cache: {
                    name: "bard-gdd",
                    paths: ["target"],
                    size: "100g"
                }
            }
        ]
    };
    let yaml = serde_yaml::to_string(&p).unwrap();
    assert!(yaml.contains("cache: target"));
    assert!(yaml.contains("name: bard-gdd"));
    assert!(yaml.contains("size: 100g"));
    assert!(yaml.contains("- tools/ci/target"));
}
