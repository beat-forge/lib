use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgeManifest {
    pub name: String,
    pub description: String,
    pub website: String,
    pub author: String,
    pub version: String,
    pub game_version: String,
    pub artifact: String,
    pub pre_exec: String,
    pub post_exec: String,
    pub includes: Vec<Include>,
    pub depends: Vec<Depends>,   // slug, version
    pub conflicts: Vec<Depends>, // slug, version
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Include {
    pub target: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depends {
    pub name: String,
    pub version: String,
}
