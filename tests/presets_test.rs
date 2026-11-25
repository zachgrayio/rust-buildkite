use rust_buildkite::pipeline;

#[test]
fn a_step_inherits_a_named_preset() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        presets: {
            primary: {
                env: { SHARED: "from-preset", OVERRIDDEN: "preset-value" },
                cache: { name: "primary", paths: ["target"], size: "100g" },
                timeout_in_minutes: 90
            },
            release: {
                env: { PROFILE_DEBUG: "0" },
                cache: { name: "release", paths: ["target"], size: "100g" },
                timeout_in_minutes: 240
            }
        },
        steps: [
            command {
                command: cmd!("echo a"),
                label: "Inherits",
                preset: "primary"
            },
            command {
                command: cmd!("echo b"),
                label: "Overrides",
                preset: "primary",
                env: { OVERRIDDEN: "step-value" },
                timeout_in_minutes: 5
            },
            command {
                command: cmd!("echo c"),
                label: "Own cache",
                preset: "release"
            }
        ]
    };
    let yaml = serde_yaml::to_string(&p).unwrap();

    // Preset cache, timeout and env arrive on the steps that name them.
    assert_eq!(
        yaml.matches("name: primary\n").count(),
        2,
        "both primary steps"
    );
    assert_eq!(yaml.matches("name: release\n").count(), 1);
    assert_eq!(yaml.matches("size: 100g").count(), 3);
    assert_eq!(yaml.matches("SHARED: from-preset").count(), 2);

    // A step's own env and timeout win: each overridden value appears exactly once.
    assert_eq!(yaml.matches("OVERRIDDEN: preset-value").count(), 1);
    assert_eq!(yaml.matches("OVERRIDDEN: step-value").count(), 1);
    assert_eq!(yaml.matches("timeout_in_minutes: 90").count(), 1);
    assert!(yaml.contains("timeout_in_minutes: 5"));
    assert!(yaml.contains("timeout_in_minutes: 240"));
}

/// `presets:` may be declared after the steps that reference it.
#[test]
fn presets_may_be_declared_after_the_steps() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        steps: [
            command {
                command: cmd!("echo a"),
                label: "Later",
                preset: "late"
            }
        ],
        presets: {
            late: { timeout_in_minutes: 12 }
        }
    };
    let yaml = serde_yaml::to_string(&p).unwrap();
    assert!(yaml.contains("timeout_in_minutes: 12"));
}
