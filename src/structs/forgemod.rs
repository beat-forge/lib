use bincode::serialize;
use bytes::Bytes;
use std::convert::TryFrom;
use xz2::{read::XzDecoder, write::XzEncoder};

use super::manifest::ForgeManifest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

pub struct ForgeModBytes<'a>(pub &'a [u8]);

impl<'a, T: Into<&'a [u8]>> From<T> for ForgeModBytes<'a> {
    fn from(data: T) -> Self {
        Self(data.into())
    }
}

impl<'a> TryFrom<ForgeModBytes<'a>> for ForgeMod {
    type Error = bincode::Error;

    fn try_from(bytes: ForgeModBytes<'a>) -> Result<Self, Self::Error> {
        let contents = XzDecoder::new(bytes.0).into_inner();
        bincode::deserialize(contents)
    }
}
