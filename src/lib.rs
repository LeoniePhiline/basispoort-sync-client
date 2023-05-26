pub use url::Url;

pub mod error;

#[cfg(feature = "hosted-license-provider")]
pub mod hosted_license_provider;

#[cfg(feature = "institutions")]
pub mod institutions;

// TODO: Add licenses client. (crate feature)

pub mod rest;

pub type Result<T> = std::result::Result<T, Box<crate::error::Error>>;

pub type BasispoortId = i64; // Defined as signed `int64`, as OpenAPI knows no unsigned types. ¯\_(ツ)_/¯
