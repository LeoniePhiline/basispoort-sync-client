use std::{io, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;
use url::Url;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open identity certificate file at '{path}'")]
    OpenIdentityCertFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read identity certificate file at '{path}'")]
    ReadIdentityCertFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed parsing identity certificate file at '{path}'")]
    ParseIdentityCertFile {
        path: PathBuf,
        #[source]
        source: reqwest::Error,
    },
    #[error("failed building request client")]
    BuildRequestClient(#[source] reqwest::Error),
    #[error("failed to parse URL")]
    ParseUrl {
        url: String,
        #[source]
        source: url::ParseError,
    },
    #[error("failed to open icon file at '{path}'")]
    OpenIconFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to read icon file at '{path}'")]
    ReadIconFile {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to encode payload")]
    // TODO: Useful information to pass here?
    EncodePayload(#[source] serde_json::Error),
    #[error("HTTP request error")]
    HttpRequest(#[source] reqwest::Error),
    #[error("HTTP response error")]
    HttpResponse {
        url: Url,
        status: reqwest::StatusCode,
        error_response: ErrorResponse,
        #[source]
        source: reqwest::Error,
    },
    #[error("failed receiving the server's response body")]
    ReceiveResponseBody(#[source] reqwest::Error),
    #[error("failed decoding the server's response body")]
    DeserializeResponseBody(#[source] serde_json::Error),
}

#[derive(Debug, Deserialize)]
pub enum ErrorResponse {
    JSON(serde_json::Value),
    Plain(String),
}
