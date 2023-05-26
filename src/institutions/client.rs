use std::fmt::Debug;

use serde::{de::DeserializeOwned, Serialize};
use tracing::instrument;

use crate::{rest, Result};

use super::model::*;

#[derive(Debug)]
pub struct InstitutionsServiceClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
}

impl<'a> InstitutionsServiceClient<'a> {
    #[instrument]
    pub fn new(rest_client: &'a rest::RestClient) -> Self {
        InstitutionsServiceClient {
            rest_client,
            // TODO: Exception "/v2/nawsearch" - separate client.
            // TODO: "/v2/licenties" as separate service (and crate feature)
            base_path: "/rest/v2/instellingen",
        }
    }

    fn make_path(&self, path: &str) -> String {
        format!("{}{}", self.base_path, path)
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

    #[instrument(skip(self))]
    async fn delete<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        self.rest_client.delete(&self.make_path(path)).await
    }

    /*
     * Institutions service
     */

    pub async fn get_institution_ids(&self) -> Result<Vec<u64>> {
        self.get("/").await
    }

    // TODO: Use in integration test
    // TODO: Rename to get_institution_staff_and_students? get_institution_people?
    pub async fn get_institution(&self, institution_id: u64) -> Result<Institution> {
        self.get(&format!("/{}", institution_id)).await
    }

    // TODO: Add next endpoints...
}
