//! Gitea remote adapter (ADR-0045).
//!
//! Fetches issues from a Gitea instance and normalizes them to the ADR-0035 schema.
//! Uses `ureq` (blocking HTTP client) — no async runtime needed.

use super::{NormalizedWorkItem, RemoteAdapter};
use serde::Deserialize;

/// Gitea adapter configuration.
pub struct GiteaAdapter {
    pub base_url: String,
    pub owner: String,
    pub repo: String,
    pub token: Option<String>,
}

/// Gitea issue response (subset of fields we need).
#[derive(Debug, Deserialize)]
struct GiteaIssue {
    number: u64,
    title: String,
    #[serde(default)]
    body: Option<String>,
    state: String,
    #[serde(default)]
    html_url: Option<String>,
    #[serde(default)]
    labels: Vec<GiteaLabel>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GiteaLabel {
    name: String,
}

impl GiteaAdapter {
    pub fn new(base_url: &str, owner: &str, repo: &str, token: Option<String>) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
            token,
        }
    }

    /// Normalize a Gitea issue into the ADR-0035 model.
    fn normalize(issue: GiteaIssue) -> NormalizedWorkItem {
        let label_names: Vec<String> = issue.labels.iter().map(|l| l.name.clone()).collect();

        let item_type = if label_names.iter().any(|l| l == "bug") {
            "bug".to_string()
        } else {
            "issue".to_string()
        };

        let state = match issue.state.as_str() {
            "closed" => "closed".to_string(),
            _ => "open".to_string(),
        };

        let description = issue
            .body
            .unwrap_or_default()
            .chars()
            .take(500)
            .collect::<String>();

        NormalizedWorkItem {
            remote: "gitea".to_string(),
            id: issue.number.to_string(),
            title: issue.title,
            item_type,
            state,
            url: issue.html_url.unwrap_or_default(),
            description,
            labels: label_names,
            created: issue.created_at.unwrap_or_default(),
            updated: issue.updated_at.unwrap_or_default(),
            cached_at: None,
        }
    }
}

impl RemoteAdapter for GiteaAdapter {
    fn fetch_issue(&self, id: &str) -> Result<NormalizedWorkItem, String> {
        let url = format!(
            "{}/api/v1/repos/{}/{}/issues/{}",
            self.base_url, self.owner, self.repo, id
        );

        let mut request = ureq::get(&url);

        if let Some(ref token) = self.token {
            if !token.is_empty() {
                request = request.set("Authorization", &format!("token {}", token));
            }
        }

        let response = request.call().map_err(|e| format!("HTTP error: {}", e))?;

        let issue: GiteaIssue = response
            .into_json()
            .map_err(|e| format!("JSON parse error: {}", e))?;

        Ok(Self::normalize(issue))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_basic_issue() {
        let issue = GiteaIssue {
            number: 42,
            title: "Evaluate PostgreSQL".to_string(),
            body: Some("Description here".to_string()),
            state: "open".to_string(),
            html_url: Some("http://localhost:3000/user/repo/issues/42".to_string()),
            labels: vec![GiteaLabel {
                name: "adr".to_string(),
            }],
            created_at: Some("2026-04-01T10:00:00Z".to_string()),
            updated_at: Some("2026-04-05T07:00:00Z".to_string()),
        };

        let normalized = GiteaAdapter::normalize(issue);
        assert_eq!(normalized.remote, "gitea");
        assert_eq!(normalized.id, "42");
        assert_eq!(normalized.title, "Evaluate PostgreSQL");
        assert_eq!(normalized.item_type, "issue");
        assert_eq!(normalized.state, "open");
        assert_eq!(normalized.labels, vec!["adr"]);
    }

    #[test]
    fn test_normalize_bug_label() {
        let issue = GiteaIssue {
            number: 7,
            title: "Fix crash".to_string(),
            body: Some("Crash on startup".to_string()),
            state: "closed".to_string(),
            html_url: None,
            labels: vec![
                GiteaLabel {
                    name: "bug".to_string(),
                },
                GiteaLabel {
                    name: "urgent".to_string(),
                },
            ],
            created_at: None,
            updated_at: None,
        };

        let normalized = GiteaAdapter::normalize(issue);
        assert_eq!(normalized.item_type, "bug");
        assert_eq!(normalized.state, "closed");
    }

    #[test]
    fn test_normalize_description_truncation() {
        let long_body = "x".repeat(600);
        let issue = GiteaIssue {
            number: 1,
            title: "Test".to_string(),
            body: Some(long_body),
            state: "open".to_string(),
            html_url: None,
            labels: vec![],
            created_at: None,
            updated_at: None,
        };

        let normalized = GiteaAdapter::normalize(issue);
        assert_eq!(normalized.description.len(), 500);
    }
}
