use std::{collections::HashSet, env, path::Path};

use color_eyre::{
    eyre::{bail, eyre, WrapErr},
    Result,
};
use dotenvy::dotenv;
use itertools::Itertools;
use tracing::{debug, error, info, instrument, trace};
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

use basispoort_sync_client::institutions::{Institution, InstitutionsServiceClient};
use basispoort_sync_client::rest::{RestClient, RestClientBuilder};

use util::*;

mod util;

#[tokio::test]
async fn get_all_institutions() -> Result<()> {
    // == Setup ==
    info!("Load environment variables from `.env`.");
    dotenv().ok();

    info!("Initialize tracing.");
    tracing_init()?;

    info!("Create an authenticated REST API client for the env-configured Basispoort environment.");
    let rest_client = make_rest_client().await?;

    info!("Create an institutions (\"Instellingen V2\") service REST API client.");
    let client = make_institutions_service_client(&rest_client);

    info!("Fetch all institutions' IDs.");
    let institution_ids = get_institution_ids(&client).await?;

    info!("Fetching all institutions.");
    let institutions = get_institutions(&client, &institution_ids).await?;

    // TODO: Run with RUST_LOG=trace,h2=warn,hyper=warn,mio=warn,reqwest=warn,rustls=warn,tokio_util=warn,want=warn cargo test --test institutions_service

    // TODO: Add fields to `Institution`

    // TODO: Add next endpoints...

    Ok(())
}

// == Setup ==

#[instrument]
fn make_institutions_service_client(rest_client: &RestClient) -> InstitutionsServiceClient<'_> {
    InstitutionsServiceClient::new(rest_client)
}

// == Institutions service ==

#[instrument]
async fn get_institution_ids(client: &InstitutionsServiceClient<'_>) -> Result<Vec<u64>> {
    debug!("Getting all institution IDs...");
    let institution_ids = client.get_institution_ids().await?;

    trace!("Institution IDs: {:#?}", institution_ids);
    debug!("Got all institution IDs.");

    Ok(institution_ids)
}

#[instrument]
async fn get_institutions(
    client: &InstitutionsServiceClient<'_>,
    institution_ids: &Vec<u64>,
) -> Result<Vec<Institution>> {
    debug!("Getting all institutions...");

    let mut institutions = Vec::with_capacity(institution_ids.len());

    for institution_id in institution_ids {
        let institution = client.get_institution(*institution_id).await?;

        trace!("Institution: {:#?}", institution);
        institutions.push(institution);
    }

    trace!("Institutions: {:#?}", institutions);
    debug!("Got all institutions.");

    Ok(institutions)
}
