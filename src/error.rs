use std::{io, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    /// Failed to open identity certificate file at the specified path.
    #[error("failed to open identity certificate file at '{path}'")]
    OpenIdentityCertFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Failed to read identity certificate file at the specified path.
    #[error("failed to read identity certificate file at '{path}'")]
    ReadIdentityCertFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Failed parsing identity certificate file at the specified path.
    #[error("failed parsing identity certificate file at '{path}'")]
    ParseIdentityCertFile {
        path: PathBuf,
        #[source]
        source: reqwest::Error,
    },

    /// Failed building request client.
    #[error("failed building request client")]
    BuildRequestClient(#[source] reqwest::Error),

    /// Failed to parse URL.
    #[error("failed to parse URL")]
    ParseUrl {
        url: String,
        #[source]
        source: url::ParseError,
    },

    /// Failed to open icon file at the specified path.
    #[error("failed to open icon file at '{path}'")]
    OpenIconFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Failed to read icon file at the specified path.
    #[error("failed to read icon file at '{path}'")]
    ReadIconFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },

    /// Failed to encode payload.
    #[error("failed to encode payload")]
    // TODO: Useful information to pass here?
    EncodePayload(#[source] serde_json::Error),

    /// HTTP request error.
    #[error("HTTP request error")]
    HttpRequest(#[source] reqwest::Error),

    /// HTTP response error.
    #[error("HTTP {status} error response for '{url}'")]
    HttpResponse {
        url: Url,
        status: reqwest::StatusCode,
        error_response: ErrorResponse,
        #[source]
        source: reqwest::Error,
    },

    /// Failed receiving the server's response body.
    #[error("failed receiving the server's response body")]
    ReceiveResponseBody(#[source] reqwest::Error),

    /// Failed decoding the server's response body.
    #[error("failed decoding the server's response body")]
    DeserializeResponseBody(#[source] serde_json::Error),

    /// Failed to url-encode the search predicate.
    #[error("failed to url-encode the search predicate")]
    SerializeSearchPredicate(#[source] serde_urlencoded::ser::Error),
}

#[derive(Debug, Deserialize)]
pub enum ErrorResponse {
    JSON(serde_json::Value),
    Plain(String),
}
