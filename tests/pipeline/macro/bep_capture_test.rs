//! Tests for the BEP capture flag injection that the `pipeline!` macro
//! performs on every supported `bazel_*` step.
//!
//! Two pipeline-level booleans control the injection (both default `true`):
//!   * `use_buildkite_job_invocation_id`     -> `--invocation_id=$$BUILDKITE_JOB_ID`
//!   * `set_build_event_binary_file_path`    -> `--build_event_binary_file=$$BUILDKITE_BUILD_PATH/bep/bep-$$BUILDKITE_JOB_ID.pb`
//!
//! Injection only fires for verbs that produce a useful Bazel invocation
//! (`build`/`test`/`run`/`coverage`/`cquery`/`aquery`); metadata verbs like
//! `info` / `version` / `mod` are skipped.

#![allow(unused_imports)]

use rust_buildkite::pipeline;

const INVOCATION_ID_FLAG: &str = "--invocation_id=$$BUILDKITE_JOB_ID";
const BEP_FILE_FLAG: &str =
    "--build_event_binary_file=$$BUILDKITE_BUILD_PATH/bep/bep-$$BUILDKITE_JOB_ID.pb";

mod defaults {
    use super::*;

    #[test]
    fn bazel_build_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "build all",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(
            yaml.contains(INVOCATION_ID_FLAG),
            "expected --invocation_id flag in:\n{yaml}"
        );
        assert!(
            yaml.contains(BEP_FILE_FLAG),
            "expected --build_event_binary_file flag in:\n{yaml}"
        );
    }

    #[test]
    fn bazel_test_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    label: "test all",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn bazel_run_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_run {
                    target_patterns: "//foo:bar",
                    label: "run",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn bazel_coverage_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_coverage {
                    target_patterns: "//...",
                    label: "coverage",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn bazel_cquery_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_cquery {
                    target_patterns: "//...",
                    label: "cquery",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn bazel_aquery_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_aquery {
                    target_patterns: "//...",
                    label: "aquery",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn invocation_id_uses_literal_buildkite_job_id() {
        // The literal `$$BUILDKITE_JOB_ID` reference must survive codegen
        // intact: Buildkite's bash expands it before Bazel parses argv.
        let p = pipeline! {
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(
            yaml.contains("--invocation_id=$$BUILDKITE_JOB_ID"),
            "expected literal $$BUILDKITE_JOB_ID:\n{yaml}"
        );
    }

    #[test]
    fn bep_file_path_uses_literal_buildkite_env_vars() {
        let p = pipeline! {
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(
            yaml.contains("$$BUILDKITE_BUILD_PATH/bep/bep-$$BUILDKITE_JOB_ID.pb"),
            "expected literal $$BUILDKITE_BUILD_PATH and $$BUILDKITE_JOB_ID:\n{yaml}"
        );
    }
}

mod verb_gating {
    use super::*;

    #[test]
    fn bazel_command_info_skips_flags() {
        let p = pipeline! {
            steps: [
                bazel_command {
                    verb: "info",
                    label: "info",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG), "info must skip:\n{yaml}");
        assert!(!yaml.contains(BEP_FILE_FLAG), "info must skip:\n{yaml}");
    }

    #[test]
    fn bazel_command_version_skips_flags() {
        let p = pipeline! {
            steps: [
                bazel_command {
                    verb: "version",
                    label: "version",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG));
        assert!(!yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn bazel_command_clean_skips_flags() {
        let p = pipeline! {
            steps: [
                bazel_command {
                    verb: "clean",
                    label: "clean",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG));
        assert!(!yaml.contains(BEP_FILE_FLAG));
    }
}

mod opt_outs {
    //! Pipeline-level booleans flip injection off independently.

    use super::*;

    #[test]
    fn opt_out_of_invocation_id_only() {
        let p = pipeline! {
            use_buildkite_job_invocation_id: false,
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(
            !yaml.contains(INVOCATION_ID_FLAG),
            "invocation_id should be suppressed:\n{yaml}"
        );
        assert!(
            yaml.contains(BEP_FILE_FLAG),
            "bep file should still be set:\n{yaml}"
        );
    }

    #[test]
    fn opt_out_of_bep_file_only() {
        let p = pipeline! {
            set_build_event_binary_file_path: false,
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(
            yaml.contains(INVOCATION_ID_FLAG),
            "invocation_id should still be set:\n{yaml}"
        );
        assert!(
            !yaml.contains(BEP_FILE_FLAG),
            "bep file should be suppressed:\n{yaml}"
        );
    }

    #[test]
    fn opt_out_of_both() {
        let p = pipeline! {
            use_buildkite_job_invocation_id: false,
            set_build_event_binary_file_path: false,
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG));
        assert!(!yaml.contains(BEP_FILE_FLAG));
        // The user's command should still be intact.
        assert!(yaml.contains("bazel build //..."), "user cmd lost:\n{yaml}");
    }

    #[test]
    fn opt_outs_apply_to_test_verb_too() {
        let p = pipeline! {
            use_buildkite_job_invocation_id: false,
            set_build_event_binary_file_path: false,
            steps: [
                bazel_test {
                    target_patterns: "//foo/...",
                    label: "test foo",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG));
        assert!(!yaml.contains(BEP_FILE_FLAG));
    }

    #[test]
    fn explicit_true_matches_default() {
        let p_default = pipeline! {
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        let p_explicit = pipeline! {
            use_buildkite_job_invocation_id: true,
            set_build_event_binary_file_path: true,
            steps: [
                bazel_build {
                    target_patterns: "//...",
                    label: "x",
                }
            ]
        };
        assert_eq!(
            serde_yaml::to_string(&p_default).unwrap(),
            serde_yaml::to_string(&p_explicit).unwrap(),
        );
    }
}

mod dynamic_bazel {
    use super::*;

    #[test]
    fn dynamic_bazel_test_with_flags_carries_both_flags() {
        let p = pipeline! {
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    flags: "--test_output=errors",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(yaml.contains(INVOCATION_ID_FLAG));
        assert!(yaml.contains(BEP_FILE_FLAG));
        assert!(yaml.contains("--test_output=errors"));
    }

    #[test]
    fn dynamic_bazel_respects_opt_out() {
        let p = pipeline! {
            use_buildkite_job_invocation_id: false,
            set_build_event_binary_file_path: false,
            steps: [
                bazel_test {
                    target_patterns: "//...",
                    flags: "--test_output=errors",
                    label: "x",
                }
            ]
        };
        let yaml = serde_yaml::to_string(&p).unwrap();
        assert!(!yaml.contains(INVOCATION_ID_FLAG));
        assert!(!yaml.contains(BEP_FILE_FLAG));
        assert!(yaml.contains("--test_output=errors"));
    }
}
