use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

/// Outer wrapper for forge manifests.
/// Generic over the inner manifest type and the version of the manifest.
/// Builders should only be able to be generic over the version of the manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForgeManifest<Inner: ManifestComponent, Version: ManifestVersion> {
    pub(crate) _id: String,
    pub(crate) manifest_version: u32,
    #[serde(rename = "type")]
    pub(crate) _type: String,

    #[serde(flatten)]
    pub(crate) inner: Inner,

    #[serde(skip)]
    pub(crate) _marker: PhantomData<Version>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ForgeManifestSafe<Inner: ManifestComponent, Version: ManifestVersion> {
    pub(crate) _id: String,
    pub(crate) manifest_version: u32,
    #[serde(rename = "type")]
    pub(crate) _type: String,

    // #[serde(flatten)] does not work with bincode
    pub(crate) inner: Inner,

    #[serde(skip)]
    pub(crate) _marker: PhantomData<Version>,
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
