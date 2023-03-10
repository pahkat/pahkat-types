use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;
#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TypedBuilder,
)]
#[cfg_attr(feature = "poem-openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::SimpleObject))]
pub struct PackageRef {
    #[serde(rename = "type")]
    #[builder(default = "MacOSPackageRef".into())]
    #[cfg_attr(feature = "poem-openapi", oai(rename = "type"))]
    #[cfg_attr(feature = "async-graphql", graphql(name = "type"))]
    _type: String,

    pub pkg_id: String,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub min_build: Option<String>,
    pub max_build: Option<String>,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TypedBuilder,
)]
#[cfg_attr(feature = "poem-openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "async-graphql", derive(async_graphql::SimpleObject))]
pub struct PathRef {
    #[serde(rename = "type")]
    #[builder(default = "MacOSPathRef".into())]
    #[cfg_attr(feature = "poem-openapi", oai(rename = "type"))]
    #[cfg_attr(feature = "async-graphql", graphql(name = "type"))]
    _type: String,

    pub app_paths: Vec<String>,
    pub min_version: Option<String>,
    pub max_version: Option<String>,
    pub min_build: Option<String>,
    pub max_build: Option<String>,
}
