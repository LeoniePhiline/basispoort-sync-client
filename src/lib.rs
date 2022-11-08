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

    use crate::hosted_sites::{
        BulkRequest, HostedSitesClient, MethodDetails, ProductDetails, SiteTag, UserChainId,
        UserChainIdList, UserIdList,
    };
    use crate::rest::RestClient;

    use super::*;

    const METHOD_ID: &str = "abcdef123456";
    const PRODUCT_ID: &str = "123456abcdef";

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
    async fn get_methods() -> Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let methods = make_sites_client(&rest_client)?.get_methods().await?;

        println!("methods: {:#?}", methods);

        Ok(())
    }

    // TODO: Make an integration tests which posts a method, then fetches it. (Then modifies it, adds, removes users, then deletes it.)
    #[tokio::test]
    async fn post_method() -> std::result::Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let mut icon_data = Vec::new();
        File::open("./tests/assets/icon_site_post.svg")?.read_to_end(&mut icon_data)?;

        let method = MethodDetails {
            id: METHOD_ID.into(),
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
            .get_method(METHOD_ID)
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
            id: METHOD_ID.into(),
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
    async fn put_method_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123, 456, 789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .put_method_user_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn get_method_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let users = make_sites_client(&rest_client)?
            .get_method_user_ids(METHOD_ID)
            .await?;

        println!("users: {users:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn delete_method_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_method_user_ids(METHOD_ID)
            .await
    }

    #[tokio::test]
    async fn add_method_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123456, 456789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .add_method_user_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn remove_method_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123456, 456789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .remove_method_user_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn put_method_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/abc".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .put_method_user_chain_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn get_method_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let users = make_sites_client(&rest_client)?
            .get_method_user_chain_ids(METHOD_ID)
            .await?;

        println!("users: {users:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn add_method_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/def".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .add_method_user_chain_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn remove_method_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/def".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .remove_method_user_chain_ids(METHOD_ID, &users)
            .await
    }

    #[tokio::test]
    async fn delete_method_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_method_user_chain_ids(METHOD_ID)
            .await
    }

    #[tokio::test]
    async fn get_products() -> Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let products = make_sites_client(&rest_client)?
            .get_products(METHOD_ID)
            .await?;

        println!("products of {}: {:#?}", METHOD_ID, products);

        Ok(())
    }

    // TODO: Make an integration tests which posts a product, then fetches it. (Then modifies it, adds, removes users, then deletes it.)
    #[tokio::test]
    async fn post_product() -> std::result::Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let mut icon_data = Vec::new();
        File::open("./tests/assets/icon_site_post.svg")?.read_to_end(&mut icon_data)?;

        let product = ProductDetails {
            id: PRODUCT_ID.into(),
            name: "Test (POST)".into(),
            icon: Some(format!("image/svg+xml,{}", base64::encode(icon_data))),
            url: env::var("HOSTED_SITES_PRODUCT_URL_POST").unwrap(),
            tags: vec![SiteTag::TeacherApplication],
        };
        println!("{product:?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .post_product(METHOD_ID, &product)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn get_product() -> Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let product = make_sites_client(&rest_client)?
            .get_product(METHOD_ID, PRODUCT_ID)
            .await?;

        println!("product: {:#?}", product);

        Ok(())
    }

    #[tokio::test]
    async fn put_product() -> std::result::Result<(), Box<dyn std::error::Error>> {
        dotenv().ok();

        let mut icon_data = Vec::new();
        File::open("./tests/assets/icon_site_put.svg")?.read_to_end(&mut icon_data)?;

        let product = ProductDetails {
            id: PRODUCT_ID.into(),
            name: "Test (PUT)".into(),
            icon: Some(format!("image/svg+xml,{}", base64::encode(icon_data))),
            url: env::var("HOSTED_SITES_PRODUCT_URL_PUT").unwrap(),
            tags: vec![SiteTag::TeacherApplication],
        };

        println!("{product:?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .put_product(METHOD_ID, &product)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn put_product_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123, 456, 789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .put_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn get_product_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let users = make_sites_client(&rest_client)?
            .get_product_user_ids(METHOD_ID, PRODUCT_ID)
            .await?;

        println!("users: {users:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn add_product_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123456, 456789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .add_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn remove_product_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let users: UserIdList = vec![123456, 456789].into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .remove_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn delete_product_user_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_product_user_ids(METHOD_ID, PRODUCT_ID)
            .await
    }

    #[tokio::test]
    async fn put_product_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/abc".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .put_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn get_product_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        let users = make_sites_client(&rest_client)?
            .get_product_user_chain_ids(METHOD_ID, PRODUCT_ID)
            .await?;

        println!("users: {users:#?}");

        Ok(())
    }

    #[tokio::test]
    async fn add_product_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/def".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .add_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn remove_product_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        // TODO: How do valid chain IDs look?
        let users: UserChainIdList = vec![UserChainId {
            institution_id: 123,
            chain_id: "https://ketenid.nl/def".into(),
        }]
        .into();
        println!("{users:#?}");

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .remove_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
            .await
    }

    #[tokio::test]
    async fn delete_product_user_chain_ids() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_product_user_chain_ids(METHOD_ID, PRODUCT_ID)
            .await
    }

    #[tokio::test]
    async fn bulk_grant_permissions() -> crate::Result<()> {
        dotenv().ok();

        let bulk_request = BulkRequest {
            method_ids: vec![METHOD_ID.into()],
            product_ids: vec![PRODUCT_ID.into()],
            user_ids: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            chain_ids: vec![
                UserChainId {
                    institution_id: 123,
                    chain_id: "https://ketenid.nl/abc".into(),
                },
                UserChainId {
                    institution_id: 123,
                    chain_id: "https://ketenid.nl/def".into(),
                },
            ],
        };

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .bulk_grant_permissions(&bulk_request)
            .await
    }

    #[tokio::test]
    async fn bulk_revoke_permissions() -> crate::Result<()> {
        dotenv().ok();

        let bulk_request = BulkRequest {
            method_ids: vec![METHOD_ID.into()],
            product_ids: vec![PRODUCT_ID.into()],
            user_ids: vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22],
            chain_ids: vec![
                UserChainId {
                    institution_id: 123,
                    chain_id: "https://ketenid.nl/abc".into(),
                },
                UserChainId {
                    institution_id: 123,
                    chain_id: "https://ketenid.nl/123".into(),
                },
            ],
        };

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .bulk_revoke_permissions(&bulk_request)
            .await
    }

    #[tokio::test]
    async fn delete_product() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_product(METHOD_ID, PRODUCT_ID)
            .await
    }
    #[tokio::test]
    async fn delete_method() -> crate::Result<()> {
        dotenv().ok();

        let rest_client = make_rest_client()?;
        make_sites_client(&rest_client)?
            .delete_method(METHOD_ID)
            .await
    }
}
