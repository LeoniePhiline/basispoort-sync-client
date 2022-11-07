use std::ops::Deref;

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

#[derive(Debug, Deserialize, Serialize)]
pub struct UserIdList {
    #[serde(rename = "gebruikers")]
    pub users: Vec<u64>,
}

impl From<Vec<u64>> for UserIdList {
    fn from(users: Vec<u64>) -> Self {
        UserIdList { users }
    }
}

impl From<UserIdList> for Vec<u64> {
    fn from(list: UserIdList) -> Self {
        list.users
    }
}

impl Deref for UserIdList {
    type Target = Vec<u64>;

    fn deref(&self) -> &Self::Target {
        &self.users
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChainIdList {
    #[serde(rename = "gebruikers")]
    pub users: Vec<UserChainId>,
}

impl From<Vec<UserChainId>> for UserChainIdList {
    fn from(users: Vec<UserChainId>) -> Self {
        UserChainIdList { users }
    }
}

impl From<UserChainIdList> for Vec<UserChainId> {
    fn from(list: UserChainIdList) -> Self {
        list.users
    }
}

impl Deref for UserChainIdList {
    type Target = Vec<UserChainId>;

    fn deref(&self) -> &Self::Target {
        &self.users
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChainId {
    #[serde(rename = "instellingId")]
    pub institution_id: u64,
    #[serde(rename = "eckId")]
    pub chain_id: String,
}

pub struct HostedSitesClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
    identity_code: String,
}

// TODO: Ensure method ID is valid and does not contain a slash; fail with an appropriate error otherwise.
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

    pub async fn get_methods(&self) -> crate::Result<MethodDetailsList> {
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

    pub async fn get_method_user_ids(&self, method_id: &str) -> crate::Result<UserIdList> {
        let response = self.get(&format!("/methode/{method_id}/gebruiker")).await?;
        response
            .json::<UserIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_method_user_ids(
        &self,
        method_id: &str,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.put(&format!("/methode/{method_id}/gebruiker"), users)
            .await?;
        Ok(())
    }

    pub async fn delete_method_user_ids(&self, method_id: &str) -> crate::Result<()> {
        self.delete(&format!("/methode/{method_id}/gebruiker"))
            .await?;
        Ok(())
    }

    pub async fn add_method_user_ids(
        &self,
        method_id: &str,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(&format!("/methode/{method_id}/gebruiker/addlist"), users)
            .await?;
        Ok(())
    }

    pub async fn remove_method_user_ids(
        &self,
        method_id: &str,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(&format!("/methode/{method_id}/gebruiker/removelist"), users)
            .await?;
        Ok(())
    }

    pub async fn get_method_user_chain_ids(
        &self,
        method_id: &str,
    ) -> crate::Result<UserChainIdList> {
        let response = self
            .get(&format!("/methode/{method_id}/gebruiker_eckid"))
            .await?;
        response
            .json::<UserChainIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_method_user_chain_ids(
        &self,
        method_id: &str,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.put(&format!("/methode/{method_id}/gebruiker_eckid"), users)
            .await?;
        Ok(())
    }

    pub async fn delete_method_user_chain_ids(&self, method_id: &str) -> crate::Result<()> {
        self.delete(&format!("/methode/{method_id}/gebruiker_eckid"))
            .await?;
        Ok(())
    }

    pub async fn add_method_user_chain_ids(
        &self,
        method_id: &str,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{method_id}/gebruiker_eckid/addlist"),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn remove_method_user_chain_ids(
        &self,
        method_id: &str,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{method_id}/gebruiker_eckid/removelist"),
            users,
        )
        .await?;
        Ok(())
    }
}
