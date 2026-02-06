//! Tests for the cmd! macro and allowed_commands validation
//!
//! This module tests:
//! 1. Basic cmd! usage - bashrs-validated shell commands
//! 2. Allowed commands validation
//! 3. Integration with pipeline! macro
//!
//! Note: cmd! now uses string literals: cmd!("npm install")
//! bashrs validates shell syntax at compile time.

use rust_buildkite::{cmd, pipeline};

mod basic_cmd {
    use super::*;

    #[test]
    fn simple_command() {
        let c = cmd!("echo hello");
        assert_eq!(c, "echo hello");
    }

    #[test]
    fn command_with_flags() {
        let c = cmd!("npm install --save-dev");
        assert_eq!(c, "npm install --save-dev");
    }

    #[test]
    fn command_with_arguments() {
        let c = cmd!("cargo build --release --target x86_64-unknown-linux-gnu");
        assert_eq!(c, "cargo build --release --target x86_64-unknown-linux-gnu");
    }

    #[test]
    fn command_with_path() {
        let c = cmd!("./deploy.sh");
        assert_eq!(c, "./deploy.sh");
    }

    #[test]
    fn command_with_absolute_path() {
        let c = cmd!("/usr/bin/env bash");
        assert_eq!(c, "/usr/bin/env bash");
    }

    #[test]
    fn command_with_pipe() {
        let c = cmd!("ls -la | grep test");
        assert_eq!(c, "ls -la | grep test");
    }

    #[test]
    fn command_with_and() {
        let c = cmd!("npm install && npm test");
        assert_eq!(c, "npm install && npm test");
    }

    #[test]
    fn command_with_or() {
        let c = cmd!("test -f file.txt || touch file.txt");
        assert_eq!(c, "test -f file.txt || touch file.txt");
    }

    #[test]
    fn command_with_semicolon() {
        let c = cmd!("echo hello; echo world");
        assert_eq!(c, "echo hello; echo world");
    }

    #[test]
    fn command_with_redirect() {
        let c = cmd!("echo hello > output.txt");
        assert_eq!(c, "echo hello > output.txt");
    }

    #[test]
    fn command_with_append() {
        let c = cmd!("echo hello >> output.txt");
        assert_eq!(c, "echo hello >> output.txt");
    }

    #[test]
    fn multiline_command_raw_string() {
        let c = cmd!(
            r#"
            set -e
            npm install
            npm test
            npm run build
        "#
        );
        assert!(c.contains("set -e"));
        assert!(c.contains("npm install"));
        assert!(c.contains("npm test"));
        assert!(c.contains("npm run build"));
    }

    #[test]
    fn multiline_command_with_heredoc() {
        let c = cmd!(
            r#"cat <<EOF
Hello World
This is a multiline heredoc
EOF"#
        );
        assert!(c.contains("<<EOF"));
        assert!(c.contains("Hello World"));
    }

    #[test]
    fn multiline_preserves_newlines() {
        let c = cmd!("echo 'line1'\necho 'line2'");
        assert!(c.contains('\n'));
    }

    #[test]
    fn command_with_quoted_string() {
        let c = cmd!("echo 'hello world'");
        assert!(c.contains("hello world"));
    }

    #[test]
    fn command_with_env_var() {
        let c = cmd!("FOO=bar ./script.sh");
        assert_eq!(c, "FOO=bar ./script.sh");
    }
}

mod runtime_interpolation {

    #[test]
    fn runtime_interpolation_without_cmd_macro() {
        let package = "lodash";
        let c = format!("npm install {}", package);
        assert_eq!(c, "npm install lodash");
    }

    #[test]
    fn multiple_runtime_variables() {
        let env = "production";
        let region = "us-east-1";
        let c = format!("./deploy.sh {} {}", env, region);
        assert_eq!(c, "./deploy.sh production us-east-1");
    }
}

mod pipeline_integration {
    use super::*;

    #[test]
    fn command_with_cmd_macro() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm install")).key("install"),
                command(cmd!("npm test")).key("test").depends_on("install")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("npm test"));
    }

    #[test]
    fn object_literal_with_cmd() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("cargo build --release"),
                    label: "Build",
                    key: "build"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("cargo build --release"));
        assert!(yaml.contains("label: Build"));
    }

    #[test]
    fn mixed_syntax_with_cmd() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm install")).key("install"),
                command {
                    command: cmd!("npm test"),
                    key: "test",
                    depends_on: ["install"]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("npm test"));
    }
}

