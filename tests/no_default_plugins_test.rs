use rust_buildkite::pipeline;

#[test]
fn a_step_can_opt_out_of_default_plugins() {
    let p = pipeline! {
        allowed_commands: ["echo"],
        default_plugins: [
            { "docker#v5.14.0": { image: "alpine" } }
        ],
        steps: [
            command {
                command: cmd!("echo host"),
                label: "Host step",
                no_default_plugins: true
            },
            command {
                command: cmd!("echo boxed"),
                label: "Container step"
            }
        ]
    };
    let yaml = serde_yaml::to_string(&p).unwrap();
    assert!(yaml.contains("label: Host step"));
    assert!(yaml.contains("label: Container step"));
    // The opting-out step carries no plugins; the other step still inherits the default.
    assert_eq!(yaml.matches("docker#v5.14.0").count(), 1);
    // Opting out must not drop the step's own plugins.
    assert!(yaml.contains("alpine"));
}
