use std::env;

use color_eyre::{
    eyre::{eyre, WrapErr},
    Result,
};
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

use basispoort_sync_client::rest::{RestClient, RestClientBuilder};

// == Setup ==

pub fn tracing_init() -> Result<()> {
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

#[instrument]
pub async fn make_rest_client() -> Result<RestClient> {
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
