
use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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
    pub name: String,
    pub version: String,
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