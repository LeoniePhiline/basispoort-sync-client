use std::{collections::HashSet, env, path::Path};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};
use itertools::Itertools;
#[cfg(not(coverage))]
use tracing::instrument;
use tracing::{debug, error, info, trace};

use basispoort_sync_client::{
    hosted_license_provider::{
        BulkRequest, HostedLicenseProviderClient, MethodDetails, MethodDetailsList, ProductDetails,
        ProductDetailsList, /* UserChainId, UserChainIdList, */ UserIdList,
    },
    rest::RestClient,
    BasispoortId,
};

use util::*;

mod util;

const METHOD_ID: &str = "lifecycle_integration_test_method";

const METHOD_CREATE_NAME: &str = "Test method (POST)";
const METHOD_CREATE_CODE: &str = "method-create";

const METHOD_UPDATE_NAME: &str = "Test method (PUT)";
const METHOD_UPDATE_CODE: &str = "method-update";

// The three-digit user IDs do not exist.
const METHOD_SET_USER_IDS: [BasispoortId; 4] = [123, 128684, 128683, 456];
const METHOD_SET_USER_IDS_EXPECTED: [BasispoortId; 2] = [128683, 128684];
const METHOD_ADD_USER_IDS: [BasispoortId; 4] = [123, 128691, 128690, 456];
const METHOD_ADD_USER_IDS_EXPECTED: [BasispoortId; 2] = [128690, 128691];

const PRODUCT_ID: &str = "lifecycle_integration_test_product";

const PRODUCT_CREATE_NAME: &str = "Test product (POST)";
const PRODUCT_CREATE_CODE: &str = "product-create";

const PRODUCT_UPDATE_NAME: &str = "Test product (PUT)";
const PRODUCT_UPDATE_CODE: &str = "product-update";

// TODO
const PRODUCT_SET_USER_IDS: [BasispoortId; 3] = [127660, 127665, 127666];
const PRODUCT_ADD_USER_IDS: [BasispoortId; 2] = [157478, 157480];

const BULK_GRANT_USER_IDS: [BasispoortId; 11] =
    [1, 2, 127664, 3, 4, 127663, 127667, 5, 6, 128690, 128683];
const BULK_GRANT_USER_IDS_EXPECTED: [BasispoortId; 5] = [127663, 127664, 127667, 128683, 128690];
const BULK_REVOKE_USER_IDS: [BasispoortId; 10] =
    [1, 2, 127663, 127667, 5, 6, 128690, 128689, 128692, 128693];

const APPLICATION_CREATE_ICON_URL: &str =
    "https://www.example.com/path/icon.svg?query=value#anchor";
const APPLICATION_UPDATE_ICON_URL: &str =
    "https://www.example.com/path/icon.png?query=value#anchor";

