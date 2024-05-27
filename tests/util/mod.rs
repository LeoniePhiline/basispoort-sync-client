use std::env;

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use tracing::info;
#[cfg(not(coverage))]
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

use basispoort_sync_client::{
    institutions::InstitutionsServiceClient,
    rest::{RestClient, RestClientBuilder},
};

// == Setup ==

static REST_CLIENT: OnceCell<Result<RestClient>> = OnceCell::const_new();

/// Use `OnceCell` to initialize tracing only once, even if multiple tests
/// within the same integration test crate are being run in parallel.
pub async fn setup() -> Result<RestClient> {
    let rest_client = REST_CLIENT.get_or_init(|| async {
        color_eyre::install()?;

        dotenv().ok();
        tracing_init()?;

        info!("Create an authenticated REST API client for the env-configured Basispoort environment.");
        let rest_client = make_rest_client().await?;

        Ok(rest_client)
     }).await;

    match rest_client {
        Ok(rest_client) => Ok(rest_client.clone()),
        Err(err) => Err(eyre!(err)),
    }
}

fn tracing_init() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_thread_names(true)
                .with_line_number(true)
                .with_filter(
                    // Use `RUST_LOG=target[span{field=value}]=level` for fine-grained verbosity control.
                    // See https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html
                    tracing_subscriber::EnvFilter::builder().from_env_lossy(),
                ),
        )
        .with(ErrorLayer::default())
        .try_init()
        .map_err(|_| eyre!("Tracing initialization failed"))?;

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn make_rest_client() -> Result<RestClient> {
    Ok(RestClientBuilder::new(
        &env::var("IDENTITY_CERT_FILE")
            .wrap_err("could not get environment variable `IDENTITY_CERT_FILE`")?,
        env::var("ENVIRONMENT")
            .wrap_err("could not get environment variable `ENVIRONMENT`")?
            .parse()?,
    )
    .build()
    .await?)
}

#[allow(dead_code)] // This function is not used in the `hosted_license_provider_lifecycle` integration test.
#[cfg_attr(not(coverage), instrument)]
pub fn make_institutions_service_client(rest_client: &RestClient) -> InstitutionsServiceClient<'_> {
    InstitutionsServiceClient::new(rest_client)
}
