use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::error::Error;
use crate::rest;

#[derive(Debug, Deserialize, Serialize)]
pub struct MethodDetailsList {
    #[serde(rename = "methodes")]
    pub methods: Vec<MethodDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MethodDetails {
    pub id: String,
    #[serde(rename = "naam")]
    pub name: String,
    pub icon: Option<String>,
    pub url: Option<String>,
    // TODO: ensure unique
    pub tags: Vec<SiteTag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductDetailsList {
    #[serde(rename = "producten")]
    pub products: Vec<ProductDetails>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductDetails {
    pub id: String,
    #[serde(rename = "naam")]
    pub name: String,
    pub icon: Option<String>,
    pub url: String,
    // TODO: ensure unique
    pub tags: Vec<SiteTag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SiteTag {
    #[serde(rename = "leerkrachtApplicatie")]
    TeacherApplication,
}

pub struct HostedSitesClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
    identity_code: String,
}

impl<'a> HostedSitesClient<'a> {
    pub fn new<S: Into<String>>(rest_client: &'a rest::RestClient, identity_code: S) -> Self {
        HostedSitesClient {
            rest_client,
            base_path: "/hosted-lika/management/lika",
            identity_code: identity_code.into(),
        }
    }

    fn make_path(&self, path: &str) -> String {
        format!("{}/{}{}", self.base_path, self.identity_code, path)
    }

    async fn get(&self, path: &str) -> crate::Result<Response> {
        self.rest_client.get(&self.make_path(path)).await
    }

    async fn post<T: Serialize + ?Sized>(
        &self,
        path: &str,
        payload: &T,
    ) -> crate::Result<Response> {
        self.rest_client.post(&self.make_path(path), payload).await
    }

    async fn put<T: Serialize + ?Sized>(&self, path: &str, payload: &T) -> crate::Result<Response> {
        self.rest_client.put(&self.make_path(path), payload).await
    }

    async fn delete(&self, path: &str) -> crate::Result<Response> {
        self.rest_client.delete(&self.make_path(path)).await
    }

    pub async fn get_all_methods(&self) -> crate::Result<MethodDetailsList> {
        let response = self.get("/methode").await?;
        response
            .json::<MethodDetailsList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn get_method(&self, method_id: &str) -> crate::Result<MethodDetails> {
        // TODO: Enfore method_id not containing slash?
        let response = self.get(&format!("/methode/{method_id}")).await?;
        response
            .json::<MethodDetails>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn post_method(&self, method: &MethodDetails) -> crate::Result<()> {
        self.post("/methode", method).await?;
        Ok(())
    }

    pub async fn put_method(&self, method: &MethodDetails) -> crate::Result<()> {
        // TODO: Enfore method_id not containing slash?
        self.put(&format!("/methode/{}", method.id), method).await?;
        Ok(())
    }

    pub async fn delete_method<T: AsRef<str>>(&self, method_id: T) -> crate::Result<()> {
        self.delete(&format!("/methode/{}", method_id.as_ref()))
            .await?;
        Ok(())
    }
}
