pub use url::Url;

pub mod error;
pub mod hosted_license_provider;
pub mod institutions;
pub mod rest;

pub type Result<T> = std::result::Result<T, Box<crate::error::Error>>;
