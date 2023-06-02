use std::fmt::Debug;

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};
use tracing::instrument;

use crate::{rest, BasispoortId, Result};

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
            base_path: "rest/v2/instellingen/",
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

    #[instrument]
    pub async fn get_institution_ids(&self) -> Result<Vec<BasispoortId>> {
        self.get("").await
    }

    #[instrument]
    pub async fn get_institution_overview(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionOverview> {
        self.get(&format!("{institution_id}")).await
    }

    #[instrument]
    pub async fn get_institution_details(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionDetails> {
        self.get(&format!("{institution_id}/details")).await
    }

    #[instrument]
    pub async fn get_institution_groups(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionGroups> {
        self.get(&format!("{institution_id}/groepen")).await
    }

    #[instrument]
    pub async fn get_institution_students(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionStudents> {
        self.get(&format!("{institution_id}/leerlingen")).await
    }

    #[instrument]
    pub async fn get_institution_students_by_id(
        &self,
        institution_id: BasispoortId,
        student_ids: &[BasispoortId],
    ) -> Result<InstitutionStudents> {
        self.post(&format!("{institution_id}/leerlingen"), student_ids)
            .await
    }

    #[instrument]
    pub async fn get_institution_students_by_chain_id(
        &self,
        institution_id: BasispoortId,
        student_chain_ids: &[String], // TODO: type def?
    ) -> Result<InstitutionStudents> {
        self.post(
            &format!("{institution_id}/leerlingen_eckid"),
            student_chain_ids,
        )
        .await
    }

    #[instrument]
    pub async fn get_institution_staff(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionStaff> {
        self.get(&format!("{institution_id}/staf")).await
    }

    #[instrument]
    pub async fn get_institution_shortcut_reference(
        &self,
        institution_id: BasispoortId,
    ) -> Result<String> {
        self.get(&format!("{institution_id}/ref")).await
    }

    // TODO: Test requesting sync permission manually with a school with ICT coordinator account.
    #[instrument]
    pub async fn get_institution_synchronization_permission(
        &self,
        institution_id: BasispoortId,
        request_permission: bool,
    ) -> Result<SynchronizationPermission> {
        self.get(&format!(
            "{institution_id}/uitgever/synchronizationpermission?request-permission={request_permission}"
        ))
        .await
    }

    // TODO: Test manually with a school with ICT coordinator account?
    #[instrument]
    pub async fn relinquish_institution_synchronization_permission(
        &self,
        institution_id: BasispoortId,
    ) -> Result<()> {
        self.delete(&format!(
            "{institution_id}/uitgever/synchronizationpermission"
        ))
        .await
    }

    #[instrument]
    pub async fn get_synchronization_permissions_granted(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<BasispoortId>> {
        self.get(&format!("synchronizationpermission/toegekend/{date}"))
            .await
    }

    #[instrument]
    pub async fn get_synchronization_permissions_revoked(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<BasispoortId>> {
        self.get(&format!("synchronizationpermission/ingetrokken/{date}"))
            .await
    }

    // TODO: Add NAW search client. (crate feature)
}
