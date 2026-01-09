use serde::{Deserialize, Serialize};
use std::fmt;

/// Error response from Buildkite API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

/// Error type for Buildkite API operations
#[derive(Debug)]
pub struct BuildkiteError {
    pub status_code: Option<u16>,
    pub message: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub raw_body: Option<Vec<u8>>,
}

impl fmt::Display for BuildkiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.method, &self.url, self.status_code) {
            (Some(method), Some(url), Some(code)) => {
                write!(f, "{} {}: {} - {}", method, url, code, self.message)
            }
            (None, Some(url), Some(code)) => {
                write!(f, "{}: {} - {}", url, code, self.message)
            }
            _ => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for BuildkiteError {}

impl From<reqwest::Error> for BuildkiteError {
    fn from(err: reqwest::Error) -> Self {
        BuildkiteError {
            status_code: err.status().map(|s| s.as_u16()),
            message: err.to_string(),
            url: err.url().map(|u| u.to_string()),
            method: None,
            raw_body: None,
        }
    }
}

impl From<url::ParseError> for BuildkiteError {
    fn from(err: url::ParseError) -> Self {
        BuildkiteError {
            status_code: None,
            message: format!("URL parse error: {}", err),
            url: None,
            method: None,
            raw_body: None,
        }
    }
}

impl From<serde_json::Error> for BuildkiteError {
    fn from(err: serde_json::Error) -> Self {
        BuildkiteError {
            status_code: None,
            message: format!("JSON error: {}", err),
            url: None,
            method: None,
            raw_body: None,
        }
    }
}
