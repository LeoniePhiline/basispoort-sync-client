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

#[derive(Debug, Default, Deserialize, Serialize)]
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

#[derive(Debug, Default, Deserialize, Serialize)]
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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BulkRequest {
    #[serde(rename = "methodes")]
    pub method_ids: Vec<String>,
    #[serde(rename = "producten")]
    pub product_ids: Vec<String>,
    #[serde(rename = "gebruikers")]
    pub user_ids: Vec<u64>,
    #[serde(rename = "gebruikerEckIds")]
    pub chain_ids: Vec<UserChainId>,
}

pub struct HostedSitesClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
    identity_code: String,
}

// TODO: Ensure method ID is valid and does not contain a slash; fail with an appropriate error otherwise.
// TODO: Ensure all validation as documented.
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

    /*
     * Method management
     */

    pub async fn get_methods(&self) -> crate::Result<MethodDetailsList> {
        let response = self.get("/methode").await?;
        response
            .json::<MethodDetailsList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn get_method<S: AsRef<str>>(&self, method_id: S) -> crate::Result<MethodDetails> {
        let response = self
            .get(&format!("/methode/{}", method_id.as_ref()))
            .await?;
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
        self.put(&format!("/methode/{}", method.id), method).await?;
        Ok(())
    }

    pub async fn delete_method<S: AsRef<str>>(&self, method_id: S) -> crate::Result<()> {
        self.delete(&format!("/methode/{}", method_id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn get_method_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
    ) -> crate::Result<UserIdList> {
        let response = self
            .get(&format!("/methode/{}/gebruiker", method_id.as_ref()))
            .await?;
        response
            .json::<UserIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_method_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.put(&format!("/methode/{}/gebruiker", method_id.as_ref()), users)
            .await?;
        Ok(())
    }

    pub async fn delete_method_user_ids<S: AsRef<str>>(&self, method_id: S) -> crate::Result<()> {
        self.delete(&format!("/methode/{}/gebruiker", method_id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn add_method_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker/addlist", method_id.as_ref()),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn remove_method_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker/removelist", method_id.as_ref()),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn get_method_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
    ) -> crate::Result<UserChainIdList> {
        let response = self
            .get(&format!("/methode/{}/gebruiker_eckid", method_id.as_ref()))
            .await?;
        response
            .json::<UserChainIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_method_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.put(
            &format!("/methode/{}/gebruiker_eckid", method_id.as_ref()),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn delete_method_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
    ) -> crate::Result<()> {
        self.delete(&format!("/methode/{}/gebruiker_eckid", method_id.as_ref()))
            .await?;
        Ok(())
    }

    pub async fn add_method_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker_eckid/addlist", method_id.as_ref()),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn remove_method_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker_eckid/removelist", method_id.as_ref()),
            users,
        )
        .await?;
        Ok(())
    }

    /*
     * Product management
     */

    pub async fn get_products<S: AsRef<str>>(
        &self,
        method_id: S,
    ) -> crate::Result<ProductDetailsList> {
        let response = self
            .get(&format!("/methode/{}/product", method_id.as_ref()))
            .await?;
        response
            .json::<ProductDetailsList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn get_product<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<ProductDetails> {
        let response = self
            .get(&format!(
                "/methode/{}/product/{}",
                method_id.as_ref(),
                product_id.as_ref()
            ))
            .await?;
        response
            .json::<ProductDetails>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn post_product<S: AsRef<str>>(
        &self,
        method_id: S,
        product: &ProductDetails,
    ) -> crate::Result<()> {
        self.post(&format!("/methode/{}/product", method_id.as_ref()), product)
            .await?;
        Ok(())
    }

    pub async fn put_product<S: AsRef<str>>(
        &self,
        method_id: S,
        product: &ProductDetails,
    ) -> crate::Result<()> {
        self.put(
            &format!("/methode/{}/product/{}", method_id.as_ref(), product.id),
            product,
        )
        .await?;
        Ok(())
    }

    pub async fn delete_product<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await?;
        Ok(())
    }

    pub async fn get_product_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<UserIdList> {
        let response = self
            .get(&format!(
                "/methode/{}/product/{}/gebruiker",
                method_id.as_ref(),
                product_id.as_ref()
            ))
            .await?;
        response
            .json::<UserIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_product_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.put(
            &format!(
                "/methode/{}/product/{}/gebruiker",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn delete_product_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}/gebruiker",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await?;
        Ok(())
    }

    pub async fn add_product_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker/addlist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn remove_product_user_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker/removelist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn get_product_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<UserChainIdList> {
        let response = self
            .get(&format!(
                "/methode/{}/product/{}/gebruiker_eckid",
                method_id.as_ref(),
                product_id.as_ref()
            ))
            .await?;
        response
            .json::<UserChainIdList>()
            .await
            .map_err(|source| Error::DecodeResponse(source).into())
    }

    pub async fn put_product_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.put(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn delete_product_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
    ) -> crate::Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}/gebruiker_eckid",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await?;
        Ok(())
    }

    pub async fn add_product_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid/addlist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    pub async fn remove_product_user_chain_ids<S: AsRef<str>>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> crate::Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid/removelist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await?;
        Ok(())
    }

    /*
     * Bulk actions
     */

    pub async fn bulk_grant_permissions(&self, bulk_request: &BulkRequest) -> crate::Result<()> {
        self.post("/permissions/grant", bulk_request).await?;
        Ok(())
    }

    pub async fn bulk_revoke_permissions(&self, bulk_request: &BulkRequest) -> crate::Result<()> {
        self.post("/permissions/revoke", bulk_request).await?;
        Ok(())
    }
}
