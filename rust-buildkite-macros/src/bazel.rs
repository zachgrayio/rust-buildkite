//! Bazel target validation using fast BUILD file parsing and bazel query.

use crate::debug::debug_log;
use crate::targets;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

pub fn find_bazel_workspace_from_env() -> Result<std::path::PathBuf, String> {
    let (workspace, _) = find_bazel_workspace_and_script_dir()?;
    Ok(workspace)
}

pub fn find_bazel_workspace_and_script_dir(
) -> Result<(std::path::PathBuf, std::path::PathBuf), String> {
    use std::path::PathBuf;

    let start = std::env::var("RUST_SCRIPT_BASE_PATH")
        .or_else(|_| std::env::var("CARGO_MANIFEST_DIR"))
        .map(PathBuf::from)
        .map_err(|_| "Neither RUST_SCRIPT_BASE_PATH nor CARGO_MANIFEST_DIR is set".to_string())?;

    let script_dir = start.clone();

    let mut dir = start.as_path();
    loop {
        if dir.join("MODULE.bazel").exists() || dir.join("WORKSPACE").exists() {
            return Ok((dir.to_path_buf(), script_dir));
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            None => break,
        }
    }

    Err(format!(
        "Could not find bazel workspace (MODULE.bazel or WORKSPACE) above {}",
        start.display()
    ))
}

pub fn fast_validate_targets(
    args: &[&str],
    workspace: &Path,
    current_pkg: Option<&str>,
) -> Result<(), String> {
    let target_args = targets::extract_targets_from_args(args);

    for target in target_args {
        if targets::should_skip_fast_validation(&target) {
            debug_log!(
                "bazel",
                "Skipping wildcard/external target: {}",
                target
            );
            continue;
        }

        debug_log!("bazel", "Validating target: {}", target);
        let start = Instant::now();

        if let Err(e) = targets::validate_target_exists(workspace, &target, current_pkg) {
            debug_log!("bazel", "Target not found in {:.2?}: {}", start.elapsed(), e);
            return Err(e);
        }

        debug_log!("bazel", "Target found in {:.2?}", start.elapsed());
    }

    Ok(())
}

#[derive(Debug, Default)]
pub struct QueryResult {
    pub kind: Option<String>,
}

pub fn validate_with_query(
    verb: &str,
    args: &[&str],
    workspace: &Path,
    current_pkg: Option<&str>,
) -> Result<HashMap<String, QueryResult>, String> {
    use std::process::Command;

    let target_args = targets::extract_targets_from_args(args);
    if target_args.is_empty() {
        return Ok(HashMap::new());
    }

    let resolved_targets: Vec<String> = target_args
        .iter()
        .map(|t| {
            if targets::is_wildcard_pattern(t) || targets::is_external_repo(t) {
                t.clone()
            } else if t.starts_with("//") {
                t.clone()
            } else {
                let pkg = current_pkg.unwrap_or("");
                if t.starts_with(':') {
                    format!("//{}:{}", pkg, &t[1..])
                } else {
                    format!("//{}:{}", pkg, t)
                }
            }
        })
        .collect();

    let query_expr = resolved_targets.join(" + ");

    let mut cmd = Command::new("bazel");
    cmd.current_dir(workspace);
    cmd.args(["query", &query_expr, "--output=label_kind"]);

    debug_log!("bazel", "Running: {:?}", cmd);
    let start = Instant::now();

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run bazel query: {}", e))?;

    debug_log!("bazel", "Query completed in {:.2?}", start.elapsed());

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no such target") || stderr.contains("no such package") {
            for line in stderr.lines() {
                if line.contains("no such target") || line.contains("no such package") {
                    return Err(line.trim().to_string());
                }
            }
        }
        return Err(format!("Bazel query failed: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut results = HashMap::new();

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.splitn(3, ' ').collect();
        if parts.len() >= 3 && parts[1] == "rule" {
            let kind = parts[0].to_string();
            let label = parts[2].to_string();
            results.insert(label, QueryResult { kind: Some(kind) });
        }
    }

    match verb {
        "run" => {
            if results.len() != 1 {
                return Err(format!(
                    "bazel run requires exactly one target, found {}",
                    results.len()
                ));
            }
            for (target, result) in &results {
                if let Some(kind) = &result.kind {
                    validate_verb_target_compatibility(verb, target, kind)?;
                }
            }
        }
        "test" => {
            let has_test_target = results.values().any(|r| {
                r.kind
                    .as_ref()
                    .map(|k| k.contains("_test"))
                    .unwrap_or(false)
            });
            if !has_test_target && !results.is_empty() {
                let targets: Vec<_> = results.keys().collect();
                return Err(format!(
                    "No test targets found. Targets {:?} are not test targets.",
                    targets
                ));
            }
        }
        _ => {}
    }

    Ok(results)
}

pub fn validate_verb_target_compatibility(
    verb: &str,
    target: &str,
    kind: &str,
) -> Result<(), String> {
    match verb {
        "run" => {
            if !kind.contains("_binary") && !kind.contains("_test") {
                return Err(format!(
                    "Cannot run '{}': '{}' is not an executable target (must be *_binary or *_test).",
                    target, kind
                ));
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn canonicalize_flags(
    verb: &str,
    args: &[&str],
    workspace: &Path,
) -> Result<Vec<String>, String> {
    use std::process::Command;

    let flags: Vec<&str> = args
        .iter()
        .filter(|a| {
            a.starts_with('-')
                && !a.starts_with("-//")
                && !a.starts_with("-@")
                && !a.starts_with("-:")
        })
        .copied()
        .collect();

    if flags.is_empty() {
        return Ok(Vec::new());
    }

    let mut cmd = Command::new("bazel");
    cmd.current_dir(workspace);
    cmd.arg("canonicalize-flags");
    cmd.arg(format!("--for_command={}", verb));
    cmd.args(&flags);

    debug_log!("bazel", "Running: {:?}", cmd);
    let start = Instant::now();

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to run bazel canonicalize-flags: {}", e))?;

    debug_log!(
        "bazel",
        "canonicalize-flags completed in {:.2?}",
        start.elapsed()
    );

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let error_lines: Vec<&str> = stderr
            .lines()
            .filter(|l| l.contains("Unrecognized") || l.contains("Error") || l.contains("error"))
            .collect();
        if !error_lines.is_empty() {
            return Err(format!("Invalid flags: {}", error_lines.join("; ")));
        }
        return Err(format!("Flag validation failed: {}", stderr.trim()));
    }

    let canonical: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verb_target_compatibility() {
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_binary rule").is_ok());
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_test rule").is_ok());
        assert!(validate_verb_target_compatibility("run", "//foo:bar", "cc_library rule").is_err());
        assert!(validate_verb_target_compatibility("test", "//foo:bar", "cc_test rule").is_ok());
        assert!(validate_verb_target_compatibility("test", "//foo:bar", "cc_binary rule").is_ok());
        assert!(validate_verb_target_compatibility("build", "//foo:bar", "cc_library rule").is_ok());
    }
}
