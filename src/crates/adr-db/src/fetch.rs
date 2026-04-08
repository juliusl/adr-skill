//! Fetch subcommand — fetch a work item from a remote and output normalized JSONL.

use adr_db_lib::remote::gitea::GiteaAdapter;
use adr_db_lib::remote::RemoteAdapter;

pub fn run_fetch(remote: &str, id: &str) -> Result<(), String> {
    match remote {
        "gitea" => {
            let adapter = build_gitea_adapter()?;
            let work_item = adapter.fetch_issue(id)?;
            let json = serde_json::to_string(&work_item)
                .map_err(|e| format!("JSON serialization error: {}", e))?;
            println!("{}", json);
            Ok(())
        }
        other => Err(format!(
            "Unknown remote '{}'. Supported: gitea",
            other
        )),
    }
}

fn build_gitea_adapter() -> Result<GiteaAdapter, String> {
    // Try reading from .adr/preferences.toml first
    // Fall back to environment variables
    let url = std::env::var("ADR_GITEA_URL")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    let owner = std::env::var("ADR_GITEA_OWNER")
        .map_err(|_| "ADR_GITEA_OWNER not set. Set it or configure [remote.gitea] in .adr/preferences.toml".to_string())?;
    let repo = std::env::var("ADR_GITEA_REPO")
        .map_err(|_| "ADR_GITEA_REPO not set. Set it or configure [remote.gitea] in .adr/preferences.toml".to_string())?;
    let token = std::env::var("ADR_GITEA_TOKEN").ok();

    Ok(GiteaAdapter::new(&url, &owner, &repo, token))
}
