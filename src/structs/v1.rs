#![allow(dead_code)]

use std::{
    marker::PhantomData,
    path::PathBuf,
    str::FromStr, fmt::Display,
    fmt::Formatter
};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::build_manifest_builder;

use super::{
    forgemod::{ForgeMod, ForgeModData, ForgeModGeneric},
    manifest::*,
};

/* -------------------------------------------------------------------------- */
/*                                  Manifest                                  */
/* -------------------------------------------------------------------------- */

/// Version 1 of the forge manifest.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestV1;

/// Marker impl
impl ManifestVersion for ManifestV1 {}

// Convenience type
type ForgeManifestV1<T> = ForgeManifest<T, ManifestV1>;

/// V1 inner components
mod manifest {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
    pub struct Dependency {
        pub(super) name: String,
        pub(super) version: VersionReq,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
    pub struct Include {
        pub(super) bs_dest: PathBuf,
        pub(super) local_src: PathBuf,
    }

    /// type: mod
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Mod {
        pub(super) name: String,
        pub(super) description: String,
        pub(super) website: String,
        pub(super) version: Version,
        pub(super) game_version: VersionReq,
        pub(super) category: String,

        pub(super) artifact: Option<PathBuf>, // not actually optional
        pub(super) includes: Vec<Include>,

        pub(super) pre_exec: Option<PathBuf>,
        pub(super) post_exec: Option<PathBuf>,

        pub(super) depends: Vec<Dependency>,
        pub(super) conflicts: Vec<Dependency>,
    }

    /// type: module_parent
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Parent {
        pub(super) name: String,
        pub(super) description: String,
        pub(super) website: String,
        pub(super) version: Version,
        pub(super) game_version: VersionReq,
        pub(super) category: String,

        pub(super) pre_exec: Option<PathBuf>,
        pub(super) post_exec: Option<PathBuf>,

        pub(super) depends: Vec<Dependency>,
        pub(super) conflicts: Vec<Dependency>,

        pub(super) modules: Vec<PathBuf>,
    }

    /// type: module
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
    pub struct Module {
        pub(super) name: String,

        pub(super) required: bool,
        pub(super) suggested: bool,

        pub(super) artifact: Option<PathBuf>, // not actually optional
        pub(super) includes: Vec<Include>,

        pub(super) pre_exec: Option<PathBuf>,
        pub(super) post_exec: Option<PathBuf>,

        pub(super) depends: Vec<Dependency>,
        pub(super) conflicts: Vec<Dependency>,
    }

    /// type: lib
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Lib {
        pub(super) name: String,
        pub(super) description: String,
        pub(super) website: String,
        pub(super) version: Version,
        pub(super) game_version: VersionReq,
        pub(super) category: String,

        pub(super) artifact: Option<PathBuf>, // not actually optional
        pub(super) includes: Vec<Include>,

        pub(super) pre_exec: Option<PathBuf>,
        pub(super) post_exec: Option<PathBuf>,

        pub(super) depends: Vec<Dependency>,
        pub(super) conflicts: Vec<Dependency>,
    }

    impl ManifestComponent for Mod {}
    impl ManifestComponent for Parent {}
    impl ManifestComponent for Module {}
    impl ManifestComponent for Lib {}

    impl Default for Mod {
        fn default() -> Self {
            Self {
                name: String::new(),
                description: String::new(),
                website: String::new(),
                version: Version::new(0, 0, 0),
                game_version: VersionReq::from_str("*").unwrap(),
                category: String::new(),
                artifact: None,
                includes: Vec::new(),
                pre_exec: None,
                post_exec: None,
                depends: Vec::new(),
                conflicts: Vec::new(),
            }
        }
    }

