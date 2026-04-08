use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::error::TmfError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSettings {
    pub host: String,
    pub port: u16,
    pub cors_origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmfApiEntry {
    pub name: String,
    pub domain: String,
    pub base_path: String,
    pub versions: Vec<String>,
    pub version: String,
    pub resources: Vec<String>,
    pub base_url: String,
}

impl TmfApiEntry {
    pub fn is_configured(&self) -> bool {
        !self.base_url.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeqSettings {
    pub url: String,
    #[serde(default)]
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmfConfigFile {
    pub portal: PortalSettings,
    #[serde(default)]
    pub seq: Option<SeqSettings>,
    pub apis: HashMap<String, TmfApiEntry>,
}

impl TmfConfigFile {
    pub fn from_file(path: &Path) -> Result<Self, TmfError> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            TmfError::Config(format!("Failed to read config file {}: {}", path.display(), e))
        })?;
        serde_json::from_str(&content).map_err(|e| {
            TmfError::Config(format!("Failed to parse config file: {}", e))
        })
    }

    pub fn configured_apis(&self) -> HashMap<String, &TmfApiEntry> {
        self.apis
            .iter()
            .filter(|(_, entry)| entry.is_configured())
            .map(|(id, entry)| (id.clone(), entry))
            .collect()
    }
}
