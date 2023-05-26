use std::collections::HashSet;

use chrono::NaiveDate;
use serde::Deserialize;

use crate::BasispoortId;

// LasKey
pub type AdministrativeKey = String;

#[derive(Debug, Deserialize)]
pub struct InstitutionOverview {
    #[serde(rename = "groepen")]
    pub groups: Vec<Group>,

    #[serde(rename = "subgroepen")]
    pub sub_groups: Vec<Group>,

    #[serde(rename = "leerlingen")]
    pub students: Vec<Student>,

    #[serde(rename = "medewerkers")]
    pub staff: Vec<StaffMember>,

    #[serde(rename = "actief")]
    pub active: bool,

    #[serde(rename = "gefuseerdNaar")]
    pub merged_into: Option<BasispoortId>,

    #[serde(rename = "metaResult")]
    pub result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionDetails {
    #[serde(rename = "naam")]
    pub name: Option<String>,

    #[serde(rename = "straat")]
    pub street: Option<String>,

    #[serde(rename = "huisnummer")]
    pub house_number: Option<String>,

    #[serde(rename = "huisnummertoevoeging")]
    pub house_number_postfix: Option<String>,

    #[serde(rename = "postcode")]
    pub postal_code: Option<String>,

    #[serde(rename = "woonplaats")]
    pub city: Option<String>,

    #[serde(rename = "brincode")]
    pub brin_code: Option<String>,

    #[serde(rename = "dependancecode")]
    pub branch_code: Option<String>,

    #[serde(rename = "schoolkey")]
    pub administrative_key: Option<AdministrativeKey>,

    #[serde(rename = "instellingRef")]
    pub shortcut_reference: Option<String>,

    #[serde(rename = "bestuurscode")]
    pub governance_code: Option<String>,

    #[serde(rename = "actief")]
    pub active: bool,

    #[serde(rename = "gefuseerdNaar")]
    pub merged_into: Option<BasispoortId>,

    #[serde(rename = "metaResult")]
    pub result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionGroups {
    #[serde(rename = "groepen")]
    pub groups: Vec<Group>,

    #[serde(rename = "subgroepen")]
    pub sub_groups: Vec<Group>,

    #[serde(rename = "metaResult")]
    pub result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionStudents {
    #[serde(rename = "leerlingen")]
    pub students: Vec<Student>,

    #[serde(rename = "metaResult")]
    pub result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
pub struct InstitutionStaff {
    #[serde(rename = "medewerkers")]
    pub staff: Vec<StaffMember>,

    #[serde(rename = "metaResult")]
    pub result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "lasKey")]
    pub administrative_key: Option<AdministrativeKey>,

    #[serde(rename = "naam")]
    pub name: Option<String>,

    #[serde(rename = "jaargroep")]
    pub year_group: Option<String>,

    #[serde(rename = "omschrijving")]
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Student {
    pub id: BasispoortId,

    #[serde(rename = "eckid")]
    pub chain_id: Option<String>,

    #[serde(rename = "lasKey")]
    pub administrative_key: Option<AdministrativeKey>,

    #[serde(rename = "persoonsgegevens")]
    pub personal_data: PersonalData,

    #[serde(rename = "jaargroep")]
    pub year_group: Option<String>,

    #[serde(rename = "groep")]
    pub group: Option<AdministrativeKey>,

    #[serde(rename = "subgroepen")]
    pub sub_groups: Vec<AdministrativeKey>,
}

#[derive(Debug, Deserialize)]
pub struct StaffMember {
    pub id: BasispoortId,

    #[serde(rename = "eckid")]
    pub chain_id: Option<String>,

    #[serde(rename = "lasKey")]
    pub administrative_key: Option<AdministrativeKey>,

    #[serde(rename = "persoonsgegevens")]
    pub personal_data: PersonalData,

    #[serde(rename = "emailadres")]
    pub email: Option<String>,

    #[serde(rename = "einddatum")]
    pub end_date: Option<NaiveDate>,

    #[serde(rename = "rollen")]
    pub roles: HashSet<StaffMemberRole>,

    #[serde(rename = "groepen")]
    pub groups: Vec<AdministrativeKey>,

    #[serde(rename = "subgroepen")]
    pub sub_groups: Vec<AdministrativeKey>,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub enum StaffMemberRole {
    #[serde(rename = "Leerkracht")]
    Teacher,
    #[serde(rename = "ICTCoordinator")]
    ITCoordinator,
    #[serde(rename = "IBRTer")]
    AssistantTeacher,
    #[serde(rename = "Stagiair")]
    TraineeTeacher,
    #[serde(rename = "Inval")]
    ReplacementTeacher,
}

#[derive(Debug, Deserialize)]
pub struct PersonalData {
    #[serde(rename = "achternaam")]
    pub last_name: Option<String>,

    #[serde(rename = "voornaam")]
    pub first_name: Option<String>,

    #[serde(rename = "voorvoegsel")]
    pub prefix: Option<String>,

    #[serde(rename = "voorletters")]
    pub initials: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultMetadata {
    pub mutation_timestamp: chrono::DateTime<chrono::Utc>,
    pub generation_timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynchronizationPermission {
    pub has_synchronization_permission: bool,
}
