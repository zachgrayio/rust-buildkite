//! Comprehensive tests for the pipeline! macro
//!
//! This module tests:
//! 1. Basic object-literal syntax for all step types
//! 2. Fluent syntax for all step types
//! 3. All fields for all step types
//! 4. Mixed syntax pipelines
//! 5. Complex real-world scenarios
//!
//! Note: Parity tests between macro and builder API are in parity.rs

use rust_buildkite::pipeline;

mod object_literal {
    use super::*;

    #[test]
    fn command_basic() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("echo hello"),
                    label: "Say Hello",
                    key: "hello"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("command: echo hello"));
        assert!(yaml.contains("label: Say Hello"));
        assert!(yaml.contains("key: hello"));
    }

    #[test]
    fn command_with_env() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    label: "Tests",
                    env: {
                        NODE_ENV: "test",
                        CI: "true"
                    }
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("NODE_ENV: test"));
        assert!(yaml.contains("CI: 'true'"));
    }

    #[test]
    fn command_with_depends_on() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("echo first"),
                    key: "first"
                },
                command {
                    command: cmd!("echo second"),
                    depends_on: ["first"]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("- first"));
    }

    #[test]
    fn command_with_agents_branches_cache() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    agents: { queue: "default", os: "linux" },
                    branches: ["main", "release/*"],
                    cache: ["node_modules", ".npm"],
                    condition: "build.branch == 'main'"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("agents:"));
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("cache:"));
        assert!(yaml.contains("if: build.branch == 'main'"));
    }

    #[test]
    fn command_with_retry_and_plugins() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    retry: {
                        automatic: { limit: 3 },
                        manual: { allowed: true }
                    },
                    plugins: [
                        { "docker#v5.0.0": { image: "node:18" } }
                    ]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("retry:"));
        assert!(yaml.contains("plugins:"));
    }

    #[test]
    fn default_plugins_applied_to_all_command_steps() {
        let pipeline = pipeline! {
            default_plugins: [
                { "vault-secrets#v2.4.0": { server: "https://vault.example.com", path: "secret/ci" } }
            ],
            steps: [
                command {
                    command: cmd!("npm install"),
                    label: "install"
                },
                command {
                    command: cmd!("npm test"),
                    label: "test"
                },
                wait,
                command {
                    command: cmd!("npm run deploy"),
                    label: "deploy"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        let count = yaml.matches("vault-secrets#v2.4.0").count();
        assert_eq!(
            count, 3,
            "vault-secrets plugin should be applied to all 3 command steps"
        );
        assert!(yaml.contains("https://vault.example.com"));
    }

    #[test]
    fn default_plugins_merged_with_step_plugins() {
        let pipeline = pipeline! {
            default_plugins: [
                { "vault-secrets#v2.4.0": { server: "https://vault.example.com" } }
            ],
            steps: [
                command {
                    command: cmd!("npm test"),
                    label: "test",
                    plugins: [
                        { "docker#v5.0.0": { image: "node:18" } }
                    ]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("vault-secrets#v2.4.0"));
        assert!(yaml.contains("docker#v5.0.0"));
    }

    #[test]
    fn default_plugins_not_applied_to_non_command_steps() {
        let pipeline = pipeline! {
            default_plugins: [
                { "vault-secrets#v2.4.0": { server: "https://vault.example.com" } }
            ],
            steps: [
                command {
                    command: cmd!("npm test"),
                    label: "test"
                },
                wait,
                block("Deploy to production?")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        let count = yaml.matches("vault-secrets#v2.4.0").count();
        assert_eq!(
            count, 1,
            "vault-secrets plugin should only be applied to command step"
        );
    }

    #[test]
    fn command_with_timeout_parallelism_artifacts() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    timeout_in_minutes: 30,
                    parallelism: 4,
                    artifact_paths: ["coverage/**/*", "test-results/*.xml"]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("timeout_in_minutes: 30"));
        assert!(yaml.contains("parallelism: 4"));
        assert!(yaml.contains("artifact_paths:"));
    }

    #[test]
    fn command_with_concurrency_skip_priority() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("echo deploy"),
                    concurrency: 1,
                    concurrency_group: "deploy/prod",
                    skip: "Temporarily disabled",
                    priority: 10
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("concurrency: 1"));
        assert!(yaml.contains("concurrency_group: deploy/prod"));
        assert!(yaml.contains("skip: Temporarily disabled"));
        assert!(yaml.contains("priority: 10"));
    }

    #[test]
    fn command_with_soft_fail_and_allow_dependency_failure() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    soft_fail: true,
                    allow_dependency_failure: true
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("soft_fail: true"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }

    #[test]
    fn command_with_matrix() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    matrix: ["node:16", "node:18", "node:20"]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("matrix:"));
    }

    #[test]
    fn command_with_notify() {
        let pipeline = pipeline! {
            steps: [
                command {
                    command: cmd!("npm test"),
                    notify: [
                        { slack: "#builds" }
                    ]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("notify:"));
        assert!(yaml.contains("slack:"));
    }

    #[test]
    fn block_basic() {
        let pipeline = pipeline! {
            steps: [
                block {
                    block: "Deploy?",
                    key: "approval"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("block: Deploy?"));
        assert!(yaml.contains("key: approval"));
    }

    #[test]
    fn block_with_fields() {
        let pipeline = pipeline! {
            steps: [
                block {
                    block: "Deploy?",
                    fields: [
                        text { key: "reason", text: "Reason", required: true, hint: "Enter reason", default: "None", format: "[a-z]+" },
                        select {
                            key: "env",
                            select: "Environment",
                            hint: "Select target",
                            required: true,
                            multiple: false,
                            default: "staging",
                            options: [
                                { label: "Staging", value: "staging" },
                                { label: "Prod", value: "prod" }
                            ]
                        }
                    ],
                    allowed_teams: ["admins"],
                    blocked_state: "running"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("fields:"));
        assert!(yaml.contains("allowed_teams:"));
        assert!(yaml.contains("blocked_state: running"));
        assert!(yaml.contains("hint:"));
    }

    #[test]
    fn block_with_branches_prompt_allow_dependency_failure() {
        let pipeline = pipeline! {
            steps: [
                block {
                    block: "Deploy?",
                    branches: ["main", "release/*"],
                    prompt: "Are you sure you want to deploy?",
                    allow_dependency_failure: true,
                    r#if: "build.branch == 'main'"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("prompt: Are you sure you want to deploy?"));
        assert!(yaml.contains("allow_dependency_failure: true"));
        assert!(yaml.contains("if: build.branch == 'main'"));
    }

    #[test]
    fn input_basic() {
        let pipeline = pipeline! {
            steps: [
                input {
                    input: "Enter version",
                    key: "version"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("input: Enter version"));
    }

    #[test]
    fn input_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                input {
                    input: "Enter details",
                    key: "details",
                    fields: [
                        text { key: "version", text: "Version" },
                        select {
                            key: "region",
                            select: "Region",
                            options: [
                                { label: "US", value: "us" },
                                { label: "EU", value: "eu" }
                            ]
                        }
                    ],
                    allowed_teams: ["platform"],
                    blocked_state: "running",
                    branches: ["main"],
                    prompt: "Please fill in the details",
                    allow_dependency_failure: true,
                    r#if: "build.branch == 'main'"
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("input: Enter details"));
        assert!(yaml.contains("fields:"));
        assert!(yaml.contains("allowed_teams:"));
        assert!(yaml.contains("blocked_state: running"));
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("prompt:"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }

    #[test]
    fn trigger_basic() {
        let pipeline = pipeline! {
            steps: [
                trigger {
                    trigger: "deploy",
                    label: "Deploy",
                    soft_fail: true
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("trigger: deploy"));
        assert!(yaml.contains("soft_fail: true"));
    }

    #[test]
    fn trigger_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command { command: cmd!("echo build"), key: "build" },
                trigger {
                    trigger: "deploy-service",
                    label: "Deploy",
                    key: "deploy",
                    r#async: true,
                    depends_on: ["build"],
                    build: {
                        branch: "main",
                        commit: "HEAD",
                        message: "Triggered deploy",
                        env: { TARGET: "prod" },
                        meta_data: { deployment_id: "123" }
                    },
                    branches: ["main", "release/*"],
                    r#if: "build.branch == 'main'",
                    skip: false,
                    soft_fail: true,
                    allow_dependency_failure: true
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("trigger: deploy-service"));
        assert!(yaml.contains("async: true"));
        assert!(yaml.contains("build:"));
        assert!(yaml.contains("branch: main"));
        assert!(yaml.contains("commit: HEAD"));
        assert!(yaml.contains("message: Triggered deploy"));
        assert!(yaml.contains("meta_data:"));
    }

    #[test]
    fn wait_simple() {
        let pipeline = pipeline! {
            steps: [
                command { command: cmd!("echo test") },
                wait
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("wait"));
    }

    #[test]
    fn wait_with_options() {
        let pipeline = pipeline! {
            steps: [
                wait { continue_on_failure: true, r#if: "build.branch == 'main'" }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("continue_on_failure: true"));
        assert!(yaml.contains("if: build.branch == 'main'"));
    }

    #[test]
    fn group_basic() {
        let pipeline = pipeline! {
            steps: [
                group {
                    group: "Tests",
                    key: "tests",
                    steps: [
                        command { command: cmd!("npm test") }
                    ]
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("group: Tests"));
        assert!(yaml.contains("key: tests"));
    }

    #[test]
    fn group_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command { command: cmd!("npm run unit"), key: "unit-tests" },
                group {
                    group: "Integration Tests",
                    key: "integration",
                    depends_on: ["unit-tests"],
                    steps: [
                        command { command: cmd!("npm run integration") }
                    ],
                    r#if: "build.branch == 'main'",
                    skip: false,
                    notify: [
                        { slack: "#builds" }
                    ],
                    allow_dependency_failure: true
                }
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("group: Integration Tests"));
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("if: build.branch == 'main'"));
        assert!(yaml.contains("notify:"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }
}

mod fluent {
    use super::*;

    #[test]
    fn command_basic() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo hello")).label("Hello").key("hello")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("command: echo hello"));
        assert!(yaml.contains("label: Hello"));
    }

    #[test]
    fn command_with_env() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .env(NODE_ENV, "test")
                    .env(CI, "true")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("env:"));
    }

    #[test]
    fn command_with_agents() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo deploy")).agents({ queue: "deploy" })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("agents:"));
    }

    #[test]
    fn command_with_retry() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo test")).retry_automatic(3)
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("retry:"));
        assert!(yaml.contains("limit: 3"));
    }

    #[test]
    fn command_with_concurrency() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo deploy"))
                    .concurrency(1)
                    .concurrency_group("deploy/prod")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("concurrency: 1"));
        assert!(yaml.contains("concurrency_group: deploy/prod"));
    }

    #[test]
    fn command_with_skip_and_priority() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo test"))
                    .skip("Temporarily disabled")
                    .priority(10)
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("skip: Temporarily disabled"));
        assert!(yaml.contains("priority: 10"));
    }

    #[test]
    fn command_with_timeout_parallelism_artifacts() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .timeout_in_minutes(30)
                    .parallelism(4)
                    .artifact_paths("coverage/**/*")
                    .artifact_paths("test-results/*.xml")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("timeout_in_minutes: 30"));
        assert!(yaml.contains("parallelism: 4"));
        assert!(yaml.contains("artifact_paths:"));
    }

    #[test]
    fn command_with_branches_cache_if() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .branches("main")
                    .branches("release/*")
                    .cache("node_modules")
                    .r#if("build.branch == 'main'")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("cache:"));
        assert!(yaml.contains("if: build.branch == 'main'"));
    }

    #[test]
    fn command_with_depends_on() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm install")).key("install"),
                command(cmd!("npm test"))
                    .depends_on("install")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("depends_on:"));
    }

    #[test]
    fn command_with_soft_fail_and_allow_dependency_failure() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .soft_fail()
                    .allow_dependency_failure()
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("soft_fail: true"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }

    #[test]
    fn command_with_matrix() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .matrix(["node:16", "node:18", "node:20"])
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("matrix:"));
    }

    #[test]
    fn command_with_plugin() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .plugin("docker#v5.0.0", { image: "node:18" })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("plugins:"));
        assert!(yaml.contains("docker#v5.0.0"));
    }

    #[test]
    fn command_with_notify_slack() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .notify_slack("#builds")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("notify:"));
        assert!(yaml.contains("slack:"));
    }

    #[test]
    fn command_with_retry_full() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm test"))
                    .retry({ automatic: { limit: 3 }, manual: { allowed: true } })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("retry:"));
        assert!(yaml.contains("automatic:"));
        assert!(yaml.contains("manual:"));
    }

    #[test]
    fn block_basic() {
        let pipeline = pipeline! {
            steps: [
                block("Approve?").key("approval")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("block: Approve?"));
    }

    #[test]
    fn block_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm run build")).key("build"),
                block("Deploy?")
                    .key("approval")
                    .depends_on("build")
                    .allowed_teams("admins")
                    .blocked_state("running")
                    .branches("main")
                    .r#if("build.branch == 'main'")
                    .prompt("Are you sure?")
                    .allow_dependency_failure()
                    .field(text { key: "reason", text: "Reason" })
                    .field(select { key: "env", select: "Environment", options: [{ label: "Prod", value: "prod" }] })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("block: Deploy?"));
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("allowed_teams:"));
        assert!(yaml.contains("blocked_state: running"));
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("prompt: Are you sure?"));
        assert!(yaml.contains("allow_dependency_failure: true"));
        assert!(yaml.contains("fields:"));
    }

    #[test]
    fn input_basic() {
        let pipeline = pipeline! {
            steps: [
                input("Enter version").key("version")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("input: Enter version"));
    }

    #[test]
    fn input_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm run build")).key("build"),
                input("Enter details")
                    .key("details")
                    .depends_on("build")
                    .allowed_teams("platform")
                    .blocked_state("running")
                    .branches("main")
                    .r#if("build.branch == 'main'")
                    .prompt("Fill in the form")
                    .allow_dependency_failure()
                    .field(text { key: "version", text: "Version" })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("input: Enter details"));
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("allowed_teams:"));
        assert!(yaml.contains("prompt: Fill in the form"));
    }

    #[test]
    fn trigger_basic() {
        let pipeline = pipeline! {
            steps: [
                trigger("deploy").label("Deploy")
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("trigger: deploy"));
    }

    #[test]
    fn trigger_with_async_and_build() {
        let pipeline = pipeline! {
            steps: [
                trigger("deploy")
                    .r#async()
                    .build({
                        branch: "main",
                        message: "Auto deploy",
                        env: { TARGET: "prod" }
                    })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("async: true"));
        assert!(yaml.contains("build:"));
        assert!(yaml.contains("branch: main"));
    }

    #[test]
    fn trigger_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm run build")).key("build"),
                trigger("deploy-service")
                    .key("deploy")
                    .label("Deploy")
                    .depends_on("build")
                    .r#async()
                    .build({
                        branch: "main",
                        commit: "HEAD",
                        message: "Deploy",
                        env: { TARGET: "prod" },
                        meta_data: { id: "123" }
                    })
                    .branches("main")
                    .r#if("build.branch == 'main'")
                    .skip("Disabled")
                    .soft_fail()
                    .allow_dependency_failure()
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("trigger: deploy-service"));
        assert!(yaml.contains("key: deploy"));
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("async: true"));
        assert!(yaml.contains("build:"));
        assert!(yaml.contains("branches:"));
        assert!(yaml.contains("skip: Disabled"));
        assert!(yaml.contains("soft_fail: true"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }

    #[test]
    fn group_basic() {
        let pipeline = pipeline! {
            steps: [
                group("Tests")
                    .key("tests")
                    .r#if("build.branch == 'main'")
                    .steps([])
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("group: Tests"));
        assert!(yaml.contains("if: build.branch == 'main'"));
    }

    #[test]
    fn group_comprehensive() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("npm run unit")).key("unit"),
                group("Integration")
                    .key("integration")
                    .depends_on("unit")
                    .steps([
                        command(cmd!("npm run integration")).key("int-test")
                    ])
                    .r#if("build.branch == 'main'")
                    .skip("Disabled")
                    .notify_slack("#builds")
                    .allow_dependency_failure()
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("group: Integration"));
        assert!(yaml.contains("depends_on:"));
        assert!(yaml.contains("skip: Disabled"));
        assert!(yaml.contains("notify:"));
        assert!(yaml.contains("allow_dependency_failure: true"));
    }
}

