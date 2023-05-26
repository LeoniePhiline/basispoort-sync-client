use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::instrument;

use crate::{rest, Result};

use super::model::*;

/// An API client for the hosted license provider service ("Hosted Lika").
#[derive(Debug)]
pub struct HostedLicenseProviderClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
    identity_code: String,
}

// TODO: Ensure method ID is valid and does not contain a slash; fail with an appropriate error otherwise.
// TODO: Ensure all validation as documented.
impl<'a> HostedLicenseProviderClient<'a> {
    #[instrument]
    pub fn new<S: Into<String> + Debug>(
        rest_client: &'a rest::RestClient,
        identity_code: S,
    ) -> Self {
        HostedLicenseProviderClient {
            rest_client,
            base_path: "/hosted-lika/management/lika",
            identity_code: identity_code.into(),
        }
    }

    fn make_path(&self, path: &str) -> String {
        format!("{}/{}{}", self.base_path, self.identity_code, path)
    }

    #[instrument(skip(self))]
    async fn get<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        self.rest_client.get(&self.make_path(path)).await
    }

    #[instrument(skip(self, payload))]
    async fn post<P: Serialize + Debug + ?Sized, T: DeserializeOwned + Debug + ?Sized>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T> {
        self.rest_client.post(&self.make_path(path), payload).await
    }

    #[instrument(skip(self, payload))]
    async fn put<P: Serialize + Debug + ?Sized, T: DeserializeOwned + Debug + ?Sized>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T> {
        self.rest_client.put(&self.make_path(path), payload).await
    }

    #[instrument(skip(self))]
    async fn delete<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        self.rest_client.delete(&self.make_path(path)).await
    }

    /*
     * Method management
     */

    #[instrument]
    pub async fn get_methods(&self) -> Result<MethodDetailsList> {
        self.get("/methode").await
    }

    #[instrument]
    pub async fn get_method<S: AsRef<str> + Debug>(&self, method_id: S) -> Result<MethodDetails> {
        self.get(&format!("/methode/{}", method_id.as_ref())).await
    }

    #[instrument]
    pub async fn create_method(&self, method: &MethodDetails) -> Result<()> {
        self.post("/methode", method).await
    }

    #[instrument]
    pub async fn update_method(&self, method: &MethodDetails) -> Result<()> {
        self.put(&format!("/methode/{}", method.id), method).await
    }

    #[instrument]
    pub async fn delete_method<S: AsRef<str> + Debug>(&self, method_id: S) -> Result<()> {
        self.delete(&format!("/methode/{}", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn get_method_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
    ) -> Result<UserIdList> {
        self.get(&format!("/methode/{}/gebruiker", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn set_method_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.put(&format!("/methode/{}/gebruiker", method_id.as_ref()), users)
            .await
    }

    #[instrument]
    pub async fn delete_method_user_ids<S: AsRef<str> + Debug>(&self, method_id: S) -> Result<()> {
        self.delete(&format!("/methode/{}/gebruiker", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn add_method_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker/addlist", method_id.as_ref()),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn remove_method_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker/removelist", method_id.as_ref()),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn get_method_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
    ) -> Result<UserChainIdList> {
        self.get(&format!("/methode/{}/gebruiker_eckid", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn set_method_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.put(
            &format!("/methode/{}/gebruiker_eckid", method_id.as_ref()),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn delete_method_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
    ) -> Result<()> {
        self.delete(&format!("/methode/{}/gebruiker_eckid", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn add_method_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker_eckid/addlist", method_id.as_ref()),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn remove_method_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.post(
            &format!("/methode/{}/gebruiker_eckid/removelist", method_id.as_ref()),
            users,
        )
        .await
    }

    /*
     * Product management
     */

    #[instrument]
    pub async fn get_products<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
    ) -> Result<ProductDetailsList> {
        self.get(&format!("/methode/{}/product", method_id.as_ref()))
            .await
    }

    #[instrument]
    pub async fn get_product<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<ProductDetails> {
        self.get(&format!(
            "/methode/{}/product/{}",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn create_product<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product: &ProductDetails,
    ) -> Result<()> {
        self.post(&format!("/methode/{}/product", method_id.as_ref()), product)
            .await
    }

    #[instrument]
    pub async fn update_product<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product: &ProductDetails,
    ) -> Result<()> {
        self.put(
            &format!("/methode/{}/product/{}", method_id.as_ref(), product.id),
            product,
        )
        .await
    }

    #[instrument]
    pub async fn delete_product<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn get_product_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<UserIdList> {
        self.get(&format!(
            "/methode/{}/product/{}/gebruiker",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn set_product_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.put(
            &format!(
                "/methode/{}/product/{}/gebruiker",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn delete_product_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}/gebruiker",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn add_product_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker/addlist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn remove_product_user_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserIdList,
    ) -> Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker/removelist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn get_product_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<UserChainIdList> {
        self.get(&format!(
            "/methode/{}/product/{}/gebruiker_eckid",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn set_product_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.put(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn delete_product_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
    ) -> Result<()> {
        self.delete(&format!(
            "/methode/{}/product/{}/gebruiker_eckid",
            method_id.as_ref(),
            product_id.as_ref()
        ))
        .await
    }

    #[instrument]
    pub async fn add_product_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid/addlist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    #[instrument]
    pub async fn remove_product_user_chain_ids<S: AsRef<str> + Debug>(
        &self,
        method_id: S,
        product_id: S,
        users: &UserChainIdList,
    ) -> Result<()> {
        self.post(
            &format!(
                "/methode/{}/product/{}/gebruiker_eckid/removelist",
                method_id.as_ref(),
                product_id.as_ref()
            ),
            users,
        )
        .await
    }

    /*
     * Bulk actions
     */

    #[instrument]
    pub async fn bulk_grant_permissions(&self, bulk_request: &BulkRequest) -> Result<()> {
        self.post("/permissions/grant", bulk_request).await
    }

    #[instrument]
    pub async fn bulk_revoke_permissions(&self, bulk_request: &BulkRequest) -> Result<()> {
        self.post("/permissions/revoke", bulk_request).await
    }
}