/// "Hosted Lika" integration test, full application (method, product) lifecycle.
///
/// # Test plan:
///
/// ## Setup
/// - Load environment variables from `.env`.
/// - Initialize tracing.
/// - Create an authenticated REST client for the env-configured Basispoort environment.
/// - Create a specialized "Hosted Lika" REST client, wrapping the authenticated REST client.
/// - Clean up possible left-overs from a previous failed test.
///
/// ## Method
/// - Post a method.
///   - Fetch methods list (confirm contained).
///   - Fetch method (confirm created).
/// - Modify method.
///   - Fetch method (confirm modified).
///
/// ## Method users (classic ID)
/// - Set users for method.
///   - Fetch method users (confirm set).
/// - Add users to method.
///   - Fetch method users (confirm added).
/// - Replace users of method.
///   - Fetch method users (confirm replaced).
/// - Remove users from method.
///   - Fetch method users (confirm removed).
/// - Delete all users from method.
///   - Fetch method users (confirm deleted).
///
/// ## Method users (chain ID)
/// - TODO: Implement chain ID  tests when / if switch to EckId is really happening.
///
/// ## Product
/// - Post a child product.
///   - Fetch method's products list (confirm contained).
/// - Fetch product (confirm created).
///   - Modify product.
///   - Fetch product (confirm modified).
///
/// ## Product users (classic ID)
/// - Set users for product.
///   - Fetch product users (confirm set).
/// - Add users to product.
///   - Fetch product users (confirm added).
/// - Replace users of product.
///   - Fetch product users (confirm replaced).
/// - Remove users from product.
///   - Fetch product users (confirm removed).
/// - Delete all users from product.
///   - Fetch product users (confirm deleted).
///
/// ## Product users (chain ID)
/// - TODO: Implement chain ID  tests when / if switch to EckId is really happening.
///
/// ## Method and product users (bulk request)
/// - Bulk-add users to method and product.
///   - Fetch method users (confirm added).
///   - Fetch product users (confirm added).
/// - Bulk-remove users to method and product.
///   - Fetch method users (confirm removed).
///   - Fetch product users (confirm removed).
///
/// ## Teardown
/// - Delete product.
///   - Fetch method's products list (confirm removed).
/// - Delete method.
///   - Fetch methods list (confirm removed).
#[tokio::test]
async fn hosted_license_provider_application_lifecycle() -> Result<()> {
    // == Setup ==
    let rest_client = setup().await?;

    info!("Create a hosted license provider (\"Hosted Lika\") service REST API client.");
    let client = make_hosted_license_provider_service_client(&rest_client)?;

    info!("Clean up possible left-overs from a previous failed test. This will print a warning if there are no left-overs to clean up.");
    delete_method(&client).await.ok();

    // == Method ==

    info!("Post a method.");
    create_method(&client).await?;

    debug!("Fetch methods list (confirm contained).");
    let methods_list = get_methods(&client).await?;
    assert!(methods_list
        .methods
        .into_iter()
        .any(|method| method.id == METHOD_ID));

    debug!("Fetch method (confirm created).");
    let method = get_method(&client).await?;
    assert_eq!(method.id, METHOD_ID);
    assert_eq!(method.name, METHOD_CREATE_NAME);
    assert_eq!(method.code, Some(METHOD_CREATE_CODE.to_string()));
    assert_eq!(
        method.icon_url,
        Some(APPLICATION_CREATE_ICON_URL.parse().unwrap())
    );

    info!("Modify method.");
    update_method(&client).await?;

    debug!("Fetch method (confirm modified).");
    let method = get_method(&client).await?;
    assert_eq!(method.id, METHOD_ID);
    assert_eq!(method.name, METHOD_UPDATE_NAME);
    assert_eq!(method.code, Some(METHOD_UPDATE_CODE.to_string()));
    assert_eq!(
        method.icon_url,
        Some(APPLICATION_UPDATE_ICON_URL.parse().unwrap())
    );

    // == Method users (classic ID) ==

    info!("Set users for method.");
    set_method_user_ids(&client).await?;

    debug!("Fetch method users (confirm set).");
    let user_id_list = get_method_user_ids(&client).await?; // The listed three-digit IDs do not exist. IDs are not sorted.
    assert_eq!(user_id_list.users, Vec::from(METHOD_SET_USER_IDS_EXPECTED)); // Expect valid IDs, sorted ascending.

    info!("Add users to method.");
    add_method_user_ids(&client).await?;

    debug!("Fetch method users (confirm added).");
    let user_id_list = get_method_user_ids(&client).await?; // The listed three-digit IDs do not (no longer?) exist. (User 789, however, exists!)
    assert!(METHOD_ADD_USER_IDS_EXPECTED
        .iter()
        .all(|user_id| user_id_list.users.iter().any(|id| id == user_id)));

    info!("Remove users from method.");
    remove_method_user_ids(&client).await?;

    debug!("Fetch method users (confirm removed).");
    let user_id_list = get_method_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(METHOD_ADD_USER_IDS_EXPECTED)  // Pre-sorted for easy comparison.
    );

    info!("Replace users for method.");
    set_method_user_ids(&client).await?;

    debug!("Fetch method users (confirm replaced).");
    let user_id_list = get_method_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(METHOD_SET_USER_IDS_EXPECTED)  // Pre-sorted for easy comparison.
    );

    info!("Delete all users from method.");
    delete_method_user_ids(&client).await?;

    debug!("Fetch method users (confirm deleted).");
    let user_id_list = get_method_user_ids(&client).await?;
    assert_eq!(user_id_list.users, Vec::<BasispoortId>::with_capacity(0));

    // == Method users (chain ID) ==

    // TODO: Implement chain ID  tests when / if switch to EckId is really happening.

    // == Product ==

    info!("Post a child product.");
    create_product(&client).await?;

    debug!("Fetch method's products list (confirm contained).");
    let products_list = get_products(&client).await?;
    assert!(products_list
        .products
        .into_iter()
        .any(|product| product.id == PRODUCT_ID));

    debug!("Fetch product (confirm created).");
    let product = get_product(&client).await?;
    assert_eq!(product.id, PRODUCT_ID);
    assert_eq!(product.name, PRODUCT_CREATE_NAME);
    assert_eq!(product.code, Some(PRODUCT_CREATE_CODE.to_string()));
    assert_eq!(
        product.icon_url,
        Some(APPLICATION_CREATE_ICON_URL.parse().unwrap())
    );

    info!("Modify product.");
    update_product(&client).await?;

    debug!("Fetch product (confirm modified).");
    let product = get_product(&client).await?;
    assert_eq!(product.id, PRODUCT_ID);
    assert_eq!(product.name, PRODUCT_UPDATE_NAME);
    assert_eq!(product.code, Some(PRODUCT_UPDATE_CODE.to_string()));
    assert_eq!(
        product.icon_url,
        Some(APPLICATION_UPDATE_ICON_URL.parse().unwrap())
    );

    // == Product users (classic ID) ==

    info!("Set users for product.");
    set_product_user_ids(&client).await?;

    debug!("Fetch product users (confirm set).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(PRODUCT_SET_USER_IDS)  // Pre-sorted for easy comparison.
    );

    info!("Add users to product.");
    add_product_user_ids(&client).await?;

    debug!("Fetch product users (confirm added).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert!(PRODUCT_ADD_USER_IDS
        .iter()
        .all(|user_id| user_id_list.users.iter().any(|id| id == user_id)));

    info!("Remove users from product.");
    remove_product_user_ids(&client).await?;

    debug!("Fetch product users (confirm removed).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(PRODUCT_ADD_USER_IDS)  // Pre-sorted for easy comparison.
    );

    info!("Replace users of product.");
    set_product_user_ids(&client).await?;

    debug!("Fetch product users (confirm replaced).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(PRODUCT_SET_USER_IDS)  // Pre-sorted for easy comparison.
    );

    info!("Delete all users from product.");
    delete_product_user_ids(&client).await?;

    debug!("Fetch product users (confirm deleted).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert_eq!(user_id_list.users, Vec::<BasispoortId>::with_capacity(0));

    // == Product users (chain ID) ==

    // TODO: Implement chain ID tests when / if switch to EckId is really happening.

    // == Method and product users (bulk request) ==

    info!("Bulk-add users to method and product.");
    bulk_grant_permissions(&client).await?;

    debug!("Fetch method users (confirm added).");
    let user_id_list = get_method_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(BULK_GRANT_USER_IDS_EXPECTED)  // Pre-sorted for easy comparison.
    );

    debug!("Fetch product users (confirm added).");
    let user_id_list = get_product_user_ids(&client).await?;
    assert_eq!(
        user_id_list.users, // Returned by the server sorted in ascending order.
        Vec::from(BULK_GRANT_USER_IDS_EXPECTED)  // Pre-sorted for easy comparison.
    );

    info!("Bulk-remove users from method and product.");
    bulk_revoke_permissions(&client).await?;

    debug!("Fetch method users (confirm removed).");
    let user_id_list = get_method_user_ids(&client).await?;
    // Added users minus removed users. Some users in the remove list were never added.
    let mut expected_user_id_list = HashSet::from(BULK_GRANT_USER_IDS_EXPECTED)
        .difference(&HashSet::from(BULK_REVOKE_USER_IDS))
        .copied()
        .collect::<Vec<_>>();
    expected_user_id_list.sort();
    assert_eq!(
        user_id_list.users,    // Returned by the server sorted in ascending order.
        expected_user_id_list  // Sorted manually after set subtraction.
    );

    debug!("Fetch product users (confirm removed).");
    let user_id_list = get_product_user_ids(&client).await?;
    // Added users minus removed users. Some users in the remove list were never added.
    let mut expected_user_id_list = HashSet::from(BULK_GRANT_USER_IDS_EXPECTED)
        .difference(&HashSet::from(BULK_REVOKE_USER_IDS))
        .copied()
        .collect::<Vec<_>>();
    expected_user_id_list.sort();
    assert_eq!(
        user_id_list.users,    // Returned by the server sorted in ascending order.
        expected_user_id_list  // Sorted manually after set subtraction.
    );

    // == Teardown ==

    info!("Delete product.");
    delete_product(&client).await?;

    debug!("Fetch method's products list (confirm removed).");
    let products_list = get_products(&client).await?;
    assert!(!products_list
        .products
        .into_iter()
        .any(|product| product.id == PRODUCT_ID));

    info!("Delete method.");
    delete_method(&client).await?;

    debug!("Fetch methods list (confirm removed).");
    let methods_list = get_methods(&client).await?;
    assert!(!methods_list
        .methods
        .into_iter()
        .any(|method| method.id == METHOD_ID));

    Ok(())
}

