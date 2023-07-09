
use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};
use semver::{Version, VersionReq};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgeManifest {
    pub manifest_version: u32,
    
    /// The mod's unique identifier. A slug. Not to be confused with the mod's ObjectId.
    pub _id: String,
    pub name: String,
    pub description: String,
    pub website: String,
    pub version: Version,
    pub game_version: VersionReq,
    pub artifact: String,

    pub pre_exec: String,
    pub post_exec: String,
    pub includes: Vec<Include>,

    pub depends: Vec<Depends>,   // slug, version
    pub conflicts: Vec<Depends>, // slug, version
    pub category: ModCategory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Include {
    pub target: String,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Depends {
    pub id: String,
    pub version: VersionReq,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ModCategory {
    Core,
    Libraries,
    Cosmetic,
    Gameplay,
    Leaderboards,
    Lighting,
    Multiplayer,
    Accessibility,
    Practice,
    Streaming,
    Text,
    Tweaks,
    UI,

    #[default]
    Other,
}

impl TryFrom<PathBuf> for ForgeManifest {
    type Error = std::io::Error;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let manifest = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&manifest)?)
    }
}

impl ToString for ModCategory {
    fn to_string(&self) -> String {
        match self {
            ModCategory::Core => "core".to_string(),
            ModCategory::Libraries => "libraries".to_string(),
            ModCategory::Cosmetic => "cosmetic".to_string(),
            ModCategory::Gameplay => "gameplay".to_string(),
            ModCategory::Leaderboards => "leaderboards".to_string(),
            ModCategory::Lighting => "lighting".to_string(),
            ModCategory::Multiplayer => "multiplayer".to_string(),
            ModCategory::Accessibility => "accessibility".to_string(),
            ModCategory::Practice => "practice".to_string(),
            ModCategory::Streaming => "streaming".to_string(),
            ModCategory::Text => "text".to_string(),
            ModCategory::Tweaks => "tweaks".to_string(),
            ModCategory::UI => "ui".to_string(),
            ModCategory::Other => "other".to_string(),
        }
    }
}

impl FromStr for ModCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "core" => Ok(ModCategory::Core),
            "libraries" => Ok(ModCategory::Libraries),
            "cosmetic" => Ok(ModCategory::Cosmetic),
            "gameplay" => Ok(ModCategory::Gameplay),
            "leaderboards" => Ok(ModCategory::Leaderboards),
            "lighting" => Ok(ModCategory::Lighting),
            "multiplayer" => Ok(ModCategory::Multiplayer),
            "accessibility" => Ok(ModCategory::Accessibility),
            "practice" => Ok(ModCategory::Practice),
            "streaming" => Ok(ModCategory::Streaming),
            "text" => Ok(ModCategory::Text),
            "tweaks" => Ok(ModCategory::Tweaks),
            "ui" => Ok(ModCategory::UI),
            _ => Ok(ModCategory::Other),
        }
    }
}