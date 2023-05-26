use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Institution {
    // TODO: First test parsing this, then add more fields.
    // TODO: See https://test-rest.basispoort.nl/rest/doc/partnerv2.html#get-/v2/instellingen/{instellingId}
    #[serde(rename = "metaResult")]
    result_metadata: ResultMetadata,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultMetadata {
    mutation_timestamp: chrono::DateTime<chrono::Utc>,
    generation_timestamp: chrono::DateTime<chrono::Utc>,
}
