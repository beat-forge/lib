use std::marker::PhantomData;

use bincode::serialize;
use bytes::Bytes;
use xz2::{read::XzDecoder, write::XzEncoder};

// use super::manifest::ForgeManifest;
use serde::{Deserialize, Serialize};

use super::manifest::{ManifestComponent, ManifestVersion, ForgeManifestSafe};

/// Outer wrapper for forge mods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeMod<Version: ManifestVersion, Comp: ManifestComponent, Inner: ForgeModData> {
    pub(crate) manifest: ForgeManifestSafe<Comp, Version>,

    pub(crate) _inner: Inner,

    #[serde(skip)]
    pub(crate) _marker: PhantomData<Version>,
}

/// Marker trait for forge mod data.
pub trait ForgeModData {}

impl<
        Version: ManifestVersion + Serialize + for<'a> Deserialize<'a>,
        Comp: ManifestComponent + Serialize + for<'a> Deserialize<'a>,
        Inner: ForgeModData + Serialize + for<'a> Deserialize<'a>,
    > ForgeMod<Version, Comp, Inner>
{
    pub fn pack(&self) -> Result<Bytes, std::io::Error> {
        let buf = serialize(&self).unwrap();
        let buf = XzEncoder::new(buf, 9).finish()?;

        Ok(Bytes::from(buf))
    }

    pub fn from_bytes<'a, T: Into<&'a [u8]>>(bytes: T) -> Result<Self, bincode::Error> {
        let contents = XzDecoder::new(bytes.into()).into_inner();
        bincode::deserialize(contents)
    }
}
