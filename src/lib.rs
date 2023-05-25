pub use url::Url;

pub mod error;

#[cfg(feature = "hosted-license-provider")]
pub mod hosted_license_provider;

#[cfg(feature = "institutions")]
pub mod institutions;

pub mod rest;

pub type Result<T> = std::result::Result<T, Box<crate::error::Error>>;
