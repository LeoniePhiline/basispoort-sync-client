use std::collections::HashSet;
use std::fmt::Debug;
use std::path::Path;

use base64::{engine::general_purpose::STANDARD as base64, Engine as _};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncReadExt};

use crate::error::Error;
use crate::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct MethodDetailsList {
    #[serde(rename = "methodes")]
    pub methods: Vec<MethodDetails>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MethodDetails {
    pub id: String,
    pub code: Option<String>,
    #[serde(rename = "naam")]
    pub name: String,
    pub icon: Option<String>,
    pub icon_url: Option<crate::Url>,
    pub url: Option<crate::Url>,
    pub tags: HashSet<ApplicationTag>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductDetailsList {
    #[serde(rename = "producten")]
    pub products: Vec<ProductDetails>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProductDetails {
    pub id: String,
    pub code: Option<String>,
    #[serde(rename = "naam")]
    pub name: String,
    pub icon: Option<String>,
    pub icon_url: Option<crate::Url>,
    pub url: crate::Url,
    pub tags: HashSet<ApplicationTag>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum ApplicationTag {
    #[serde(rename = "leerkrachtApplicatie")]
    TeacherApplication,
    #[serde(rename = "toetsApplicatie")]
    TestApplication,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserIdList {
    #[serde(rename = "gebruikers")]
    pub users: Vec<u64>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct UserChainIdList {
    #[serde(rename = "gebruikers")]
    pub users: Vec<UserChainId>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserChainId {
    #[serde(rename = "instellingId")]
    pub institution_id: u64,
    #[serde(rename = "eckId")]
    pub chain_id: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct BulkRequest {
    #[serde(rename = "methodes")]
    pub method_ids: Vec<String>,
    #[serde(rename = "producten")]
    pub product_ids: Vec<String>,
    #[serde(rename = "gebruikers")]
    pub user_ids: Vec<u64>,
    #[serde(rename = "gebruikerEckIds")]
    pub user_chain_ids: Vec<UserChainId>,
}

// == Implementations ==

impl MethodDetails {
    /// Create a new `MethodDetails`.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            code: None,
            name: name.into(),
            icon: None,
            icon_url: None,
            url: None,
            // There is currently only one possible tag.
            tags: HashSet::with_capacity(1),
        }
    }

    /// Return a new `MethodeDetails with the provided code.
    pub fn with_code(self, code: impl Into<String>) -> Self {
        Self {
            code: Some(code.into()),
            ..self
        }
    }

    /// Return a new `MethodDetails` with the provided icon.
    pub fn with_icon(self, icon: impl Into<String>) -> Self {
        Self {
            icon: Some(icon.into()),
            ..self
        }
    }

    /// Read the icon from the provided file, then return a new `MethodDetails` with the provided icon.
    pub async fn with_icon_from_file(self, path: &Path) -> Result<Self> {
        Ok(self.with_icon(icon_from_file(path).await?))
    }

    /// Return a new `MethodeDetails` with the provided icon URL.
    pub fn with_icon_url(self, icon_url: &str) -> Result<Self> {
        Ok(Self {
            icon_url: Some(
                crate::Url::parse(icon_url).map_err(|source| Error::ParseUrl {
                    url: icon_url.to_string(),
                    source,
                })?,
            ),
            ..self
        })
    }

    /// Return a new `MethodDetails` with the provided URL.
    pub fn with_url(self, url: &str) -> Result<Self> {
        Ok(Self {
            url: Some(crate::Url::parse(url).map_err(|source| Error::ParseUrl {
                url: url.to_string(),
                source,
            })?),
            ..self
        })
    }

    /// Turn the `MethodeDetails` into a teacher application.
    pub fn into_teacher_application(self) -> Self {
        let mut tags = self.tags;
        tags.insert(ApplicationTag::TeacherApplication);

        Self { tags, ..self }
    }

    /// Turn the `MethodeDetails` into a test application.
    pub fn into_test_application(self) -> Self {
        let mut tags = self.tags;
        tags.insert(ApplicationTag::TeacherApplication);

        Self { tags, ..self }
    }
}

impl ProductDetails {
    /// Create a new `ProductDetails`. Other than with `MethodDetails`, the `url` field is obligatory.
    pub fn new(id: impl Into<String>, name: impl Into<String>, url: &str) -> Result<Self> {
        Ok(Self {
            id: id.into(),
            code: None,
            name: name.into(),
            icon: None,
            icon_url: None,
            url: crate::Url::parse(url).map_err(|source| Error::ParseUrl {
                url: url.to_string(),
                source,
            })?,
            // There is currently only one possible tag.
            tags: HashSet::with_capacity(1),
        })
    }

    /// Return a new `MethodeDetails with the provided code.
    pub fn with_code(self, code: impl Into<String>) -> Self {
        Self {
            code: Some(code.into()),
            ..self
        }
    }

    /// Return a new `ProductDetails` with the added icon.
    pub fn with_icon(self, icon: impl Into<String>) -> Self {
        Self {
            icon: Some(icon.into()),
            ..self
        }
    }

    /// Read the icon from the provided file, then return a new `ProductDetails` with the added icon.
    pub async fn with_icon_from_file(self, path: &Path) -> Result<Self> {
        Ok(self.with_icon(icon_from_file(path).await?))
    }

    /// Return a new `ProductDetails` with the provided icon URL.
    pub fn with_icon_url(self, icon_url: &str) -> Result<Self> {
        Ok(Self {
            icon_url: Some(
                crate::Url::parse(icon_url).map_err(|source| Error::ParseUrl {
                    url: icon_url.to_string(),
                    source,
                })?,
            ),
            ..self
        })
    }

    /// Turn the `ProductDetails` into a teacher application.
    pub fn into_teacher_application(self) -> Self {
        let mut tags = self.tags;
        tags.insert(ApplicationTag::TeacherApplication);

        Self { tags, ..self }
    }

    /// Return a new `MethodDetails` with the provided icon.
    pub fn into_test_application(self) -> Self {
        let mut tags = self.tags;
        tags.insert(ApplicationTag::TeacherApplication);

        Self { tags, ..self }
    }
}

/// Read an icon from file, encode it as base64 string and optionally prefix it by mime type.
async fn icon_from_file(path: &Path) -> Result<String> {
    let mut icon_data = Vec::new();
    File::open(path)
        .await
        .map_err(|source| Error::OpenIconFile {
            path: path.to_owned(),
            source,
        })?
        .read_to_end(&mut icon_data)
        .await
        .map_err(|source| Error::ReadIconFile {
            path: path.to_owned(),
            source,
        })?;

    let mime_type_prefix = match path.extension() {
        Some(ext) => match ext.to_str() {
            Some("svg") => "image/svg+xml,",
            Some("png") => "image/png,",
            Some(_) | None => "",
        },
        None => "",
    };

    Ok(format!("{mime_type_prefix}{}", base64.encode(icon_data)))
}

impl From<Vec<u64>> for UserIdList {
    fn from(users: Vec<u64>) -> Self {
        UserIdList { users }
    }
}

impl From<UserIdList> for Vec<u64> {
    fn from(list: UserIdList) -> Self {
        list.users
    }
}

impl From<Vec<UserChainId>> for UserChainIdList {
    fn from(users: Vec<UserChainId>) -> Self {
        UserChainIdList { users }
    }
}

impl From<UserChainIdList> for Vec<UserChainId> {
    fn from(list: UserChainIdList) -> Self {
        list.users
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn builds_method_with_svg_icon() -> Result<()> {
        let method = MethodDetails::new("method-with-svg-icon", "Method with SVG icon")
            .with_code("method-code")
            .with_icon_from_file(Path::new("./tests/assets/icon_application_create.svg"))
            .await?
            .with_icon_url("https://www.example.com/path/icon.svg?query=value#anchor")?
            .with_url("https://www.example.com/path/?query=value#anchor")?
            .into_teacher_application();

        assert_eq!(
            method,
            MethodDetails {
                id: String::from("method-with-svg-icon"),
                code: Some(String::from("method-code")),
                name: String::from("Method with SVG icon"),
                icon: Some(String::from("image/svg+xml,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB2aWV3Qm94PSIwIDAgMTA2IDEwNiI+CiAgPCEtLSBPd25lZCBieSB0aGUgUnVzdCBmb3VuZGF0aW9uLCBsaWNlbnNlZCB1bmRlciBDQy1CWSBodHRwczovL2NyZWF0aXZlY29tbW9ucy5vcmcvbGljZW5zZXMvYnkvNC4wLwogICAgICAgTW9kaWZpY2F0aW9uczogT3B0aW1pemVkIHRocm91Z2ggU1ZHT01HLgogICAgICAgU291cmNlOiBodHRwczovL2NvbW1vbnMud2lraW1lZGlhLm9yZy93aWtpL0ZpbGU6UnVzdF9wcm9ncmFtbWluZ19sYW5ndWFnZV9ibGFja19sb2dvLnN2ZyAtLT4KICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSg1MyA1MykiPgogICAgPHBhdGggc3Ryb2tlPSIjMDAwIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBkPSJNLTguNS0xNC41aDEzYzggMCA4IDggMCA4aC0xM1ptLTMxIDM3aDQwdi0xMWgtOXYtOGgxMGMxMSAwIDUgMTkgMTQgMTloMjV2LTE5aC02djJjMCA4LTkgNy0xMCAycy01LTktNi05YzE1LTggNi0yNC02LTI0aC00N3YxMWgxMHYyNmgtMTVaIi8+CiAgICA8ZyBtYXNrPSJ1cmwoI2EpIj4KICAgICAgPGNpcmNsZSByPSI0MyIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjMDAwIiBzdHJva2Utd2lkdGg9IjkiLz4KICAgICAgPHBhdGggaWQ9ImIiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLXdpZHRoPSIzIiBkPSJtNDYgMyA1LTMtNS0zeiIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxMS4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyMi41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMy44KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSg0NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNTYuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNjcuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNzguOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoOTApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDEwMS4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxMTIuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTIzLjgpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDEzNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTQ2LjMpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDE1Ny41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxNjguOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTgwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxOTEuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjAyLjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDIxMy44KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyMjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDIzNi4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyNDcuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjU4LjgpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDI3MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjgxLjMpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDI5Mi41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMDMuOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMzE1KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMjYuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMzM3LjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDM0OC44KSIvPgogICAgICA8cGF0aCBpZD0iYyIgc3Ryb2tlPSIjMDAwIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjYiIGQ9Im0tNy00MiA3IDcgNy03eiIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjIiB0cmFuc2Zvcm09InJvdGF0ZSg3MikiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYyIgdHJhbnNmb3JtPSJyb3RhdGUoMTQ0KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjIiB0cmFuc2Zvcm09InJvdGF0ZSgyMTYpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2MiIHRyYW5zZm9ybT0icm90YXRlKDI4OCkiLz4KICAgIDwvZz4KICAgIDxtYXNrIGlkPSJhIj4KICAgICAgPHBhdGggZmlsbD0iI2ZmZiIgZD0iTS02MC02MEg2MFY2MEgtNjB6Ii8+CiAgICAgIDxjaXJjbGUgaWQ9ImQiIGN5PSItNDAiIHI9IjMiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjZCIgdHJhbnNmb3JtPSJyb3RhdGUoNzIpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2QiIHRyYW5zZm9ybT0icm90YXRlKDE0NCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjZCIgdHJhbnNmb3JtPSJyb3RhdGUoMjE2KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNkIiB0cmFuc2Zvcm09InJvdGF0ZSgyODgpIi8+CiAgICA8L21hc2s+CiAgPC9nPgo8L3N2Zz4K")),
                icon_url: Some("https://www.example.com/path/icon.svg?query=value#anchor".parse().unwrap()),
                url: Some(
                    "https://www.example.com/path/?query=value#anchor"
                        .parse()
                        .unwrap()
                ),
                tags: HashSet::from([ApplicationTag::TeacherApplication])
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn builds_method_with_png_icon() -> Result<()> {
        let method = MethodDetails::new("method-with-png-icon", "Method with SVG icon")
            .with_icon_from_file(Path::new("./tests/assets/icon_application_update.png"))
            .await?
            .with_url("https://www.example.com/path/?query=value#anchor")?
            .into_teacher_application();

        assert_eq!(
            method,
            MethodDetails {
                id: String::from("method-with-png-icon"),
                code: None,
                name: String::from("Method with SVG icon"),
                icon: Some(String::from("image/png,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")),
                icon_url: None,
                url: Some(
                    "https://www.example.com/path/?query=value#anchor"
                        .parse()
                        .unwrap()
                ),
                tags: HashSet::from([ApplicationTag::TeacherApplication])
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn builds_product_with_svg_icon() -> Result<()> {
        let product = ProductDetails::new(
            "product-with-svg-icon",
            "Product with SVG icon",
            "https://www.example.com/path/?query=value#anchor",
        )?
        .with_code("product-code")
        .with_icon_from_file(Path::new("./tests/assets/icon_application_create.svg"))
        .await?
        .with_icon_url("https://www.example.com/path/icon.svg?query=value#anchor")?
        .into_teacher_application();

        assert_eq!(
            product,
            ProductDetails {
                id: String::from("product-with-svg-icon"),
                code: Some(String::from("product-code")),
                name: String::from("Product with SVG icon"),
                icon: Some(String::from("image/svg+xml,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB2aWV3Qm94PSIwIDAgMTA2IDEwNiI+CiAgPCEtLSBPd25lZCBieSB0aGUgUnVzdCBmb3VuZGF0aW9uLCBsaWNlbnNlZCB1bmRlciBDQy1CWSBodHRwczovL2NyZWF0aXZlY29tbW9ucy5vcmcvbGljZW5zZXMvYnkvNC4wLwogICAgICAgTW9kaWZpY2F0aW9uczogT3B0aW1pemVkIHRocm91Z2ggU1ZHT01HLgogICAgICAgU291cmNlOiBodHRwczovL2NvbW1vbnMud2lraW1lZGlhLm9yZy93aWtpL0ZpbGU6UnVzdF9wcm9ncmFtbWluZ19sYW5ndWFnZV9ibGFja19sb2dvLnN2ZyAtLT4KICA8ZyB0cmFuc2Zvcm09InRyYW5zbGF0ZSg1MyA1MykiPgogICAgPHBhdGggc3Ryb2tlPSIjMDAwIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBkPSJNLTguNS0xNC41aDEzYzggMCA4IDggMCA4aC0xM1ptLTMxIDM3aDQwdi0xMWgtOXYtOGgxMGMxMSAwIDUgMTkgMTQgMTloMjV2LTE5aC02djJjMCA4LTkgNy0xMCAycy01LTktNi05YzE1LTggNi0yNC02LTI0aC00N3YxMWgxMHYyNmgtMTVaIi8+CiAgICA8ZyBtYXNrPSJ1cmwoI2EpIj4KICAgICAgPGNpcmNsZSByPSI0MyIgZmlsbD0ibm9uZSIgc3Ryb2tlPSIjMDAwIiBzdHJva2Utd2lkdGg9IjkiLz4KICAgICAgPHBhdGggaWQ9ImIiIHN0cm9rZT0iIzAwMCIgc3Ryb2tlLWxpbmVqb2luPSJyb3VuZCIgc3Ryb2tlLXdpZHRoPSIzIiBkPSJtNDYgMyA1LTMtNS0zeiIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxMS4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyMi41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMy44KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSg0NSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNTYuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNjcuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoNzguOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoOTApIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDEwMS4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxMTIuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTIzLjgpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDEzNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTQ2LjMpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDE1Ny41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxNjguOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMTgwKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgxOTEuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjAyLjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDIxMy44KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyMjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDIzNi4zKSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgyNDcuNSkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjU4LjgpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDI3MCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMjgxLjMpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDI5Mi41KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMDMuOCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMzE1KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNiIiB0cmFuc2Zvcm09InJvdGF0ZSgzMjYuMykiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYiIgdHJhbnNmb3JtPSJyb3RhdGUoMzM3LjUpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2IiIHRyYW5zZm9ybT0icm90YXRlKDM0OC44KSIvPgogICAgICA8cGF0aCBpZD0iYyIgc3Ryb2tlPSIjMDAwIiBzdHJva2UtbGluZWpvaW49InJvdW5kIiBzdHJva2Utd2lkdGg9IjYiIGQ9Im0tNy00MiA3IDcgNy03eiIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjIiB0cmFuc2Zvcm09InJvdGF0ZSg3MikiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjYyIgdHJhbnNmb3JtPSJyb3RhdGUoMTQ0KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNjIiB0cmFuc2Zvcm09InJvdGF0ZSgyMTYpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2MiIHRyYW5zZm9ybT0icm90YXRlKDI4OCkiLz4KICAgIDwvZz4KICAgIDxtYXNrIGlkPSJhIj4KICAgICAgPHBhdGggZmlsbD0iI2ZmZiIgZD0iTS02MC02MEg2MFY2MEgtNjB6Ii8+CiAgICAgIDxjaXJjbGUgaWQ9ImQiIGN5PSItNDAiIHI9IjMiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjZCIgdHJhbnNmb3JtPSJyb3RhdGUoNzIpIi8+CiAgICAgIDx1c2UgeGxpbms6aHJlZj0iI2QiIHRyYW5zZm9ybT0icm90YXRlKDE0NCkiLz4KICAgICAgPHVzZSB4bGluazpocmVmPSIjZCIgdHJhbnNmb3JtPSJyb3RhdGUoMjE2KSIvPgogICAgICA8dXNlIHhsaW5rOmhyZWY9IiNkIiB0cmFuc2Zvcm09InJvdGF0ZSgyODgpIi8+CiAgICA8L21hc2s+CiAgPC9nPgo8L3N2Zz4K")),
                icon_url: Some("https://www.example.com/path/icon.svg?query=value#anchor".parse().unwrap()),
                url: "https://www.example.com/path/?query=value#anchor".parse().unwrap(),
                tags: HashSet::from([ApplicationTag::TeacherApplication])
            }
        );

        Ok(())
    }

    #[tokio::test]
    async fn builds_product_with_png_icon() -> Result<()> {
        let product = ProductDetails::new(
            "product-with-png-icon",
            "Product with SVG icon",
            "https://www.example.com/path/?query=value#anchor",
        )?
        .with_icon_from_file(Path::new("./tests/assets/icon_application_update.png"))
        .await?
        .into_teacher_application();

        assert_eq!(
            product,
            ProductDetails {
                id: String::from("product-with-png-icon"),
                code: None,
                name: String::from("Product with SVG icon"),
                icon: Some(String::from("image/png,iVBORw0KGgoAAAANSUhEUgAAAEsAAABLCAYAAAA4TnrqAAABhGlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TS0UqDi0oxSFDdbKLijpqFYpQIdQKrTqYXPoFTRqSFBdHwbXg4Mdi1cHFWVcHV0EQ/ABxdXFSdJES/5cWWsR4cNyPd/ced+8AoVFhmtUzC2i6baaTCTGbWxWDrwhgEGFEMS0zy5iTpBQ8x9c9fHy9i/Ms73N/jn41bzHAJxLPMsO0iTeIpzZtg/M+cYSVZJX4nHjMpAsSP3JdafEb56LLAs+MmJn0PHGEWCx2sdLFrGRqxJPEMVXTKV/ItljlvMVZq9RY+578haG8vrLMdZrDSGIRS5AgQkENZVRgI06rToqFNO0nPPxR1y+RSyFXGYwcC6hCg+z6wf/gd7dWYWK8lRRKAIEXx/kYAYK7QLPuON/HjtM8AfzPwJXe8VcbwMwn6fWOFjsCBraBi+uOpuwBlzvA0JMhm7Ir+WkKhQLwfkbflAPCt0DfWqu39j5OH4AMdZW6AQ4OgdEiZa97vLu3u7d/z7T7+wHa1nLQSHD57AAAAAZiS0dEAB8AHwAfgYFSlAAAAAlwSFlzAAAN1wAADdcBQiibeAAAAAd0SU1FB+cFFxMbGoiPTgUAAAvTSURBVHja7Zt7cFTVHcc/5+4m2c0mISQUBJW3QgiIEVEUrOADVJIxlIoPIqKgMorYQdtatSJi0drRjiCOaDtiBamPEUrCw0QU0IS3oshDfAAqBCSIJNns855f/9gl2SS7YbMkgON+Z+7s3t+e37nnfM/vcR53IY444ogjjjjiiCOOOOKII4444ojjV4Gf59Kjeg7XnI5tM07FQ2UaRtWLvFL9IlPryR8n0epjKcKyqjkMC/3NOYeB1bMpqnqevr8qsqrbMVSZTMTk2apZTKklpC0PoOmFiVJ+XpC5JAD8PIse4qMIk5FK8+CpIku1qgW9is1ZRSHwuQMeVlPwAFQ9zysKJiIAmAI3Wkw2aAs7EBwhVfwRH/OxUgZ0C8qqHNBBTcUF4HyO6wX+ojUPpj3Ix79Ysiqf4zJDsSZ4u9ECY2wWyp0m5QhtQ4pqJIyVB8j0QcDCjskEbkxxstjp4O/A/QhK4OXUP3F3a/bH2pqVWzRZUjccA03hE6fJHgghSsKEA6l3l6AayBS86EpmujLpHVKud2u7YYuRVfUso5WQpwxecExlE4AIWbWdDHy2PUaUakxKOKLCy4RMgcwGLlJLVuVMMi1wL8IZyQ9zr1Jhaz11blj9DGuUcFnwtkSEeSimKeHcmAiJTTZZoIeCu2pjn4VOjkcoP60sy9D4pa7RVyu4Gh2VlbQkcS+oBjJtxX/auaH4cdXaaetZTrP1/B58p5Ssyhn0MiBXYHOKg/VqKi50BJeWUyuz+kmQaRhOH30NxVAtVKTM5I2TFrOc05kH3Ba89QI7EPq3tpXEKNuDkEpdQvA6nibppFmWCDtVXZZLhAZEnQprilyma/0gxo6T6oYWk1Itp5/LRSlbe1LJslWzqcaOF0g8mcH6RGUKUKqVyXL9mc5acY0ofIZQXQNt0VhOB8tRzdQTuKbm/sC6UiDd0Oyyz2Z1iwV450PMQ9cG9FYnRJ1c0r32djjU9OPPx6JzQ5M1SAhZsY7+KXbDCLK10RAV9X5WspsFaA5iAiagQR27zODVhAwd1AvqEirTLSQzY5QJs1t080/NxoPJ3FBCYm5ca8liI32fXbOkRQO83IDFLbhFTu3SpcVlikMkkRTcM4s9wDvvIwfNFKXoh5AN2E7reVTsdQnCbhRbBZY65vFK88maxKtKMb41LeKwG37yQMdkSLFGp7u3Cpw+yGrbKlnTbe+OQ00Pu1/SRMzSzEbjbY3A7LK04dOblrFp9CIuWZrMma/D1DIwzchx6TtfOtvHvceOm99lyAobOe/C1oqWjX1i8o9IRDVJluNlPlEmT7RGYN7U+04uvfJa8vPzKSgowKfhpR3wxObwBPv98O5ZtzPg8uGMGjWK0aNHs+sojCwBl69l2mb42ZLs48mYs6HtKE+j2dCS6dtvwrdGx9pndOrUqfb73J1Q7W2su2QvaEfdLnK7du0AqHDDyn2R21ZvOqPDTHHqZKIMblNv442ZLJedAZhktWT63vwjLF5WjIigtaa4uLhun8wHnx9uXP8H5bB48WK8Xi9Op5OioqJanZX7I8/xmjGISnvJiXnqUHUrfRGWBfeCWiyQ7quG90rfY8SIEXg8HsrKyuoV8/iCHQ3BETds2rSJfv364fF42Lt3b+1v+2tqJ5cn2rZ5rnyutCVwj3qb6qjJct9ID60pJvQEJcr0rY5T5mx74LOkpCTsIHWxBzsfopudBouAXbt2NSrf0Ra0kJbJhre6PVxUnc9NKYvZclw3rBnLWVpRgqZjre8fZzkTGg+O54YD02FY+/DWPCgTeiQ3dpuJXSEtzLAmGjChSzPjpj/kCh9Keln8rHPncn/DsVcNXK+91csaoFdrTiKPeGH8Rig+WCfr1wbeGgRd7eF1V1fAmPVQGVzyDsmE6VlwaUYz29G8PfdFSYoCVUhN/d2QsaS5vKxR0D/0IQKurQex/eRCdUuHM1PBqmqXCydE3LqfYPPP0DcNBmcE621Ct9IPn/0MnWzQw3FiROw6Ct9XoRMs6LZJWLul4UmxNt6bV5prbctYUY8s9yiuEcXyMOzmvrOTcUoxBsCq8HdMwXpeBiRZQuxTtbAlRsBRPxxwQ9sEaN+MYwctYATb6PRD8T72aOEhJSwALIbi/vyu3ARcEjoh8Ahnpi/lSL0An2RltdvHd0Dn0A5oIVVZmITmQqC7X/jw+yreOezmyas68RurESgrAsqI3Pkvq2HWt/BZJbhN8EvAmu7pBtmpx+/sqgqY/wN0c8A5yQEr2+2CS9Jh5Bnhx0oLHHIHPjsm18m/c/KOdjHBSGKigEVgvSWLObgoqDeGisfSiwJENYpZ3lz6m4oPgIwQ8QZ7IRe/nsUFhkEpkGQoLvIZ7BicwUudUuoe8PVR8JiQaQtcCUaAlCe+BJsBt3cOZi/Ao2HlIfjnt5DTBmb0hqQwsz6vhkd3QvdkGH82WBqw8ulR+N8B+GuvwDO8ZoCgfTVw0And0qBXer2TqQVtllMgoBb25XMRzrEockb0pNzmpwI45i/rbHaGqLfrcm2jAXHmMdAQ3gfSGvrtgr5MQPiXgiW3bON6gJqRvEnQRbUgH5XDTx6UUmC3wPIj8FAvOCOCywjw8h4oOggLB0BqSNbzaJj0GUzqAuelRba6Sj9M2wn5mVATTACGwp+TwaH2yXQMKVrWVnGFWo5nQRZXYVAiwsMF23nKlctYYP6xqGQIFyQtrX9s1mgsHYVsFIPrAGdthwxmybUkjf2CfwNzBPIW9gnMeKudTPAL2/watKAGd+BI91QeEGHGR5WU/60PtEsAvw5/mRomdIbJXeF362F3Fexzwg9OuHsL3NUZ+qRE1vdrSDbgoZ4w/wBblDCpm4MxV3Rie4aNjiHl1hgWctXywAt1YvAHYG1iNs8Eu3lHiPU93pCoJrdoXHlcgVAE2IMm8Jh9KTM+HIp1/yHeA3xjtwVelD08nLMxWE3d23kuhLkuYYrdiP5VzPcPoRd8x4Eb21O++gipQzvQ8/J20evvdXGkSzI7lXChhLwAJ/C6y8qd5wSJmt+PfkpTqoULbt3O166RdEfxdZCPdbZqLlOrGu/LN3m6485juAiLg4S5UGTbC9n9Wm8yrRbWK2HyLdsDafXAcLoBJUCPaDsnoE2hQoT9VoNyQ1GN5vkOJZTuvZI+wEBDs/NLJ4nf1JDmF7q0SaR3RgJ9OiSRnZlAB6Wa7EOFgsmdV/JmqHBBNguVsOqW7cwFcOUyA3gUqFImObblfBPTUVjQwpYADoGi5CLyAN7oQ7Yonh67LXAP8P0IMjBZABFfza4Q4T6rsMubQHmXQRxsav+oKeweSm8zcGCaHq7ZCl6y+pjR5eO6bAbwxrm0I4GXb97GaAUiN2Bxu9gNnA3cYS/i1RM6N3TlMQRhGZCKYpy9kNcB5mfze6XZOHYHe0PLfzuMMSimABeHWX9+pWF8zw8oi3VC+dVvyTEVK4BwC6dCq4+JPcv4MZzuG1nkK82Gm79kfzBB3aQUC5XiXVsho0/4kDVY6aVKsRyFYNLfvixA0H/OwzHu87pkEIqdwxijhP82eo6gUbylNTOz1rC1OUR9MYRxKGaHZusQrEhKIf9YbAqH+T1JK/iaytpQk8tmgQ6+BPqnLeJwi5AFUJPHIAUrFBTbCgPTheNh6xAeUSriDqQILDeEdwwrH2atYk+4QmWXYLcprjMUDwoMihD/3jyawPhhq3BH3Z/ruFgZlKG42l7IBzGf7kR8QC4XKVipDX7rWMKn0ehsGcx0gceiKPpITikzATYOYrBhMFngLGAgRHynyqsU0y8o5SnVzKWzK4+nRZOavJR7W+yQtd7pdBEbDEWBMpkYrc75pUwzhXtMwWNK4F8CYa6PUtJ49pjOwHWUCpQB5zVBVLFhcNGAUmaqGPYYlNDLnxjVIMZmWaFLo8QiPmuOzoZLOd+AZ4ErGkziX3MrpgwppSqMC2YkGIxCuNpQZIpQjeJTNIsuXNu8eNcQnuvol7Qs+jpa9R8WkbB+COcaJoMAqyGsHbAu9rfx4ogjjjjiiCOOOOKII4444ogjjjh++fg/KboVXt0xhlUAAAAASUVORK5CYII=")),
                icon_url: None,
                url: "https://www.example.com/path/?query=value#anchor".parse().unwrap(),
                tags: HashSet::from([ApplicationTag::TeacherApplication])
            }
        );

        Ok(())
    }
}
