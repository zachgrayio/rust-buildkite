# rust-buildkite

A [Rust](https://www.rust-lang.org/) library and client for the [Buildkite API](https://buildkite.com/docs/api).

## Disclaimer

- The initial commit of this project is a direct port of [go-buildkite](https://github.com/buildkite/go-buildkite), with an intentionally similar API and test scenarios.
- This is not yet fully validated, and as such, it's not ready for production usage.

# Usage

Note: the crate is not published yet, but eventually:

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
rust-buildkite = "0.1"
```

Simple example for listing all pipelines:

```rust
use rust_buildkite::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder("your-api-token").build();
    
    let pipelines = client.pipelines.list("my-org").await?;
    
    println!("{:?}", pipelines);
    
    Ok(())
}
```

See the [api_examples](examples/api/) directory for additional examples on usage of the API client, and [pipeline_examples](examples/pipeline/) for example usage of the Pipeline definition code generated from the Buildkite Json Schema.

## Pipeline Registration

For projects with multiple pipelines, use the `#[register]` attribute macro to declare pipelines with metadata, then use `registered_pipelines()` to iterate over them at runtime:

```rust
// src/pipelines/premerge.rs
use rust_buildkite::{pipeline, register};

#[register(branch = Prefix("feature/"))]
pub fn premerge() {
    let p = pipeline! { /* ... */ };
    println!("{}", serde_yaml::to_string(&p).unwrap());
}
```

```rust
// src/bin/generate.rs
use rust_buildkite::registered_pipelines;

fn main() {
    mylib::link_pipelines();  // Required - see below
    
    let branch = std::env::var("BUILDKITE_BRANCH").unwrap_or_default();
    
    for p in registered_pipelines() {
        if let Some(ref pattern) = p.branch {
            if pattern.matches(&branch) {
                (p.generate)();
                return;
            }
        }
    }
}
```

### Linker Requirements

Due to how Rust's linker works with `inventory`, pipeline modules that aren't directly referenced by the binary may be stripped. Use `link_pipelines!` to force linking:

Option 1: Manual

```rust
// src/lib.rs
rust_buildkite::link_pipelines!(
    pipelines::premerge::premerge,
    pipelines::postmerge::postmerge,
    pipelines::release::release,
);
```

Option 2: Generate with build.rs

```rust
// build.rs
use std::{env, fs, path::Path};

fn main() {
    let out = env::var("OUT_DIR").unwrap();
    let mut fns = Vec::new();
    scan("src/pipelines", &[], &mut fns);
    
    let list = fns.join(",\n    ");
    fs::write(
        Path::new(&out).join("links.rs"),
        format!("rust_buildkite::link_pipelines!(\n    {list},\n);"),
    ).unwrap();
    
    println!("cargo:rerun-if-changed=src/pipelines");
}

fn scan(dir: &str, prefix: &[&str], out: &mut Vec<String>) {
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        let name = p.file_name().unwrap().to_str().unwrap();
        if p.is_dir() {
            let mut new_prefix = prefix.to_vec();
            new_prefix.push(name);
            scan(p.to_str().unwrap(), &new_prefix, out);
        } else if name.ends_with(".rs") && name != "mod.rs" {
            if fs::read_to_string(&p).unwrap().contains("#[register") {
                let mod_name = name.trim_end_matches(".rs");
                let path = if prefix.is_empty() {
                    format!("pipelines::{mod_name}::{mod_name}")
                } else {
                    format!("pipelines::{}::{mod_name}::{mod_name}", prefix.join("::"))
                };
                out.push(path);
            }
        }
    }
}
```

and then:

```rust
include!(concat!(env!("OUT_DIR"), "/links.rs"));
```

## Validation Control

### Skipping Compile-Time Validation

For Bazel builds or sandboxed environments where file paths aren't available at compile time:

```python
rust_library(
    name = "pipeline",
    rustc_env = {
        "BUILDKITE_SKIP_COMPTIME_VALIDATION": "1",
    },
    deps = ["@crates//:rust-buildkite"],
)
```

This skips path existence, command, Bazel target, and env var validation at compile time. Runtime validation is still performed when the binary runs.

### Skipping Runtime Validation

For testing or special cases:

```bash
BUILDKITE_SKIP_RUNTIME_VALIDATION=1 ./my_pipeline
```

## Development

### Running Tests

Tests are configured via `.cargo/config.toml` to automatically skip runtime validation:

```bash
cargo test --workspace --all-features
```

UI tests verify compile-time validation errors, so `BUILDKITE_SKIP_COMPTIME_VALIDATION` should NOT be set when running tests.

# Roadmap

- [ ] create a CI pipeline and releases
- [ ] validate all functionality
- [x] macros for better usability

# License

Released under the [MIT License](https://opensource.org/license/MIT).