// == Setup ==

#[cfg_attr(not(coverage), instrument)]
fn make_hosted_license_provider_service_client(
    rest_client: &RestClient,
) -> Result<HostedLicenseProviderClient<'_>> {
    Ok(HostedLicenseProviderClient::new(
        rest_client,
        &env::var("HOSTED_LICENSE_PROVIDER_IDENTITY_CODE").wrap_err(
            "could not get environment variable `HOSTED_LICENSE_PROVIDER_IDENTITY_CODE`",
        )?,
    ))
}

// == Method ==

#[cfg_attr(not(coverage), instrument)]
async fn get_methods(client: &HostedLicenseProviderClient<'_>) -> Result<MethodDetailsList> {
    debug!("Getting all methods...");
    let methods_list = client.get_methods().await?;

    trace!("Methods: {:#?}", methods_list);
    debug!("Got all methods.");

    Ok(methods_list)
}

#[cfg_attr(not(coverage), instrument)]
async fn get_method(client: &HostedLicenseProviderClient<'_>) -> Result<MethodDetails> {
    debug!("Getting method '{METHOD_ID}'...");
    let method = client.get_method(METHOD_ID).await?;

    trace!("Method: {:#?}", method);
    debug!("Got method '{METHOD_ID}'.");

    Ok(method)
}

