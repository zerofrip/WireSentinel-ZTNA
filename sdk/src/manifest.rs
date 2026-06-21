use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Manifest describing a ZTNA provider plugin.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZtnaPluginManifest {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub provider: String,
    pub description: Option<String>,
    pub min_trust_score: u8,
}

impl ZtnaPluginManifest {
    pub fn new(name: impl Into<String>, version: impl Into<String>, provider: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version: version.into(),
            provider: provider.into(),
            description: None,
            min_trust_score: 50,
        }
    }
}
