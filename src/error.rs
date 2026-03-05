use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug)]
pub enum HetznerError {
    Http(reqwest::Error),
    Serialization(serde_json::Error),
    Api(ApiError),
    UnexpectedResponse(&'static str),
}

impl fmt::Display for HetznerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(err) => write!(f, "http request failed: {err}"),
            Self::Serialization(err) => write!(f, "failed to decode response body: {err}"),
            Self::Api(err) => write!(
                f,
                "api error (status {}, code {}): {}",
                err.status.as_u16(),
                err.code,
                err.message
            ),
            Self::UnexpectedResponse(message) => write!(f, "unexpected response: {message}"),
        }
    }
}

impl std::error::Error for HetznerError {}

impl From<reqwest::Error> for HetznerError {
    fn from(value: reqwest::Error) -> Self {
        Self::Http(value)
    }
}

impl From<serde_json::Error> for HetznerError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialization(value)
    }
}

#[derive(Debug, Clone)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub details: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiErrorEnvelope {
    pub error: ApiErrorBody,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiErrorBody {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<Value>,
}

pub type Result<T> = std::result::Result<T, HetznerError>;