mod allowed_commands {
    use super::*;

    #[test]
    fn valid_commands_in_allowlist() {
        let pipeline = pipeline! {
            allowed_commands: ["npm", "cargo", "echo"],
            steps: [
                command(cmd!("npm install")).key("install"),
                command(cmd!("cargo build")).key("build"),
                command(cmd!("echo done")).key("done")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("cargo build"));
        assert!(yaml.contains("echo done"));
    }

    #[test]
    fn path_command_in_allowlist() {
        // Create the script file so runtime validation passes
        let script_path = std::path::Path::new("./deploy.sh");
        std::fs::write(script_path, "#!/bin/bash\necho deploy").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(script_path, std::fs::Permissions::from_mode(0o755)).unwrap();
        }

        let pipeline = pipeline! {
            allowed_commands: ["./deploy.sh", "/usr/bin/env"],
            expect_paths: ["./deploy.sh"],
            steps: [
                command(cmd!("./deploy.sh production")).key("deploy"),
                command(cmd!("/usr/bin/env bash -c 'echo hi'")).key("bash")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("./deploy.sh production"));

        // Clean up
        let _ = std::fs::remove_file(script_path);
    }

    #[test]
    fn hyphenated_command_in_allowlist() {
        let pipeline = pipeline! {
            allowed_commands: ["docker-compose"],
            steps: [
                command(cmd!("docker-compose up -d")).key("compose")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("docker-compose up -d"));
    }

    #[test]
    fn allowlist_with_object_literal() {
        let pipeline = pipeline! {
            allowed_commands: ["npm"],
            steps: [
                command {
                    command: cmd!("npm test"),
                    label: "Test",
                    key: "test"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("npm test"));
    }

    #[test]
    fn allowlist_in_group_steps() {
        let pipeline = pipeline! {
            allowed_commands: ["npm", "cargo"],
            steps: [
                group {
                    group: "Build",
                    key: "build-group",
                    steps: [
                        command(cmd!("npm install")).key("install"),
                        command(cmd!("cargo build")).key("build")
                    ]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("cargo build"));
    }

    #[test]
    fn default_uses_host_path_commands() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo hello")).key("echo"),
                command(cmd!("npm install")).key("npm")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("echo hello"));
        assert!(yaml.contains("npm install"));
    }
}

mod expect_env {
    use super::*;

    #[test]
    fn env_vars_from_pipeline_env_block() {
        let pipeline = pipeline! {
            env: {
                MY_VAR: "value"
            },
            steps: [
                command(cmd!(r#"echo "$MY_VAR""#)).key("test")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$MY_VAR"));
    }

    #[test]
    fn env_vars_from_step_env() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!(r#"echo "$STEP_VAR""#))
                    .key("test")
                    .env(STEP_VAR, "value")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$STEP_VAR"));
    }

    #[test]
    fn expect_env_permits_vars() {
        let pipeline = pipeline! {
            expect_env: ["HOME", "PATH", "USER"],
            steps: [
                command(cmd!(r#"echo "$HOME" "$PATH" "$USER""#)).key("test")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$HOME"));
    }

    #[test]
    fn combined_env_sources() {
        let pipeline = pipeline! {
            env: {
                PIPELINE_VAR: "p"
            },
            expect_env: ["ALLOWED_VAR"],
            steps: [
                command(cmd!(r#"echo "$PIPELINE_VAR" "$STEP_VAR" "$ALLOWED_VAR""#))
                    .key("test")
                    .env(STEP_VAR, "s")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$PIPELINE_VAR"));
        assert!(yaml.contains("$STEP_VAR"));
        assert!(yaml.contains("$ALLOWED_VAR"));
    }

    #[test]
    fn shell_builtins_allowed() {
        let pipeline = pipeline! {
            expect_env: ["HOME", "PROJECT_DIR"],
            steps: [
                command(cmd!(r#"cd "$HOME" && pwd"#)).key("cd"),
                command(cmd!("export FOO=bar")).key("export"),
                command(cmd!("read -r LINE")).key("read"),
                command(cmd!("set -e")).key("set"),
                command(cmd!(r#"source "$PROJECT_DIR/env.sh""#)).key("source"),
                command(cmd!("test -f /etc/passwd")).key("test"),
                command(cmd!("[ -d /tmp ]")).key("bracket")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains(r#"cd "$HOME" && pwd"#));
        assert!(yaml.contains("export FOO=bar"));
        assert!(yaml.contains("read -r LINE"));
        assert!(yaml.contains("set -e"));
        assert!(yaml.contains("source"));
        assert!(yaml.contains("test -f"));
        assert!(yaml.contains("[ -d"));
    }

    #[test]
    fn default_to_host_env_when_expect_env_not_specified() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!(r#"echo "$HOME""#)).key("home"),
                command(cmd!(r#"echo "$PATH""#)).key("path")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$HOME"));
        assert!(yaml.contains("$PATH"));
    }
}

mod bashrs_behavior {
    use super::*;

    #[test]
    fn simple_var_works() {
        let c = cmd!(r#"echo "$VAR""#);
        assert!(c.contains("$VAR"));
    }

    #[test]
    fn braced_var_works() {
        let c = cmd!(r#"echo "${VAR}""#);
        assert!(c.contains("${VAR}"));
    }

    #[test]
    fn var_with_default_works() {
        let c = cmd!(r#"echo "${VAR:-default}""#);
        assert!(c.contains("${VAR:-default}"));
    }

    #[test]
    fn var_with_default_empty_string() {
        let c = cmd!(r#"echo "${VAR:-}""#);
        assert!(c.contains("${VAR:-}"));
    }

    #[test]
    fn var_with_default_in_pipeline() {
        let pipeline = pipeline! {
            expect_env: ["VAR"],
            steps: [
                command(cmd!(r#"echo "${VAR:-fallback}""#)).key("test")
            ]
        };
        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("${VAR:-fallback}"));
    }

    #[test]
    fn simple_var_in_pipeline() {
        let pipeline = pipeline! {
            expect_env: ["VAR"],
            steps: [
                command(cmd!(r#"echo "$VAR""#)).key("test")
            ]
        };
        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("$VAR"));
    }

    #[test]
    fn braced_var_in_pipeline() {
        let pipeline = pipeline! {
            expect_env: ["VAR"],
            steps: [
                command(cmd!(r#"echo "${VAR}""#)).key("test")
            ]
        };
        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("${VAR}"));
    }

    #[test]
    fn simple_git_checkout() {
        // Simple commands like git checkout "$VAR" -- file work fine
        let c = cmd!(r#"git checkout "$BRANCH" -- file.txt"#);
        assert!(c.contains("git checkout"));
    }

    #[test]
    fn echo_multiple_vars() {
        let c = cmd!(r#"echo "$FOO" "$BAR""#);
        assert!(c.contains("$FOO"));
        assert!(c.contains("$BAR"));
    }

    // =========================================================================
    // BASHRS FALSE POSITIVES - WORKAROUNDS THAT WORK
    // =========================================================================

    // jq: Avoid $ in single quotes - use field access without $
    #[test]
    fn jq_workaround_no_dollar() {
        let c = cmd!(r#"jq '.foo' file.json"#);
        assert!(c.contains("jq"));
    }

    // jq: Use double quotes with escaping
    #[test]
    fn jq_workaround_double_quotes() {
        let c = cmd!(r#"jq "{\"field\": .value}" file.json"#);
        assert!(c.contains("jq"));
    }

    // date: Pipe to xargs avoids DET002 entirely (no command substitution)
    #[test]
    fn date_workaround_pipe_xargs() {
        let c = cmd!(r#"date +%Y-%m-%d | xargs -I{} echo "Today is {}""#);
        assert!(c.contains("date"));
    }

    // Heredoc without $ in content works fine
    #[test]
    fn heredoc_without_dollar_works() {
        let c = cmd!(
            r#"cat <<EOF
Hello World
No dollar signs here
EOF"#
        );
        assert!(c.contains("<<EOF"));
    }

    // todo: find a solution for these false positives in bashrs.

    // FAILS: [SC2086] [SC2128]
    // #[test]
    // fn heredoc_with_dollar_fails() {
    //     let c = cmd!(r#"cat <<EOF
    // Text with $variable
    // EOF"#);
    //     assert!(c.contains("<<EOF"));
    // }

    // FAILS: [SC2086] [SC2128] - quoted delimiter doesn't help
    // #[test]
    // fn heredoc_quoted_delimiter_fails() {
    //     let c = cmd!(r#"cat <<'EOF'
    // Text with $variable
    // EOF"#);
    //     assert!(c.contains("<<'EOF'"));
    // }

    // FAILS: [SC2086] [SC2128] - escaping doesn't help
    // #[test]
    // fn heredoc_escaped_dollar_fails() {
    //     let c = cmd!(r#"cat <<EOF
    // Text with \$variable
    // EOF"#);
    //     assert!(c.contains("<<EOF"));
    // }

    // FAILS: [SC2086] - bashrs doesn't understand single quote semantics
    // #[test]
    // fn jq_dollar_in_single_quotes_fails() {
    //     let c = cmd!(r#"jq '{$var}' file.json"#);
    //     assert!(c.contains("jq"));
    // }

    // FAILS: [SC2086] - escaping doesn't help either
    // #[test]
    // fn jq_escaped_dollar_fails() {
    //     let c = cmd!(r#"jq '{\$var}' file.json"#);
    //     assert!(c.contains("jq"));
    // }

    // FAILS: [SC2086] - --arg still has $ in single quotes
    // #[test]
    // fn jq_with_arg_fails() {
    //     let c = cmd!(r#"jq --arg v "$VAR" '{($v): .value}' file.json"#);
    //     assert!(c.contains("jq"));
    // }

    // FAILS: [SC2046] [DET002]
    // #[test]
    // fn cmd_subst_date_fails() {
    //     let c = cmd!(r#"echo "Today is $(date +%Y-%m-%d)""#);
    //     assert!(c.contains("$(date"));
    // }

    // FAILS: [SC2036] [SC2046] [DET002] - backticks are worse
    // #[test]
    // fn cmd_subst_backticks_fails() {
    //     let c = cmd!(r#"echo "Today is `date +%Y-%m-%d`""#);
    //     assert!(c.contains("date"));
    // }

    // FAILS: [DET002] - even properly quoted, DET002 fires
    // #[test]
    // fn cmd_subst_quoted_fails() {
    //     let c = cmd!(r#"echo "Today is" "$(date +%Y-%m-%d)""#);
    //     assert!(c.contains("date"));
    // }

    // FAILS: [SC2046] [DET002] - variable assignment doesn't help
    // #[test]
    // fn cmd_subst_via_variable_fails() {
    //     let c = cmd!(r#"TODAY=$(date +%Y-%m-%d); echo "Today is $TODAY""#);
    //     assert!(c.contains("date"));
    // }

    // FAILS: [DET002] - printf doesn't help
    // #[test]
    // fn cmd_subst_printf_fails() {
    //     let c = cmd!(r#"printf "Today is %s\n" "$(date +%Y-%m-%d)""#);
    //     assert!(c.contains("date"));
    // }
}

mod pipeline_properties {
    use super::*;

    #[test]
    fn notify_slack() {
        let p = pipeline! {
            notify: [{ slack: "#builds" }],
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("slack"));
    }

    #[test]
    fn notify_slack_with_if() {
        let p = pipeline! {
            notify: [{ slack: "#builds", r#if: "build.state == 'failed'" }],
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("slack"));
        assert!(yaml.contains("if"));
    }

    #[test]
    fn notify_email() {
        let p = pipeline! {
            notify: [{ email: "team@example.com" }],
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("email"));
    }

    #[test]
    fn agents_object() {
        let p = pipeline! {
            agents: { queue: "deploy" },
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("queue"));
    }

    #[test]
    fn image_string() {
        let p = pipeline! {
            image: "node:18-alpine",
            steps: [command(cmd!("npm test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("node:18-alpine"));
    }

    #[test]
    fn secrets_array() {
        let p = pipeline! {
            secrets: ["API_KEY"],
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("API_KEY"));
    }

    #[test]
    fn secrets_object() {
        let p = pipeline! {
            secrets: { MY_KEY: "API_KEY" },
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("MY_KEY"));
    }

    #[test]
    fn priority_integer() {
        let p = pipeline! {
            priority: 10,
            steps: [command(cmd!("echo test")).key("test")]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("priority"));
    }
}

/// Tests for BUILDKITE_SKIP_COMPTIME_VALIDATION with shell commands.
mod skip_comptime_validation {
    use super::*;

    #[test]
    fn cmd_macro_still_works() {
        let c = cmd!("echo hello world");
        assert_eq!(c, "echo hello world");
    }

    #[test]
    fn cmd_with_path_still_works() {
        // Path validation happens at runtime when skip_comptime is set
        let c = cmd!("./some_script.sh --flag");
        assert_eq!(c, "./some_script.sh --flag");
    }

    #[test]
    fn pipeline_with_commands() {
        let p = pipeline! {
            steps: [
                command(cmd!("npm install")).key("install"),
                command(cmd!("npm test")).key("test")
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("npm test"));
    }
}
