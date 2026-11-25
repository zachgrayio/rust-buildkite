use rust_buildkite::pipeline;

/// `if_changed` is an agent-applied attribute: the agent evaluates the glob against
/// `git diff --merge-base <base>` during `pipeline upload`. The macro only has to emit it.
#[test]
fn if_changed_accepts_a_glob_a_list_and_include_exclude() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        steps: [
            command {
                command: cmd!("echo a"),
                label: "Single glob",
                if_changed: "gdd/**"
            },
            command {
                command: cmd!("echo b"),
                label: "Pattern list",
                if_changed: ["gdd/**", "tools/ci/Dockerfile"]
            },
            command {
                command: cmd!("echo c"),
                label: "Include and exclude",
                if_changed: {
                    include: "**",
                    exclude: ["context/**", "**.md"]
                }
            }
        ]
    };
    let yaml = serde_yaml::to_string(&p).unwrap();

    // One occurrence per step, and every pattern survives.
    assert_eq!(yaml.matches("if_changed:").count(), 3, "{yaml}");
    assert!(yaml.contains("gdd/**"), "{yaml}");
    assert!(yaml.contains("tools/ci/Dockerfile"), "{yaml}");
    assert!(yaml.contains("include:"), "{yaml}");
    assert!(yaml.contains("exclude:"), "{yaml}");
    assert!(yaml.contains("context/**"), "{yaml}");
    assert!(yaml.contains("**.md"), "{yaml}");
}

/// A step without the attribute emits nothing for it.
#[test]
fn a_step_without_if_changed_omits_the_key() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        steps: [
            command {
                command: cmd!("echo a"),
                label: "Always"
            }
        ]
    };
    let yaml = serde_yaml::to_string(&p).unwrap();
    assert!(!yaml.contains("if_changed"), "{yaml}");
}
