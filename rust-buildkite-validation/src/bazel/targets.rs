//! Bazel target validation via BUILD file parsing.

use super::labels;
use starlark::analysis::find_call_name::AstModuleFindCallName;
use starlark::syntax::{AstModule, Dialect};
use std::path::{Path, PathBuf};

/// Find BUILD file for a package.
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

/// Check if target exists in BUILD file.
pub fn target_exists_in_build_file(build_path: &Path, target_name: &str) -> Result<bool, String> {
    let content = std::fs::read_to_string(build_path)
        .map_err(|e| format!("Failed to read {}: {}", build_path.display(), e))?;

    let ast = AstModule::parse("BUILD", content, &Dialect::Standard)
        .map_err(|e| format!("Failed to parse {}: {}", build_path.display(), e))?;

    Ok(ast.find_function_call_with_name(target_name).is_some())
}

/// Validate target exists in workspace.
pub fn validate_target_exists(
    workspace: &Path,
    label: &str,
    current_pkg: Option<&str>,
) -> Result<(), String> {
    let (pkg, target) = labels::resolve_label(label, current_pkg)?;
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

/// Validate targets from args.
pub fn fast_validate_targets(
    args: &[&str],
    workspace: &Path,
    current_pkg: Option<&str>,
) -> Result<(), String> {
    let target_args = labels::extract_targets_from_args(args);

    for target in target_args {
        if labels::should_skip_validation(&target) {
            continue;
        }

        validate_target_exists(workspace, &target, current_pkg)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_build_file(dir: &Path, content: &str) -> PathBuf {
        let build_path = dir.join("BUILD.bazel");
        let mut f = std::fs::File::create(&build_path).unwrap();
        writeln!(f, "{}", content).unwrap();
        build_path
    }

    #[test]
    fn test_find_build_file_bazel() {
        let dir = tempdir().unwrap();
        let pkg_dir = dir.path().join("foo");
        std::fs::create_dir(&pkg_dir).unwrap();
        std::fs::write(pkg_dir.join("BUILD.bazel"), "").unwrap();

        let result = find_build_file(dir.path(), "foo");
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("BUILD.bazel"));
    }

    #[test]
    fn test_find_build_file_legacy() {
        let dir = tempdir().unwrap();
        let pkg_dir = dir.path().join("bar");
        std::fs::create_dir(&pkg_dir).unwrap();
        std::fs::write(pkg_dir.join("BUILD"), "").unwrap();

        let result = find_build_file(dir.path(), "bar");
        assert!(result.is_ok());
        assert!(result.unwrap().ends_with("BUILD"));
    }

    #[test]
    fn test_find_build_file_not_found() {
        let dir = tempdir().unwrap();
        let pkg_dir = dir.path().join("missing");
        std::fs::create_dir(&pkg_dir).unwrap();

        let result = find_build_file(dir.path(), "missing");
        assert!(result.is_err());
    }

    #[test]
    fn test_target_exists_in_build_file() {
        let dir = tempdir().unwrap();
        let build_path = create_build_file(
            dir.path(),
            r#"
cc_binary(
    name = "hello",
    srcs = ["hello.cc"],
)

cc_library(
    name = "lib",
    srcs = ["lib.cc"],
)
"#,
        );

        assert!(target_exists_in_build_file(&build_path, "hello").unwrap());
        assert!(target_exists_in_build_file(&build_path, "lib").unwrap());
        assert!(!target_exists_in_build_file(&build_path, "notfound").unwrap());
    }

    #[test]
    fn test_validate_target_exists() {
        let dir = tempdir().unwrap();
        let pkg_dir = dir.path().join("app");
        std::fs::create_dir(&pkg_dir).unwrap();
        std::fs::write(
            pkg_dir.join("BUILD.bazel"),
            r#"cc_binary(name = "main", srcs = ["main.cc"])"#,
        )
        .unwrap();

        assert!(validate_target_exists(dir.path(), "//app:main", None).is_ok());
        assert!(validate_target_exists(dir.path(), "//app:missing", None).is_err());
    }
}
