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

## Complete opinionated example
```rust
#!/usr/bin/env rust-script

//! This is an example dynamic pipeline script meant to be directly invoked with something like:
//! 
//!   ./dynamic_pipeline.rs | buildkite-agent pipeline upload
//! 
//! When executed on a CI worker, the pipeline will be validated at compile time, 
//! including all commands and environment variables.
//! 
//! ```cargo
//! [dependencies]
//! rust-buildkite = { path = "../../.." }
//! serde_yaml = "0.9"
//! ```

use rust_buildkite::pipeline;

fn main() {
    let pipeline = pipeline! {
        // Buildkite pipeline-level environment variables
        // these are available to all steps, and references to them are validated at compile time.
        env: {
            CI: "true",
            RUST_BACKTRACE: "1",
            CARGO_TERM_COLOR: "always"
        },

        // additional runtime environment variables that commands are allowed to reference, aside from those in the env block.
        //  - these are validated at compile time - undefined vars cause errors. 
        //  - if not supplied, defaults to host env; including these here allows this to build on non-CI machines
        //  - SHELL_ENV, BUILDKITE_ENV, CI_ENV are keywords that expand to known values for convenience.
        // nb: this could be omitted entirely to validate against the host env during compilation!
        runtime_env: [SHELL_ENV, BUILDKITE_ENV],

        // commands use host PATH by default for compile time validation of commands;
        // you can override this with an explicit allowed_commands list:
        // allowed_commands: ["cargo", "npm", "docker"],

        // declare paths that don't exist at compile time to allow them to be used at runtime
        // again these will be validated at runtime on the CI worker, but we need it here to allow the pipeline to build on non-CI machines
        allow_missing_paths: ["./scripts/deploy.sh"],

        steps: [
            command {
                // shell syntax and executable references are validated at compile time.
                // nb: the cmd! macro is required here, and raw string commands are not allowed.
                // this is what enables compile time validation of commands.
                command: cmd!("cargo fmt --check"),
                label: "🎨 Format",
                key: "fmt"
            },

            command {
                command: cmd!("cargo clippy -- -D warnings"),
                label: "📎 Clippy",
                key: "clippy",
                soft_fail: true
            },

            command {
                command: cmd!("cargo test --all-features"),
                label: "🧪 Tests",
                key: "test",
                env: {
                    RUST_LOG: "debug"
                },
                parallelism: 4,
                retry: {
                    automatic: { limit: 2 }
                }
            },

            wait,

            command {
                // these env vars are all checked at compile time, along with shell syntax.
                command: cmd!(r#"echo "Building $BUILDKITE_BRANCH @ $BUILDKITE_COMMIT""#),
                label: "📋 Build Info"
            },

            command {
                command: cmd!("cargo build --release"),
                label: "🔨 Build",
                key: "build",
                artifact_paths: ["target/release/myapp"]
            },

            wait,

            block {
                block: "Deploy to Production?",
                key: "approval",
                prompt: "Are you sure?",
                branches: ["main"]
            },

            command {
                // builtins (cd, source, export) are always allowed.
                // bash is always used for shell syntax validation, and bash
                // builtins will be allowed by default in addition to POSIX standard;
                // this may change in the future
                command: cmd!("cd dist && ./scripts/deploy.sh production"),
                label: "🚀 Deploy",
                key: "deploy",
                depends_on: ["build", "approval"],
                concurrency: 1,
                concurrency_group: "deploy/production"
            }
        ]
    };

    println!("{}", serde_yaml::to_string(&pipeline).expect("yaml"));
}
```

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
# In your BUILD file
rust_library(
    name = "pipeline",
    rustc_env = {
        "BUILDKITE_SKIP_COMPTIME_VALIDATION": "1",
    },
    deps = ["@crates//:rust-buildkite"],
)
```

This skips path existence, command allowlist, Bazel target, and env var validation at compile time. Runtime validation is still performed when the binary runs.

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
