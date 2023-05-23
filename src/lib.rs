pub use url::Url;

pub mod error;
pub mod hosted_sites;
pub mod institutions;
pub mod rest;

pub type Result<T> = std::result::Result<T, Box<crate::error::Error>>;
