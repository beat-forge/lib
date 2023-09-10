use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// Outer wrapper for forge manifests.
/// Generic over the inner manifest type and the version of the manifest.
/// Builders should only be able to be generic over the version of the manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeManifest<Inner: ManifestComponent, Version: ManifestVersion> {
    pub _id: String,
    pub manifest_version: u32,
    #[serde(rename = "type")]
    pub _type: String,

    #[serde(flatten)]
    pub inner: Inner,

    #[serde(skip)]
    pub(crate) _marker: PhantomData<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeManifestSafe<Inner: ManifestComponent, Version: ManifestVersion> {
    pub _id: String,
    pub manifest_version: u32,
    #[serde(rename = "type")]
    pub _type: String,

    // #[serde(flatten)] does not work with bincode
    pub inner: Inner,

    #[serde(skip)]
    pub(crate) _marker: PhantomData<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeManifestGeneric {
    pub _id: String,
    pub manifest_version: u32,
    #[serde(rename = "type")]
    pub _type: String,
}

impl ForgeManifestGeneric {
    pub fn from_bytes<'a, T: Into<&'a [u8]>>(bytes: T) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_slice(bytes.into())?)
    }
}

impl<Inner: ManifestComponent, Version: ManifestVersion> From<ForgeManifest<Inner, Version>> for ForgeManifestSafe<Inner, Version> {
    fn from(manifest: ForgeManifest<Inner, Version>) -> Self {
        Self {
            _id: manifest._id,
            manifest_version: manifest.manifest_version,
            _type: manifest._type,
            inner: manifest.inner,
            _marker: PhantomData,
        }
    }
}

/// Marker trait for forge manifest components.
pub trait ManifestComponent {}

/// Marker trait for forge manifest versions.
pub trait ManifestVersion {}

#[macro_export]
macro_rules! build_manifest_builder {
    ($name:ident, $inner:ty) => {
        pub fn $name(&mut self, $name: $inner) -> &mut Self {
            self._inner.$name = $name;
            self
        }
    };
}