#[cfg_attr(not(coverage), instrument)]
async fn create_method(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Creating method '{METHOD_ID}'...");

    let method = MethodDetails::new(METHOD_ID, METHOD_CREATE_NAME)
        .with_code(METHOD_CREATE_CODE)
        .with_icon_from_file(Path::new("./tests/assets/icon_application_create.svg"))
        .await?
        .with_icon_url(APPLICATION_CREATE_ICON_URL)?
        .with_url(
            &env::var("HOSTED_LICENSE_PROVIDER_METHOD_URL_POST").wrap_err(
                "could not get environment variable `HOSTED_LICENSE_PROVIDER_METHOD_URL_POST`",
            )?,
        )?
        .into_teacher_application();

    trace!("Method (Debug): {method:#?}");
    debug!("Method (JSON): {}", serde_json::to_string_pretty(&method)?);

    if let Err(err) = client.create_method(&method).await {
        error!("Error creating method '{METHOD_ID}': {err:#?}");
        bail!(err);
    }

    debug!("Created method '{METHOD_ID}'.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn update_method(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Updating (or creating) method '{METHOD_ID}'...");

    let method = MethodDetails::new(METHOD_ID, METHOD_UPDATE_NAME)
        .with_code(METHOD_UPDATE_CODE)
        .with_icon_from_file(Path::new("./tests/assets/icon_application_update.svg"))
        .await?
        .with_icon_url(APPLICATION_UPDATE_ICON_URL)?
        .with_url(
            &env::var("HOSTED_LICENSE_PROVIDER_METHOD_URL_PUT").wrap_err(
                "could not get environment variable `HOSTED_LICENSE_PROVIDER_METHOD_URL_POST`",
            )?,
        )?
        .into_teacher_application();

    trace!("Method (Debug): {method:#?}");
    debug!("Method (JSON): {}", serde_json::to_string_pretty(&method)?);

    if let Err(err) = client.update_method(&method).await {
        error!("Error updating (or creating) method '{METHOD_ID}': {err:#?}");
        bail!(err);
    }

    debug!("Updated (or created) method '{METHOD_ID}'.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn delete_method(client: &HostedLicenseProviderClient<'_>) -> crate::Result<()> {
    debug!("Deleting method '{METHOD_ID}'...");

    client.delete_method(METHOD_ID).await?;

    debug!("Deleted method '{METHOD_ID}'.");

    Ok(())
}

// == Method users (classic ID) ==

#[cfg_attr(not(coverage), instrument)]
async fn get_method_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<UserIdList> {
    debug!("Getting user IDs with access to method '{METHOD_ID}'...");

    let users = client.get_method_user_ids(METHOD_ID).await?;
    trace!("User IDs with access to method '{METHOD_ID}': {users:#?}");

    debug!("Got user IDs with access to method '{METHOD_ID}'.");

    Ok(users)
}

#[cfg_attr(not(coverage), instrument)]
async fn set_method_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(METHOD_SET_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Granting access to method '{METHOD_ID}' exclusively to user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client.set_method_user_ids(METHOD_ID, &users).await?;

    debug!("Granted access to method '{METHOD_ID}' exclusively to user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn add_method_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(METHOD_ADD_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Granting access to method '{METHOD_ID}' to additional user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client.add_method_user_ids(METHOD_ID, &users).await?;

    debug!("Granted access to method '{METHOD_ID}' to additional user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn remove_method_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(METHOD_SET_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Revoking access to method '{METHOD_ID}' from user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client.remove_method_user_ids(METHOD_ID, &users).await?;

    debug!("Revoked access to method '{METHOD_ID}' from user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn delete_method_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Revoking all access to method '{METHOD_ID}'...");

    client.delete_method_user_ids(METHOD_ID).await?;

    debug!("Revoked all access to method '{METHOD_ID}'.");

    Ok(())
}

// == Method users (chain ID) ==

// TODO: Implement chain ID  tests when / if switch to EckId is really happening.

// #[cfg_attr(not(coverage), instrument)]
// async fn get_method_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     let users = client.get_method_user_chain_ids(METHOD_ID).await?;

//     println!("users: {users:#?}");

//     Ok(())
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn set_method_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/abc".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client.set_method_user_chain_ids(METHOD_ID, &users).await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn add_method_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/def".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client.add_method_user_chain_ids(METHOD_ID, &users).await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn remove_method_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/def".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client.remove_method_user_chain_ids(METHOD_ID, &users).await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn delete_method_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     client.delete_method_user_chain_ids(METHOD_ID).await
// }

// == Product ==

#[cfg_attr(not(coverage), instrument)]
async fn get_products(client: &HostedLicenseProviderClient<'_>) -> Result<ProductDetailsList> {
    debug!("Getting all products of method '{METHOD_ID}'...");
    let products_list = client.get_products(METHOD_ID).await?;

    trace!("Products of method '{METHOD_ID}': {:#?}", products_list);
    debug!("Got all products of method '{METHOD_ID}'.");

    Ok(products_list)
}

#[cfg_attr(not(coverage), instrument)]
async fn get_product(client: &HostedLicenseProviderClient<'_>) -> Result<ProductDetails> {
    debug!("Getting product '{PRODUCT_ID}' of method '{METHOD_ID}'...");
    let product = client.get_product(METHOD_ID, PRODUCT_ID).await?;

    trace!("Product: {:#?}", product);
    debug!("Got product '{PRODUCT_ID}' of method '{METHOD_ID}'.");

    Ok(product)
}

#[cfg_attr(not(coverage), instrument)]
async fn create_product(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Creating product '{PRODUCT_ID}' in '{METHOD_ID}'...");

    let product = ProductDetails::new(
        PRODUCT_ID,
        PRODUCT_CREATE_NAME,
        &env::var("HOSTED_LICENSE_PROVIDER_PRODUCT_URL_POST").wrap_err(
            "could not get environment variable `HOSTED_LICENSE_PROVIDER_PRODUCT_URL_POST`",
        )?,
    )?
    .with_code(PRODUCT_CREATE_CODE)
    .with_icon_from_file(Path::new("./tests/assets/icon_application_create.svg"))
    .await?
    .with_icon_url(APPLICATION_CREATE_ICON_URL)?
    .into_teacher_application();

    trace!("Product (Debug): {product:#?}");
    debug!(
        "Product (JSON): {}",
        serde_json::to_string_pretty(&product)?
    );

    if let Err(err) = client.create_product(METHOD_ID, &product).await {
        error!("Error creating product '{PRODUCT_ID}' in method '{METHOD_ID}': {err:#?}");
        bail!(err);
    }

    debug!("Created product '{PRODUCT_ID}' in '{METHOD_ID}'...");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn update_product(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Updating (or creating) product '{PRODUCT_ID}' in '{METHOD_ID}'...");

    let product = ProductDetails::new(
        PRODUCT_ID,
        PRODUCT_UPDATE_NAME,
        &env::var("HOSTED_LICENSE_PROVIDER_PRODUCT_URL_PUT").wrap_err(
            "could not get environment variable `HOSTED_LICENSE_PROVIDER_PRODUCT_URL_POST`",
        )?,
    )?
    .with_code(PRODUCT_UPDATE_CODE)
    .with_icon_from_file(Path::new("./tests/assets/icon_application_update.svg"))
    .await?
    .with_icon_url(APPLICATION_UPDATE_ICON_URL)?
    .into_teacher_application();

    trace!("Product (Debug): {product:#?}");
    debug!(
        "Product (JSON): {}",
        serde_json::to_string_pretty(&product)?
    );

    if let Err(err) = client.update_product(METHOD_ID, &product).await {
        error!(
            "Error updating (or creating) product '{PRODUCT_ID}' in method '{METHOD_ID}': {err:#?}"
        );
        bail!(err);
    }

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn delete_product(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Deleting product '{PRODUCT_ID}' of method '{METHOD_ID}'...");

    client.delete_product(METHOD_ID, PRODUCT_ID).await?;

    debug!("Deleted product '{PRODUCT_ID}' of method '{METHOD_ID}'.");

    Ok(())
}

// == Product users (classic ID) ==

#[cfg_attr(not(coverage), instrument)]
async fn get_product_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<UserIdList> {
    debug!("Getting user IDs with access to product '{PRODUCT_ID}' of method '{METHOD_ID}'...");

    let users = client.get_product_user_ids(METHOD_ID, PRODUCT_ID).await?;
    trace!("User IDs with access to product '{PRODUCT_ID}' of method '{METHOD_ID}': {users:#?}");

    debug!("Got user IDs with access to product '{PRODUCT_ID}' of method '{METHOD_ID}'.");

    Ok(users)
}

#[cfg_attr(not(coverage), instrument)]
async fn set_product_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(PRODUCT_SET_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Granting access to product '{PRODUCT_ID}' of method '{METHOD_ID}' exclusively to user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client
        .set_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
        .await?;

    debug!("Granted access to product '{PRODUCT_ID}' of method '{METHOD_ID}' exclusively to user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn add_product_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(PRODUCT_ADD_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Granting access to product '{PRODUCT_ID}' of method '{METHOD_ID}' to additional user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client
        .add_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
        .await?;

    debug!("Granted access to product '{PRODUCT_ID}' of method '{METHOD_ID}' to additional user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn remove_product_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(PRODUCT_SET_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Revoking access to product '{PRODUCT_ID}' of method '{METHOD_ID}' from user IDs {user_ids_fmt}...");

    let users: UserIdList = user_ids.into();

    trace!("UserIdList (Debug): {users:#?}");
    debug!(
        "UserIdList (JSON): {}",
        serde_json::to_string_pretty(&users)?
    );

    client
        .remove_product_user_ids(METHOD_ID, PRODUCT_ID, &users)
        .await?;

    debug!("Revoked access to product '{PRODUCT_ID}' of method '{METHOD_ID}' from user IDs {user_ids_fmt}.");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn delete_product_user_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    debug!("Revoking all access to product '{PRODUCT_ID}' of method '{METHOD_ID}'...");

    client
        .delete_product_user_ids(METHOD_ID, PRODUCT_ID)
        .await?;

    debug!("Revoked all access to product '{PRODUCT_ID}' of method '{METHOD_ID}'.");

    Ok(())
}

// == Product users (chain ID) ==

// TODO: Implement chain ID  tests when / if switch to EckId is really happening.

// #[cfg_attr(not(coverage), instrument)]
// async fn get_product_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     let users = client
//         .get_product_user_chain_ids(METHOD_ID, PRODUCT_ID)
//         .await?;

//     println!("users: {users:#?}");

//     Ok(())
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn set_product_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/abc".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client
//         .set_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
//         .await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn add_product_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/def".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client
//         .add_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
//         .await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn remove_product_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     // TODO: How do valid chain IDs look?
//     let users: UserChainIdList = vec![UserChainId {
//         institution_id: 123,
//         chain_id: "https://ketenid.nl/def".into(),
//     }]
//     .into();
//     println!("{users:#?}");

//     client
//         .remove_product_user_chain_ids(METHOD_ID, PRODUCT_ID, &users)
//         .await
// }

// #[cfg_attr(not(coverage), instrument)]
// async fn delete_product_user_chain_ids(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
//     client
//         .delete_product_user_chain_ids(METHOD_ID, PRODUCT_ID)
//         .await
// }

// == Method and product users (bulk request) ==

#[cfg_attr(not(coverage), instrument)]
async fn bulk_grant_permissions(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(BULK_GRANT_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Granting access to product '{PRODUCT_ID}' and method '{METHOD_ID}' to bulk user IDs {user_ids_fmt}...");

    let bulk_request = BulkRequest {
        method_ids: vec![METHOD_ID.into()],
        product_ids: vec![PRODUCT_ID.into()],
        user_ids,
        user_chain_ids: vec![
            // TODO: Implement chain ID  tests when / if switch to EckId is really happening.
            // UserChainId {
            //     institution_id: 123,
            //     chain_id: "https://ketenid.nl/abc".into(),
            // },
            // UserChainId {
            //     institution_id: 123,
            //     chain_id: "https://ketenid.nl/def".into(),
            // },
        ],
    };

    trace!("BulkRequest (Debug): {bulk_request:#?}");
    debug!(
        "BulkRequest (JSON): {}",
        serde_json::to_string_pretty(&bulk_request)?
    );

    client.bulk_grant_permissions(&bulk_request).await?;

    debug!("Granted access to product '{PRODUCT_ID}' and method '{METHOD_ID}' to bulk user IDs {user_ids_fmt}...");

    Ok(())
}

#[cfg_attr(not(coverage), instrument)]
async fn bulk_revoke_permissions(client: &HostedLicenseProviderClient<'_>) -> Result<()> {
    let user_ids = Vec::from(BULK_REVOKE_USER_IDS);
    let user_ids_fmt = user_ids.iter().join(", ");
    debug!("Revoking access to product '{PRODUCT_ID}' and method '{METHOD_ID}' from bulk user IDs {user_ids_fmt}...");

    let bulk_request = BulkRequest {
        method_ids: vec![METHOD_ID.into()],
        product_ids: vec![PRODUCT_ID.into()],
        user_ids,
        user_chain_ids: vec![
            // TODO: Implement chain ID  tests when / if switch to EckId is really happening.
            // UserChainId {
            //     institution_id: 123,
            //     chain_id: "https://ketenid.nl/abc".into(),
            // },
            // UserChainId {
            //     institution_id: 123,
            //     chain_id: "https://ketenid.nl/123".into(),
            // },
        ],
    };

    trace!("BulkRequest (Debug): {bulk_request:#?}");
    debug!(
        "BulkRequest (JSON): {}",
        serde_json::to_string_pretty(&bulk_request)?
    );

    client.bulk_revoke_permissions(&bulk_request).await?;

    debug!("Revoked access to product '{PRODUCT_ID}' and method '{METHOD_ID}' from bulk user IDs {user_ids_fmt}.");

    Ok(())
}
