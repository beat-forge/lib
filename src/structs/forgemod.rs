use super::manifest::ForgeManifest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeMod {
    pub manifest: ForgeManifest,
    #[serde(with = "serde_bytes")]
    pub artifact_data: Vec<u8>,
    pub includes_data: Vec<IncludeData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncludeData {
    pub dest: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}