    impl Default for Parent {
        fn default() -> Self {
            Self {
                name: String::new(),
                description: String::new(),
                website: String::new(),
                version: Version::new(0, 0, 0),
                game_version: VersionReq::from_str("*").unwrap(),
                category: String::new(),
                pre_exec: None,
                post_exec: None,
                depends: Vec::new(),
                conflicts: Vec::new(),
                modules: Vec::new(),
            }
        }
    }

    impl Default for Lib {
        fn default() -> Self {
            Self {
                name: String::new(),
                description: String::new(),
                website: String::new(),
                version: Version::new(0, 0, 0),
                game_version: VersionReq::from_str("*").unwrap(),
                category: String::new(),
                artifact: None,
                includes: Vec::new(),
                pre_exec: None,
                post_exec: None,
                depends: Vec::new(),
                conflicts: Vec::new(),
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum ManifestTypes {
    Mod,
    ModuleParent,
    Module,
    Lib,
}

impl ToString for ManifestTypes {
    fn to_string(&self) -> String {
        match self {
            Self::Mod => "mod".to_string(),
            Self::ModuleParent => "module_parent".to_string(),
            Self::Module => "module".to_string(),
            Self::Lib => "lib".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManifestBuilder<T: ManifestComponent> {
    kind: ManifestTypes,
    _inner: T,
}

impl ManifestBuilder<manifest::Mod> {
    pub fn new_mod(
        name: String,
        mod_version: Version,
        game_version: VersionReq,
        artifact: PathBuf,
    ) -> Self {
        Self {
            kind: ManifestTypes::Mod,
            _inner: manifest::Mod {
                name,
                version: mod_version,
                artifact: Some(artifact),
                game_version,
                ..Default::default()
            },
        }
    }

    build_manifest_builder!(description, String);
    build_manifest_builder!(website, String);
    build_manifest_builder!(category, String);
    build_manifest_builder!(pre_exec, Option<PathBuf>);
    build_manifest_builder!(post_exec, Option<PathBuf>);
    build_manifest_builder!(includes, Vec<manifest::Include>);
    build_manifest_builder!(depends, Vec<manifest::Dependency>);
    build_manifest_builder!(conflicts, Vec<manifest::Dependency>);

    pub fn build(self) -> ForgeManifestV1<manifest::Mod> {
        ForgeManifestV1 {
            _id: slug::slugify(&self._inner.name),
            manifest_version: 1,
            _type: self.kind.to_string(),
            inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ManifestBuilder<manifest::Lib> {
    pub fn new_lib(
        name: String,
        lib_version: Version,
        game_version: VersionReq,
        artifact: PathBuf,
    ) -> Self {
        Self {
            kind: ManifestTypes::Lib,
            _inner: manifest::Lib {
                name,
                version: lib_version,
                artifact: Some(artifact),
                game_version,
                ..Default::default()
            },
        }
    }

    build_manifest_builder!(description, String);
    build_manifest_builder!(website, String);
    build_manifest_builder!(category, String);
    build_manifest_builder!(pre_exec, Option<PathBuf>);
    build_manifest_builder!(post_exec, Option<PathBuf>);
    build_manifest_builder!(includes, Vec<manifest::Include>);
    build_manifest_builder!(depends, Vec<manifest::Dependency>);
    build_manifest_builder!(conflicts, Vec<manifest::Dependency>);

    pub fn build(self) -> ForgeManifestV1<manifest::Lib> {
        ForgeManifestV1 {
            _id: slug::slugify(&self._inner.name),
            manifest_version: 1,
            _type: self.kind.to_string(),
            inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ManifestBuilder<manifest::Parent> {
    pub fn new_module_parent(name: String, parent_version: Version, game_version: VersionReq) -> Self {
        Self {
            kind: ManifestTypes::ModuleParent,
            _inner: manifest::Parent {
                name,
                version: parent_version,
                game_version,
                ..Default::default()
            },
        }
    }

    // build_manifest_builder!(name, String);
    build_manifest_builder!(description, String);
    build_manifest_builder!(website, String);
    build_manifest_builder!(category, String);
    build_manifest_builder!(pre_exec, Option<PathBuf>);
    build_manifest_builder!(post_exec, Option<PathBuf>);
    build_manifest_builder!(modules, Vec<PathBuf>);
    build_manifest_builder!(depends, Vec<manifest::Dependency>);
    build_manifest_builder!(conflicts, Vec<manifest::Dependency>);

    pub fn build(self) -> ForgeManifestV1<manifest::Parent> {
        ForgeManifestV1 {
            _id: slug::slugify(&self._inner.name),
            manifest_version: 1,
            _type: self.kind.to_string(),
            inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ManifestBuilder<manifest::Module> {
    pub fn new_module(module_name: String, artifact: PathBuf) -> Self {
        Self {
            kind: ManifestTypes::Module,
            _inner: manifest::Module {
                name: module_name,
                artifact: Some(artifact),
                ..Default::default()
            },
        }
    }

    build_manifest_builder!(required, bool);
    build_manifest_builder!(suggested, bool);
    build_manifest_builder!(pre_exec, Option<PathBuf>);
    build_manifest_builder!(post_exec, Option<PathBuf>);
    build_manifest_builder!(includes, Vec<manifest::Include>);
    build_manifest_builder!(depends, Vec<manifest::Dependency>);
    build_manifest_builder!(conflicts, Vec<manifest::Dependency>);

    pub fn build(self) -> ForgeManifestV1<manifest::Module> {
        ForgeManifestV1 {
            _id: slug::slugify(&self._inner.name),
            manifest_version: 1,
            _type: self.kind.to_string(),
            inner: self._inner,
            _marker: PhantomData,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncludeBuilder {
    pub(self) _inners: Vec<manifest::Include>,
}

impl IncludeBuilder {
    pub fn new() -> Self {
        Self { _inners: vec![] }
    }

    pub fn add(&mut self, bs_dest: PathBuf, local_src: PathBuf) -> &mut Self {
        self._inners.push(manifest::Include { bs_dest, local_src });

        self
    }

    pub fn build(self) -> Vec<manifest::Include> {
        self._inners
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DependencyBuilder {
    pub(self) _inners: Vec<manifest::Dependency>,
}

impl DependencyBuilder {
    pub fn new() -> Self {
        Self { _inners: vec![] }
    }

    pub fn add(&mut self, name: String, version: VersionReq) -> &mut Self {
        self._inners.push(manifest::Dependency { name, version });

        self
    }

    pub fn build(self) -> Vec<manifest::Dependency> {
        self._inners
    }
}

/* -------------------------------------------------------------------------- */
/*                             Data Storage Format                            */
/* -------------------------------------------------------------------------- */

type ForgeModV1<Comp, Data> = ForgeMod<ManifestV1, Comp, Data>;

mod data {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct IncludeData {
        pub(super) dest: String,
        #[serde(with = "serde_bytes")]
        pub(super) data: Vec<u8>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Mod {
        #[serde(with = "serde_bytes")]
        pub(super) artifact_data: Vec<u8>,
        pub(super) includes_data: Vec<IncludeData>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Parent {}

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Module {
        pub(super) _id: String,
        pub(super) required: bool,
        pub(super) suggested: bool,
        #[serde(with = "serde_bytes")]
        pub(super) artifact_data: Vec<u8>,
        pub(super) includes_data: Vec<IncludeData>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Lib {
        #[serde(with = "serde_bytes")]
        pub(super) artifact_data: Vec<u8>,
        pub(super) includes_data: Vec<IncludeData>,
    }

    impl ForgeModData for Mod {}
    impl ForgeModData for Parent {}
    impl ForgeModData for Module {}
    impl ForgeModData for Lib {}
}

/// It is intended that the manifest is build first, then the data is added.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModBuilder<Type: ManifestComponent, Data: ForgeModData> {
    pub(self) _manifest: ForgeManifestV1<Type>,
    pub(self) _inner: Data,
}

impl ModBuilder<manifest::Mod, data::Mod> {
    pub fn new_mod_raw(manifest: ForgeManifestV1<manifest::Mod>, artifact_data: Vec<u8>) -> Self {
        Self {
            _manifest: manifest,
            _inner: data::Mod {
                artifact_data,
                includes_data: vec![],
            },
        }
    }

    pub fn new_mod(
        manifest: ForgeManifestV1<manifest::Mod>,
        artifact_path: PathBuf,
    ) -> Result<Self, std::io::Error> {
        let artifact_data = std::fs::read(artifact_path)?;

        Ok(Self {
            _manifest: manifest,
            _inner: data::Mod {
                artifact_data,
                includes_data: vec![],
            },
        })
    }

    pub fn includes(&mut self, includes: Vec<data::IncludeData>) -> &mut Self {
        self._inner.includes_data = includes;
        self
    }

    pub fn build(self) -> ForgeModV1<manifest::Mod, data::Mod> {
        ForgeMod {
            format_version: 1,
            kind: "mod".into(),
            manifest: self._manifest.into(),
            _inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ForgeMod<ManifestV1, manifest::Mod, data::Mod> {
    pub fn artifact_data(&self) -> &[u8] {
        &self._inner.artifact_data
    }

    pub fn includes_data(&self) -> &[data::IncludeData] {
        &self._inner.includes_data
    }
}

impl ModBuilder<manifest::Parent, data::Parent> {
    pub fn new_module_parent(manifest: ForgeManifestV1<manifest::Parent>) -> Self {
        Self {
            _manifest: manifest,
            _inner: data::Parent {},
        }
    }

    pub fn build(self) -> ForgeModV1<manifest::Parent, data::Parent> {
        ForgeMod {
            format_version: 1,
            kind: "parent".into(),
            manifest: self._manifest.into(),
            _inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ModBuilder<manifest::Module, data::Module> {
    pub fn new_module_raw(
        manifest: ForgeManifestV1<manifest::Module>,
        artifact_data: Vec<u8>,
    ) -> Self {
        Self {
            _inner: data::Module {
                _id: manifest._id.clone(),
                required: manifest.inner.required,
                suggested: manifest.inner.suggested,
                artifact_data,
                includes_data: vec![],
            },
            _manifest: manifest,
        }
    }

    pub fn new_module(manifest: ForgeManifestV1<manifest::Module>) -> Result<Self, std::io::Error> {
        // safety: artifact is not optional
        let artifact_data = std::fs::read(manifest.inner.artifact.as_ref().unwrap())?;

        Ok(Self {
            _inner: data::Module {
                _id: manifest._id.clone(),
                required: manifest.inner.required,
                suggested: manifest.inner.suggested,
                artifact_data,
                includes_data: vec![],
            },
            _manifest: manifest,
        })
    }

    pub fn includes(&mut self, includes: Vec<data::IncludeData>) -> &mut Self {
        self._inner.includes_data = includes;
        self
    }

    pub fn build(self) -> ForgeModV1<manifest::Module, data::Module> {
        ForgeMod {
            format_version: 1,
            kind: "module".into(),
            manifest: self._manifest.into(),
            _inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ForgeMod<ManifestV1, manifest::Module, data::Module> {
    pub fn artifact_data(&self) -> &[u8] {
        &self._inner.artifact_data
    }

    pub fn includes_data(&self) -> &[data::IncludeData] {
        &self._inner.includes_data
    }
}

impl ModBuilder<manifest::Lib, data::Lib> {
    pub fn new_lib_raw(manifest: ForgeManifestV1<manifest::Lib>, artifact_data: Vec<u8>) -> Self {
        Self {
            _inner: data::Lib {
                artifact_data,
                includes_data: vec![],
            },
            _manifest: manifest,
        }
    }

    pub fn new_lib(manifest: ForgeManifestV1<manifest::Lib>) -> Result<Self, std::io::Error> {
        // safety: artifact is not optional
        let artifact_data = std::fs::read(manifest.inner.artifact.as_ref().unwrap())?;

        Ok(Self {
            _inner: data::Lib {
                artifact_data,
                includes_data: vec![],
            },
            _manifest: manifest,
        })
    }

    pub fn includes(&mut self, includes: Vec<data::IncludeData>) -> &mut Self {
        self._inner.includes_data = includes;
        self
    }

    pub fn build(self) -> ForgeModV1<manifest::Lib, data::Lib> {
        ForgeMod {
            format_version: 1,
            kind: "lib".into(),
            manifest: self._manifest.into(),
            _inner: self._inner,
            _marker: PhantomData,
        }
    }
}

impl ForgeMod<ManifestV1, manifest::Lib, data::Lib> {
    pub fn artifact_data(&self) -> &[u8] {
        &self._inner.artifact_data
    }

    pub fn includes_data(&self) -> &[data::IncludeData] {
        &self._inner.includes_data
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IncludeDataBuilder {
    pub(self) _inners: Vec<data::IncludeData>,
}

impl IncludeDataBuilder {
    pub fn new() -> Self {
        Self { _inners: vec![] }
    }

    pub fn add_raw(&mut self, dest: String, data: Vec<u8>) -> &mut Self {
        self._inners.push(data::IncludeData { dest, data });

        self
    }

    pub fn add(&mut self, dest: String, src: PathBuf) -> Result<&mut Self, std::io::Error> {
        let data = std::fs::read(src)?;

        self._inners.push(data::IncludeData { dest, data });

        Ok(self)
    }

    pub fn build(self) -> Vec<data::IncludeData> {
        self._inners
    }
}

pub enum ForgeModTypes {
    Mod(ForgeModV1<manifest::Mod, data::Mod>),
    Parent(ForgeModV1<manifest::Parent, data::Parent>),
    Module(ForgeModV1<manifest::Module, data::Module>),
    Lib(ForgeModV1<manifest::Lib, data::Lib>),
}

impl Display for ForgeModTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ForgeModTypes::Mod(_) => write!(f, "mod"),
            ForgeModTypes::Parent(_) => write!(f, "parent"),
            ForgeModTypes::Module(_) => write!(f, "module"),
            ForgeModTypes::Lib(_) => write!(f, "lib"),
        }
    }
}

/// THIS IS THE FUNCTION THAT YOU WANT TO USE
/// DONT USE ANYTHING ELSE!!!!
/// I promise i will make it better!
pub fn unpack_v1_forgemod<'a, T: Into<&'a [u8]>>(data: T) -> Result<ForgeModTypes, Box<dyn std::error::Error>> {
    let data = data.into();
    let generic = ForgeModGeneric::from_bytes(data)?;
    let kind = generic.kind.as_str();
    let format_version = generic.format_version;

    if format_version != 1 {
        return Err("cannot find v1 manifest information.".into());
    }

    match kind {
        "mod" => {
            ForgeModV1::<manifest::Mod, data::Mod>::from_bytes(data)
                .map(|v| ForgeModTypes::Mod(v))
                .map_err(|e| e.into())
        },
        "parent" => {
            ForgeModV1::<manifest::Parent, data::Parent>::from_bytes(data)
                .map(|v| ForgeModTypes::Parent(v))
                .map_err(|e| e.into())
        },
        "module" => {
            ForgeModV1::<manifest::Module, data::Module>::from_bytes(data)
                .map(|v| ForgeModTypes::Module(v))
                .map_err(|e| e.into())
        },
        "lib" => {
            ForgeModV1::<manifest::Lib, data::Lib>::from_bytes(data)
                .map(|v| ForgeModTypes::Lib(v))
                .map_err(|e| e.into())
        },
        _ => Err("unknown kind".into()),
    }
}