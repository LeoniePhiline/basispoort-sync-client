use chrono::{DateTime, Datelike, Days, Local, NaiveDate};
use color_eyre::Result;
#[cfg(not(coverage))]
use tracing::instrument;
use tracing::{debug, info};

use basispoort_sync_client::institutions::InstitutionsServiceClient;

use util::*;

mod util;

#[tokio::test]
async fn synchronization_permissions_mutations() -> Result<()> {
    // == Setup ==
    let rest_client = setup().await?;

    info!("Create an institutions (\"Instellingen V2\") service REST API client.");
    let client = make_institutions_service_client(&rest_client);

    info!("Fetch synchronization permission grants and revokations.");
    get_synchronization_permissions_mutations(&client).await?;

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn get_synchronization_permissions_mutations(
    client: &InstitutionsServiceClient<'_>,
) -> Result<()> {
    let local: DateTime<Local> = Local::now();
    let date = NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();

    let test_range = 0..=365u64; // TODO: Reduce?
    for days_back in test_range {
        let date = date.checked_sub_days(Days::new(days_back)).unwrap();
        debug!("Getting synchronization permissions mutations on {date}...");

        let synchronization_permissions_granted = client
            .get_synchronization_permissions_granted(&date)
            .await?;
        debug!(
            "Synchronization permissions granted on {date}: {:#?}",
            synchronization_permissions_granted
        );

        let synchronization_permissions_revoked = client
            .get_synchronization_permissions_revoked(&date)
            .await?;
        debug!(
            "synchronization permissions revoked on {date}: {:#?}",
            synchronization_permissions_revoked
        );
    }

    Ok(())
}
