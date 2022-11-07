use std::fs::File;
use std::io::Read;
use std::time::Duration;

use reqwest::{Identity, Response, Url};
use serde::Serialize;
use url::Host;

use crate::error::{Error, ErrorResponse};

pub struct RestClientBuilder<'i> {
    identity_cert_file: &'i str,
    environment: Environment,
    connect_timeout: Duration,
    timeout: Duration,
    min_tls_version: reqwest::tls::Version,
}

impl<'i> RestClientBuilder<'i> {
    pub fn new(identity_cert_file: &'i str, environment: Environment) -> Self {
        Self {
            identity_cert_file,
            environment,
            connect_timeout: Duration::from_secs(10),
            timeout: Duration::from_secs(30),
            // Basispoort does not support TLS 1.3 yet, so we cannot enforce it by default :(
            min_tls_version: reqwest::tls::Version::TLS_1_2,
        }
    }

    pub fn connect_timeout(&mut self, duration: Duration) -> &mut Self {
        self.connect_timeout = duration;
        self
    }

    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = duration;
        self
    }

    pub fn min_tls_version(&mut self, version: reqwest::tls::Version) -> &mut Self {
        self.min_tls_version = version;
        self
    }

    pub fn build(&self) -> crate::Result<RestClient> {
        let mut cert = Vec::new();
        File::open(self.identity_cert_file)
            .map_err(|source| Error::OpenIdentityCertFile {
                path: self.identity_cert_file.into(),
                source,
            })?
            .read_to_end(&mut cert)
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

        let host = match self.environment {
            Environment::Test => Host::Domain("test-rest.basispoort.nl"),
            Environment::Acceptance => Host::Domain("acceptatie-rest.basispoort.nl"),
            Environment::Staging => Host::Domain("staging-rest.basispoort.nl"),
            Environment::Production => Host::Domain("rest.basispoort.nl"),
        };

        Ok(RestClient { client, host })
    }
}

pub enum Environment {
    Test,
    Acceptance,
    Staging,
    Production,
}

pub struct RestClient {
    client: reqwest::Client,
    pub host: Host<&'static str>,
}

impl RestClient {
    // TODO: Unit test
    fn make_url(&self, path: &str) -> crate::Result<Url> {
        let url = format!("https://{}{}", &self.host, &path);
        Url::parse(&url).map_err(|source| Error::ParseUrl { url, source }.into())
    }

    async fn error_status(&self, url: Url, response: Response) -> crate::Result<Response> {
        let status = response.status();
        match response.error_for_status_ref() {
            Err(source) => {
                let response_bytes = response.bytes().await.map_err(Error::DecodeResponse)?;

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

    pub async fn get(&self, path: &str) -> crate::Result<Response> {
        let url = self.make_url(path)?;
        let response = self
            .client
            .get(url.clone())
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        self.error_status(url, response).await
    }

    pub async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        payload: &T,
    ) -> crate::Result<Response> {
        let url = self.make_url(path)?;
        let response = self
            .client
            .post(url.clone())
            .json(payload)
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        self.error_status(url, response).await
    }

    pub async fn put<T: Serialize + ?Sized>(
        &self,
        path: &str,
        payload: &T,
    ) -> crate::Result<Response> {
        let url = self.make_url(path)?;
        let response = self
            .client
            .put(url.clone())
            .json(payload)
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        self.error_status(url, response).await
    }

    pub async fn delete(&self, path: &str) -> crate::Result<Response> {
        let url = self.make_url(path)?;
        let response = self
            .client
            .delete(url.clone())
            .send()
            .await
            .map_err(Error::HttpRequest)?;

        self.error_status(url, response).await
    }
}