mod complex {
    use super::*;

    #[test]
    fn mixed_syntax_pipeline() {
        let pipeline = pipeline! {
            env: {
                CI: "true",
                NODE_ENV: "test"
            },
            steps: [
                command(cmd!("npm install")).label("Install").key("install"),
                command {
                    command: cmd!("npm test"),
                    label: "Test",
                    key: "test",
                    depends_on: ["install"]
                },
                wait,
                block {
                    block: "Deploy?",
                    key: "approval"
                },
                trigger("deploy").r#async()
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("CI:"));
        assert!(yaml.contains("npm install"));
        assert!(yaml.contains("npm test"));
        assert!(yaml.contains("wait"));
        assert!(yaml.contains("block: Deploy?"));
        assert!(yaml.contains("trigger: deploy"));
    }

    #[test]
    fn realistic_cicd_pipeline() {
        let pipeline = pipeline! {
            env: { CI: "true" },
            steps: [
                command {
                    command: cmd!("cargo fmt --check && cargo clippy"),
                    label: ":rust: Lint",
                    key: "lint"
                },
                command {
                    command: cmd!("cargo test"),
                    label: ":test_tube: Tests",
                    key: "test",
                    retry: { automatic: { limit: 2 } },
                    artifact_paths: ["target/test-results/**/*"]
                },
                command {
                    command: cmd!("cargo build --release"),
                    label: ":package: Build",
                    key: "build",
                    depends_on: ["lint", "test"]
                },
                wait,
                block {
                    block: "Deploy to Production?",
                    key: "prod-approval",
                    allowed_teams: ["platform-team"],
                    fields: [
                        select {
                            key: "region",
                            select: "Region",
                            options: [
                                { label: "US East", value: "us-east-1" },
                                { label: "EU West", value: "eu-west-1" }
                            ]
                        }
                    ]
                },
                trigger("deploy-service")
                    .label(":rocket: Deploy")
                    .build({ branch: "main", env: { TARGET: "production" } })
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("cargo fmt"));
        assert!(yaml.contains("cargo test"));
        assert!(yaml.contains("cargo build"));
        assert!(yaml.contains("Deploy to Production?"));
        assert!(yaml.contains("deploy-service"));
    }

    #[test]
    fn all_step_types() {
        let pipeline = pipeline! {
            steps: [
                command(cmd!("echo hello")).key("cmd"),
                wait,
                block("Approve?").key("block"),
                input("Enter value").key("input"),
                trigger("other-pipeline").key("trigger"),
                group("Test Group").key("group").steps([])
            ]
        };

        let yaml = serde_yaml::to_string(&pipeline).unwrap();
        assert!(yaml.contains("command:"));
        assert!(yaml.contains("wait"));
        assert!(yaml.contains("block:"));
        assert!(yaml.contains("input:"));
        assert!(yaml.contains("trigger:"));
        assert!(yaml.contains("group:"));
    }
}
