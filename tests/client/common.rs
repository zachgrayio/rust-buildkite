//! Common test utilities and mock server setup

use rust_buildkite::Client;
use wiremock::{MockServer, ResponseTemplate};

/// Set up a new mock HTTP server for testing
pub async fn setup_mock_server() -> MockServer {
    MockServer::start().await
}

/// Create a JSON response with the given body
#[allow(dead_code)]
pub fn json_response(body: &str) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_string(body)
        .insert_header("content-type", "application/json")
}

/// Helper to construct a reqwest client for tests that need custom client setup
#[allow(dead_code)]
pub fn mock_client() -> reqwest::Client {
    reqwest::ClientBuilder::new().build().unwrap()
}

/// Create a typed Buildkite client for testing, pointing to the mock server
#[allow(dead_code)]
pub fn buildkite_client(mock_server: &MockServer) -> Client {
    Client::builder("test-token")
        .base_url(format!("{}/", mock_server.uri()))
        .build()
}
