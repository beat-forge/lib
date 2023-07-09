use bincode::serialize;
use bytes::Bytes;
use std::convert::TryFrom;
use xz2::{read::XzDecoder, write::XzEncoder};

use super::manifest::ForgeManifest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeMod {
    pub manifest: ForgeManifest,
    #[serde(with = "serde_bytes")]
    pub artifact_data: Vec<u8>,
    pub includes_data: Vec<IncludeData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct IncludeData {
    pub dest: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
}

impl ForgeMod {
    pub fn pack(&self) -> Result<Bytes, std::io::Error> {
        let buf = serialize(&self).unwrap();
        let buf = XzEncoder::new(buf, 9).finish()?;

        Ok(Bytes::from(buf))
    }
}

impl<'a> TryFrom<&'a [u8]> for ForgeMod {
    type Error = bincode::Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let contents = XzDecoder::new(bytes).into_inner();
        bincode::deserialize(contents)
    }
}
