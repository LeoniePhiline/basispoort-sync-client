use color_eyre::Result;
use tracing::{debug, info, instrument, trace};

use basispoort_sync_client::{
    institutions::{InstitutionDetails, InstitutionsSearchPredicate, InstitutionsServiceClient},
    BasispoortId,
};

use util::*;

mod util;

#[tokio::test]
async fn institution_properties_service() -> Result<()> {
    // == Setup ==
    let rest_client = setup().await?;

    info!("Create an institutions (\"Instellingen V2\") service REST API client.");
    let client = make_institutions_service_client(&rest_client);

    info!("Fetch all institutions' IDs.");
    let institution_ids = get_institution_ids(&client).await?;

    info!("Fetch all institution details.");
    let institutions_details = get_institutions_details(&client, &institution_ids).await?;

    info!("Fetch all institutions overviews.");
    get_institutions_overviews(&client, &institution_ids).await?;

    info!("Fetch all institutions groups.");
    get_institutions_groups(&client, &institution_ids).await?;

    info!("Fetch all institutions students.");
    get_institutions_students(&client, &institution_ids).await?;

    info!("Fetch all institutions staff.");
    get_institutions_staff(&client, &institution_ids).await?;

    info!("Fetch all institutions shortcut references.");
    get_institutions_shortcut_references(&client, &institution_ids).await?;

    info!("Fetch all institutions synchronization permissions.");
    get_institutions_synchronization_permissions(&client, &institution_ids).await?;

    info!("Searching for institutions by known details.");

    search_institutions_by_brin_code(&client, &institutions_details).await?;

    Ok(())
}

#[instrument]
async fn get_institution_ids(client: &InstitutionsServiceClient<'_>) -> Result<Vec<BasispoortId>> {
    debug!("Getting all institution IDs...");
    let institution_ids = client.get_institution_ids().await?;

    trace!("Institution IDs: {:#?}", institution_ids);
    debug!("Got all institution IDs.");

    Ok(institution_ids)
}

#[instrument]
async fn get_institutions_details(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<Vec<(BasispoortId, InstitutionDetails)>> {
    debug!("Getting all institutions details...");

    let mut institutions_details = Vec::with_capacity(institution_ids.len());

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} details...");
        let institution_details = client.get_institution_details(*institution_id).await?;
        trace!("Institution details: {:#?}", institution_details);

        institutions_details.push((*institution_id, institution_details));
    }

    debug!("Got all institutions details.");

    Ok(institutions_details)
}

#[instrument]
async fn get_institutions_overviews(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions overviews...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} overview...");
        let institution_overview = client.get_institution_overview(*institution_id).await?;
        trace!("Institution overview: {:#?}", institution_overview);
    }

    debug!("Got all institutions overviews.");

    Ok(())
}

#[instrument]
async fn get_institutions_groups(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions groups...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} groups...");
        let institution_groups = client.get_institution_groups(*institution_id).await?;
        trace!("Institution groups: {:#?}", institution_groups);
    }

    debug!("Got all institutions groups.");

    Ok(())
}

#[instrument]
async fn get_institutions_students(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions students...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} students...");
        let institution_students = client.get_institution_students(*institution_id).await?;
        trace!("Institution students: {:#?}", institution_students);

        debug!("Getting institution {institution_id} students by ID...");
        let student_ids = institution_students
            .students
            .iter()
            .map(|student| student.id)
            .collect::<Vec<_>>();
        let institution_students_by_id = client
            .get_institution_students_by_id(*institution_id, &student_ids)
            .await?;
        trace!(
            "Institution students by ID: {:#?}",
            institution_students_by_id
        );

        debug!("Getting institution {institution_id} students by chain ID...");
        let student_chain_ids = institution_students
            .students
            .into_iter()
            .filter_map(|student| student.chain_id)
            .collect::<Vec<_>>();
        let institution_students_by_chain_id = client
            .get_institution_students_by_chain_id(*institution_id, &student_chain_ids)
            .await?;
        trace!(
            "Institution students by chain ID: {:#?}",
            institution_students_by_chain_id
        );
    }

    debug!("Got all institutions students.");

    Ok(())
}

#[instrument]
async fn get_institutions_staff(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions staff...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} staff...");
        let institution_staff = client.get_institution_staff(*institution_id).await?;
        trace!("Institution staff: {:#?}", institution_staff);
    }

    debug!("Got all institutions staff.");

    Ok(())
}

#[instrument]
async fn get_institutions_shortcut_references(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions shortcut references...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} shortcut reference...");
        let institution_shortcut_reference = client
            .get_institution_shortcut_reference(*institution_id)
            .await?;
        trace!(
            "Institution shortcut reference: {:#?}",
            institution_shortcut_reference
        );
    }

    debug!("Got all institutions shortcut references.");

    Ok(())
}

#[instrument]
async fn get_institutions_synchronization_permissions(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<BasispoortId>,
) -> Result<()> {
    debug!("Getting all institutions synchronization permissions...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} synchronization permission...");
        let institution_synchronization_permission = client
            .get_institution_synchronization_permission(*institution_id, false)
            .await?;
        trace!(
            "Institution synchronization permission: {:#?}",
            institution_synchronization_permission
        );
    }

    debug!("Got all institutions synchronization permissions.");

    Ok(())
}

#[instrument]
async fn search_institutions_by_brin_code(
    client: &InstitutionsServiceClient<'_>,
    institutions_details: &Vec<(BasispoortId, InstitutionDetails)>,
) -> Result<()> {
    debug!("Searching for institutions per BRIN code without branch code...");

    for (institution_id, institution_details) in institutions_details {
        if let Some(brin_code) = &institution_details.brin_code {
            if !brin_code.is_empty() {
                debug!("Searching for institution per BRIN code: {}...", brin_code);
                let search_results = client
                    .find_institutions(InstitutionsSearchPredicate::new().with_brin_code(brin_code))
                    .await?;
                trace!(
                    "Search results for BRIN code '{}': {:#?}",
                    brin_code,
                    search_results
                );

                // Assert the known institution is found in the search results.
                // TODO: All input schools are always active - think of a way to test the activeOnly search predicate flag.
                assert!(search_results
                    .into_iter()
                    .any(|search_result| &search_result.id == institution_id));
            } else {
                debug!(
                    "Institution [{institution_id}] '{}' has an empty BRIN code.",
                    institution_details.name.as_deref().unwrap_or_default()
                );
            }
        } else {
            debug!(
                "Institution [{institution_id}] '{}' has no BRIN code.",
                institution_details.name.as_deref().unwrap_or_default()
            );
        }
    }

    Ok(())
}

