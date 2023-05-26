use color_eyre::Result;
use tracing::{debug, info, instrument, trace};

use basispoort_sync_client::{institutions::InstitutionsServiceClient, BasispoortId};

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
    get_institutions_details(&client, &institution_ids).await?;

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
) -> Result<()> {
    debug!("Getting all institutions details...");

    for institution_id in institution_ids {
        debug!("Getting institution {institution_id} details...");
        let institution_details = client.get_institution_details(*institution_id).await?;
        trace!("Institution details: {:#?}", institution_details);
    }

    debug!("Got all institutions details.");

    Ok(())
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
