use std::fmt::Debug;

use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Serialize};
#[cfg(not(coverage))]
use tracing::instrument;

use crate::{error::Error, rest, BasispoortId, Result};

use super::model::*;

#[derive(Debug)]
pub struct InstitutionsServiceClient<'a> {
    rest_client: &'a rest::RestClient,
    base_path: &'static str,
}

impl<'a> InstitutionsServiceClient<'a> {
    #[cfg_attr(not(coverage), instrument)]
    pub fn new(rest_client: &'a rest::RestClient) -> Self {
        InstitutionsServiceClient {
            rest_client,
            // TODO: "/v2/licenties" as separate service (and crate feature)?
            base_path: "rest/v2/",
        }
    }

    fn make_path(&self, path: &str) -> String {
        format!("{}{}", self.base_path, path)
    }

    #[cfg_attr(not(coverage), instrument(skip(self)))]
    async fn get<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        self.rest_client.get(&self.make_path(path)).await
    }

    #[cfg_attr(not(coverage), instrument(skip(self, payload)))]
    async fn post<P: Serialize + Debug + ?Sized, T: DeserializeOwned + Debug + ?Sized>(
        &self,
        path: &str,
        payload: &P,
    ) -> Result<T> {
        self.rest_client.post(&self.make_path(path), payload).await
    }

    #[cfg_attr(not(coverage), instrument(skip(self)))]
    async fn delete<T: DeserializeOwned + Debug + ?Sized>(&self, path: &str) -> Result<T> {
        self.rest_client.delete(&self.make_path(path)).await
    }

    /*
     * Institutions service
     */

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_ids(&self) -> Result<Vec<BasispoortId>> {
        self.get("instellingen").await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_overview(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionOverview> {
        self.get(&format!("instellingen/{institution_id}")).await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_details(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionDetails> {
        self.get(&format!("instellingen/{institution_id}/details"))
            .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_groups(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionGroups> {
        self.get(&format!("instellingen/{institution_id}/groepen"))
            .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_students(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionStudents> {
        self.get(&format!("instellingen/{institution_id}/leerlingen"))
            .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_students_by_id(
        &self,
        institution_id: BasispoortId,
        student_ids: &[BasispoortId],
    ) -> Result<InstitutionStudents> {
        self.post(
            &format!("instellingen/{institution_id}/leerlingen"),
            student_ids,
        )
        .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_students_by_chain_id(
        &self,
        institution_id: BasispoortId,
        student_chain_ids: &[String], // TODO: type def?
    ) -> Result<InstitutionStudents> {
        self.post(
            &format!("instellingen/{institution_id}/leerlingen_eckid"),
            student_chain_ids,
        )
        .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_staff(
        &self,
        institution_id: BasispoortId,
    ) -> Result<InstitutionStaff> {
        self.get(&format!("instellingen/{institution_id}/staf"))
            .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_shortcut_reference(
        &self,
        institution_id: BasispoortId,
    ) -> Result<String> {
        self.get(&format!("instellingen/{institution_id}/ref"))
            .await
    }

    // TODO: Test requesting sync permission manually with a school with ICT coordinator account.
    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_institution_synchronization_permission(
        &self,
        institution_id: BasispoortId,
        request_permission: bool,
    ) -> Result<SynchronizationPermission> {
        self.get(&format!(
            "instellingen/{institution_id}/uitgever/synchronizationpermission?request-permission={request_permission}"
        ))
        .await
    }

    // TODO: Test manually with a school with ICT coordinator account?
    #[cfg_attr(not(coverage), instrument)]
    pub async fn relinquish_institution_synchronization_permission(
        &self,
        institution_id: BasispoortId,
    ) -> Result<()> {
        self.delete(&format!(
            "instellingen/{institution_id}/uitgever/synchronizationpermission"
        ))
        .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_synchronization_permissions_granted(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<BasispoortId>> {
        self.get(&format!(
            "instellingen/synchronizationpermission/toegekend/{date}"
        ))
        .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn get_synchronization_permissions_revoked(
        &self,
        date: &NaiveDate,
    ) -> Result<Vec<BasispoortId>> {
        self.get(&format!(
            "instellingen/synchronizationpermission/ingetrokken/{date}"
        ))
        .await
    }

    #[cfg_attr(not(coverage), instrument)]
    pub async fn find_institutions(
        &self,
        predicate: InstitutionsSearchPredicate<'_>,
    ) -> Result<Vec<InstitutionSearchResult>> {
        self.get(&format!(
            "nawsearch?{query}",
            query = String::try_from(&predicate).map_err(Error::SerializeSearchPredicate)?
        ))
        .await
    }
}
