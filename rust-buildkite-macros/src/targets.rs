//! Provides a fast first-pass validation that parses BUILD files
//! using the Starlark parser to check if targets exist, without running
//! any bazel subprocesses.

use starlark::analysis::find_call_name::AstModuleFindCallName;
use starlark::syntax::{AstModule, Dialect};
use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn is_relative_label(label: &str) -> bool {
    !label.starts_with("//") && !label.starts_with('@')
}

pub fn resolve_label(label: &str, current_pkg: Option<&str>) -> Result<(String, String), String> {
    let label = label.trim_start_matches('@').trim_start_matches('@');

    if label.starts_with("//") {
        let without_slashes = &label[2..];
        if let Some((pkg, target)) = without_slashes.split_once(':') {
            Ok((pkg.to_string(), target.to_string()))
        } else {
            let pkg = without_slashes;
            let target = pkg.rsplit('/').next().unwrap_or(pkg);
            Ok((pkg.to_string(), target.to_string()))
        }
    } else if label.starts_with(':') {
        let target = &label[1..];
        let pkg = current_pkg.unwrap_or("");
        Ok((pkg.to_string(), target.to_string()))
    } else if !label.contains('/') && !label.contains(':') {
        let pkg = current_pkg.unwrap_or("");
        Ok((pkg.to_string(), label.to_string()))
    } else {
        Err(format!(
            "Invalid label '{}': relative paths with / are not supported",
            label
        ))
    }
}

#[allow(dead_code)]
pub fn parse_label(label: &str) -> Result<(String, String), String> {
    resolve_label(label, None)
}

pub fn find_build_file(workspace: &Path, pkg: &str) -> Result<PathBuf, String> {
    let pkg_dir = workspace.join(pkg);
    
    let build_bazel = pkg_dir.join("BUILD.bazel");
    if build_bazel.exists() {
        return Ok(build_bazel);
    }
    
    let build = pkg_dir.join("BUILD");
    if build.exists() {
        return Ok(build);
    }
    
    Err(format!(
        "No BUILD file found for package '{}' in {}",
        pkg,
        pkg_dir.display()
    ))
}

pub fn target_exists_in_build_file(build_path: &Path, target_name: &str) -> Result<bool, String> {
    let content = std::fs::read_to_string(build_path)
        .map_err(|e| format!("Failed to read {}: {}", build_path.display(), e))?;
    
    let ast = AstModule::parse("BUILD", content, &Dialect::Standard)
        .map_err(|e| format!("Failed to parse {}: {}", build_path.display(), e))?;
    
    Ok(ast.find_function_call_with_name(target_name).is_some())
}

pub fn validate_target_exists(
    workspace: &Path,
    label: &str,
    current_pkg: Option<&str>,
) -> Result<(), String> {
    let (pkg, target) = resolve_label(label, current_pkg)?;
    let build_path = find_build_file(workspace, &pkg)?;

    if target_exists_in_build_file(&build_path, &target)? {
        Ok(())
    } else {
        let label_display = if pkg.is_empty() {
            format!("//:{}", target)
        } else {
            format!("//{}:{}", pkg, target)
        };
        Err(format!(
            "Target '{}' not found. Check {} for available targets.",
            label_display,
            build_path.display()
        ))
    }
}

pub fn is_wildcard_pattern(target: &str) -> bool {
    target.contains("...")
        || target.ends_with(":all")
        || target.ends_with(":*")
}

pub fn is_external_repo(target: &str) -> bool {
    target.starts_with('@') && !target.starts_with("@@")
}

pub fn should_skip_fast_validation(target: &str) -> bool {
    is_wildcard_pattern(target) || is_external_repo(target)
}

pub fn get_current_package(workspace: &Path, script_dir: &Path) -> Option<String> {
    script_dir
        .strip_prefix(workspace)
        .ok()
        .map(|rel| rel.to_string_lossy().to_string())
        .map(|s| s.replace('\\', "/"))
}

pub fn extract_targets_from_args(args: &[&str]) -> Vec<String> {
    args.iter()
        .filter(|arg| !arg.starts_with('-'))
        .filter(|arg| {
            arg.starts_with("//")
                || arg.starts_with('@')
                || arg.starts_with(':')
                || (!arg.contains('=') && !arg.contains('/'))
        })
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_label_absolute() {
        let (pkg, target) = resolve_label("//cpp:hello-world", None).unwrap();
        assert_eq!(pkg, "cpp");
        assert_eq!(target, "hello-world");
    }

    #[test]
    fn test_resolve_label_absolute_no_target() {
        let (pkg, target) = resolve_label("//cpp/subdir", None).unwrap();
        assert_eq!(pkg, "cpp/subdir");
        assert_eq!(target, "subdir");
    }

    #[test]
    fn test_resolve_label_root_package() {
        let (pkg, target) = resolve_label("//:main", None).unwrap();
        assert_eq!(pkg, "");
        assert_eq!(target, "main");
    }

    #[test]
    fn test_resolve_label_relative_colon() {
        let (pkg, target) = resolve_label(":baz", Some("foo/bar")).unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "baz");
    }

    #[test]
    fn test_resolve_label_relative_bare() {
        let (pkg, target) = resolve_label("baz", Some("foo/bar")).unwrap();
        assert_eq!(pkg, "foo/bar");
        assert_eq!(target, "baz");
    }

    #[test]
    fn test_resolve_label_relative_no_current_pkg() {
        let (pkg, target) = resolve_label(":main", None).unwrap();
        assert_eq!(pkg, "");
        assert_eq!(target, "main");
    }

    #[test]
    fn test_is_relative_label() {
        assert!(is_relative_label(":baz"));
        assert!(is_relative_label("baz"));
        assert!(!is_relative_label("//foo:bar"));
        assert!(!is_relative_label("@repo//foo:bar"));
    }

    #[test]
    fn test_is_wildcard_pattern() {
        assert!(is_wildcard_pattern("//..."));
        assert!(is_wildcard_pattern("//foo/..."));
        assert!(is_wildcard_pattern("//foo:all"));
        assert!(is_wildcard_pattern("//foo:*"));
        assert!(is_wildcard_pattern(":all"));
        assert!(!is_wildcard_pattern("//foo:bar"));
        assert!(!is_wildcard_pattern(":bar"));
    }

    #[test]
    fn test_is_external_repo() {
        assert!(is_external_repo("@rules_cc//cc:defs"));
        assert!(!is_external_repo("//foo:bar"));
        assert!(!is_external_repo("@@//foo:bar"));
    }

    #[test]
    fn test_extract_targets_from_args() {
        let args = vec!["//foo:bar", "--config=opt", "//baz:qux", "-k", ":local"];
        let targets = extract_targets_from_args(&args);
        assert_eq!(targets, vec!["//foo:bar", "//baz:qux", ":local"]);
    }

    #[test]
    fn test_get_current_package() {
        let workspace = Path::new("/home/user/myproject");
        let script_dir = Path::new("/home/user/myproject/foo/bar");
        assert_eq!(
            get_current_package(workspace, script_dir),
            Some("foo/bar".to_string())
        );
    }

    #[test]
    fn test_get_current_package_root() {
        let workspace = Path::new("/home/user/myproject");
        let script_dir = Path::new("/home/user/myproject");
        assert_eq!(
            get_current_package(workspace, script_dir),
            Some("".to_string())
        );
    }
}
