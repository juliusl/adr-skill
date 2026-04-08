//! Remote adapters for normalized work item fetching (ADR-0035, ADR-0045).
//!
//! Each adapter fetches work items from a remote system and normalizes them
//! to the `NormalizedWorkItem` schema defined by ADR-0035.

pub mod gitea;

use serde::{Deserialize, Serialize};

/// Normalized work item matching ADR-0035's schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedWorkItem {
    pub remote: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub state: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub created: String,
    #[serde(default)]
    pub updated: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_at: Option<String>,
}

/// Trait for remote adapters.
pub trait RemoteAdapter {
    /// Fetch a single work item by ID and return it normalized.
    fn fetch_issue(&self, id: &str) -> Result<NormalizedWorkItem, String>;
}
