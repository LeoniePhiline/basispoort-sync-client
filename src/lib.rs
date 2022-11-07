pub mod error;
pub mod hosted_sites;
pub mod institutions;
pub mod rest;

pub type Result<T> = std::result::Result<T, Box<crate::error::Error>>;

// TODO: Move into tests/integration_test.rs
#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::File;
    use std::io::Read;

    use dotenvy::dotenv;

    use crate::hosted_sites::{HostedSitesClient, MethodDetails, SiteTag};
    use crate::rest::RestClient;

    use super::*;

    fn make_rest_client() -> Result<RestClient> {
        rest::RestClientBuilder::new(
            &env::var("IDENTITY_CERT_FILE").unwrap(),
            rest::Environment::Test,
        )
        .build()
    }

    fn make_sites_client(rest_client: &RestClient) -> Result<HostedSitesClient<'_>> {
        Ok(hosted_sites::HostedSitesClient::new(
            rest_client,
            &env::var("HOSTED_SITES_IDENTITY_CODE").unwrap(),
        ))
    }

    #[tokio::test]
    async fn get_all_methods() -> Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let all_methods = make_sites_client(&rest_client)?.get_all_methods().await?;

        println!("all_methods: {:#?}", all_methods);

        Ok(())
    }

    // TODO: Make an integration tests which posts a method, then fetches it. (Then modifies it, adds, removes users, then deletes it.)
    #[tokio::test]
    async fn post_method() -> std::result::Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let mut icon_data = Vec::new();
        File::open("./tests/assets/icon_site_post.svg")?.read_to_end(&mut icon_data)?;

        let method = MethodDetails {
            id: "abcdef123456".into(),
            name: "Test (POST)".into(),
            icon: Some(format!("image/svg+xml,{}", base64::encode(icon_data))),
            url: Some(env::var("HOSTED_SITES_METHOD_URL_POST").unwrap()),
            tags: vec![SiteTag::TeacherApplication],
        };

        println!("{method:?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .post_method(&method)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_method() -> Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let method = make_sites_client(&rest_client)?
            .get_method("abcdef123456")
            .await?;

        println!("method: {:#?}", method);

        Ok(())
    }

    #[tokio::test]
    async fn put_method() -> std::result::Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let mut icon_data = Vec::new();
        File::open("./tests/assets/icon_site_put.svg")?.read_to_end(&mut icon_data)?;

        let method = MethodDetails {
            id: "abcdef123456".into(),
            name: "Test (PUT)".into(),
            icon: Some(format!("image/svg+xml,{}", base64::encode(icon_data))),
            url: Some(env::var("HOSTED_SITES_METHOD_URL_PUT").unwrap()),
            tags: vec![SiteTag::TeacherApplication],
        };

        println!("{method:?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?.put_method(&method).await?;

        Ok(())
    }

    #[tokio::test]
    async fn delete_method() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_method("abcdef123456")
            .await?;

        Ok(())
    }
}
