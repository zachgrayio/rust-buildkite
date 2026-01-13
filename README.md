# rust-buildkite

A [Rust](https://www.rust-lang.org/) library and client for the [Buildkite API](https://buildkite.com/docs/api).

## Disclaimer

- The initial commit of this project is a direct port of [go-buildkite](https://github.com/buildkite/go-buildkite), with an intentionally similar API and test scenarios.
- This is not yet fully validated, and as such, it's not ready for production usage.

# Usage

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

See the [api_examples](api_examples/) directory for additional examples on usage of the API client, and [pipeline_examples](pipeline_examples/) for example usage of the Pipeline definition code generated from the Buildkite Json Schema.

# Roadmap

- [ ] create a CI pipeline and releases
- [ ] validate all functionality
- [ ] macros for better usability

# License

Released under the [MIT License](https://opensource.org/license/MIT).
