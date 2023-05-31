use std::fmt::Debug;
use std::str::FromStr;
use std::time::Duration;

use bytes::Bytes;
use reqwest::{Identity, Response, Url};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};
use tracing::{debug, info, instrument, trace};

use crate::{
    error::{Error, ErrorResponse},
    Result,
};

/// Build [`RestClient`] ergonomically.
#[derive(Debug)]
pub struct RestClientBuilder<'i> {
    identity_cert_file: &'i str,
    environment: Environment,
    connect_timeout: Duration,
    timeout: Duration,
    min_tls_version: reqwest::tls::Version,
}

impl<'i> RestClientBuilder<'i> {
    #[instrument]
    pub fn new(identity_cert_file: &'i str, environment: Environment) -> Self {
        info!(
            "Configured environment: {environment:?}, connecting to '{}'.",
            environment.base_url()
        );

        Self {
            identity_cert_file,
            environment,
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
            // Basispoort does not support TLS 1.3 yet, so we cannot enforce it by default :(
            min_tls_version: reqwest::tls::Version::TLS_1_2,
        }
    }

    /// Sets the connect timeout on the HTTP request client.
    pub fn connect_timeout(&mut self, duration: Duration) -> &mut Self {
        self.connect_timeout = duration;
        self
    }

    /// Sets the request-response timeout on the HTTP request client.
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = duration;
        self
    }

    /// Sets the minimum TLS version. At the time of writing, Basispoort does not yet support TLS 1.3.
    pub fn min_tls_version(&mut self, version: reqwest::tls::Version) -> &mut Self {
        self.min_tls_version = version;
        self
    }

    /// Build the configured [`RestClient`].
    ///
    /// Note that this method is `async` and returns a `Result`, as it reads the client certificate from disk.
    #[instrument]
    pub async fn build(self) -> Result<RestClient> {
        let mut cert = Vec::new();
        File::open(self.identity_cert_file)
            .await
            .map_err(|source| Error::OpenIdentityCertFile {
                path: self.identity_cert_file.into(),
                source,
            })?
            .read_to_end(&mut cert)
            .await
            .map_err(|source| Error::ReadIdentityCertFile {
                path: self.identity_cert_file.into(),
                source,
            })?;
        let identity =
            Identity::from_pem(&cert).map_err(|source| Error::ParseIdentityCertFile {
                path: self.identity_cert_file.into(),
                source,
            })?;

        let client = reqwest::ClientBuilder::new()
            .identity(identity)
            .connect_timeout(self.connect_timeout)
            .timeout(self.timeout)
            .min_tls_version(self.min_tls_version)
            .build()
            .map_err(Error::BuildRequestClient)?;

        Ok(RestClient {
            client,
            base_url: self.environment.base_url(),
        })
    }
}

/// A Basispoort environment.
///
/// Environments can be parsed from string, e.g. from `.env` variables.
///
/// Each environment has its own [`Environment::base_url`],
/// which is used for all [`RestClient`]s [configured][`RestClientBuilder::new`] with this `Environment`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Environment {
    Test,
    Acceptance,
    Staging,
    Production,
}

/// [`Environment`] parse error.
#[derive(Error, Debug)]
pub enum ParseEnvironmentError {
    #[error("'{0}' is not a valid environment string")]
    InvalidEnvironmentString(String),
}

impl FromStr for Environment {
    type Err = ParseEnvironmentError;

    #[instrument]
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(match s {
            "test" => Self::Test,
            "acceptance" => Self::Acceptance,
            "staging" => Self::Staging,
            "production" => Self::Production,
            s => return Err(ParseEnvironmentError::InvalidEnvironmentString(s.into())),
        })
    }
}

impl Environment {
    pub fn base_url(&self) -> Url {
        match self {
            Environment::Test => "https://test-rest.basispoort.nl/".parse().unwrap(),
            Environment::Acceptance => "https://acceptatie-rest.basispoort.nl/".parse().unwrap(),
            Environment::Staging => "https://staging-rest.basispoort.nl/".parse().unwrap(),
            Environment::Production => "https://rest.basispoort.nl/".parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RestClient {
    client: reqwest::Client,
    pub base_url: Url,
}

impl RestClient {
    // TODO: Unit test
    #[instrument]
    fn make_url(&self, path: &str) -> Result<Url> {
        self.base_url.join(path).map_err(|source| {
            Error::ParseUrl {
                url: path.to_owned(),
                source,
            }
            .into()
        })
    }

    #[instrument]
    async fn error_status(&self, url: Url, response: Response) -> Result<Response> {
        let status = response.status();

        debug!(status = status.to_string(), headers = ?response.headers());

        match response.error_for_status_ref() {
            Err(source) => {
                let response_bytes = response.bytes().await.map_err(Error::ReceiveResponseBody)?;

                let error_response = match serde_json::from_slice(&response_bytes) {
                    Ok(error_response) => ErrorResponse::JSON(error_response),
                    Err(_) => ErrorResponse::Plain(String::from_utf8_lossy(&response_bytes).into()),
                };
                Err(Error::HttpResponse {
                    url,
                    status,
                    error_response,
                    source,
                }
                .into())
            }
            Ok(_) => Ok(response),
        }
    }

    #[instrument(skip(self, response))]
    async fn deserialize<T: DeserializeOwned + Debug>(&self, response: Response) -> Result<T> {
        let payload_raw = response.bytes().await.map_err(Error::ReceiveResponseBody)?;
        trace!(?payload_raw);

        // Replace empty responses by valid JSON, deserializable into `T = ()`.
        let payload_raw = match payload_raw.len() {
            0 => Bytes::from_static(b"null"),
            _ => payload_raw,
        };

        let payload_deserialized =
            serde_json::from_slice(&payload_raw).map_err(Error::DeserializeResponseBody)?;
        debug!(?payload_deserialized);

        Ok(payload_deserialized)
    }

    #[instrument]
    pub async fn get<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        let url = self.make_url(path)?;
        trace!("GET {}", url.as_str());

        let response = self
            .client
            .get(url.clone())
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        let response = self.error_status(url, response).await?;
        self.deserialize(response).await
    }

    #[instrument(skip(payload))]
    pub async fn post<P: Serialize + Debug + ?Sized, T: DeserializeOwned + Debug + ?Sized>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T> {
        let url = self.make_url(path)?;
        trace!(?payload, "POST {}", url.as_str());

        let response = self
            .client
            .post(url.clone())
            .json(payload)
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        let response = self.error_status(url, response).await?;
        self.deserialize(response).await
    }

    #[instrument(skip(payload))]
    pub async fn put<P: Serialize + Debug + ?Sized, T: DeserializeOwned + Debug + ?Sized>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T> {
        let url = self.make_url(path)?;
        trace!(?payload, "PUT {}", url.as_str());

        let response = self
            .client
            .put(url.clone())
            .json(payload)
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        let response = self.error_status(url, response).await?;
        self.deserialize(response).await
    }

    #[instrument]
    pub async fn delete<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        let url = self.make_url(path)?;
        trace!("DELETE {}", url.as_str());

        let response = self
            .client
            .delete(url.clone())
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        let response = self.error_status(url, response).await?;
        self.deserialize(response).await
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // TODO: Test make_url
}
