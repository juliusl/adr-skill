use std::fs;
use std::path::Path;

use clap::Subcommand;

use adr_db_lib::{generate_template, parse_adr, serialize_adr};
use adr_db_lib::format::schema::{AdrOption, CheckpointItem, Consequence, Decision};

/// ADR authoring subcommand definitions.
#[derive(Subcommand)]
pub enum AuthorCommands {
    /// Create a new ADR with work-item-referenced naming
    New {
        /// Git remote name (e.g., origin) or 'local' for offline use
        remote: String,
        /// Work item ID (alphanumeric)
        id: String,
        /// ADR title
        title: String,
        /// Target directory
        dir: String,
    },
    /// Bootstrap ADR directory with initial record
    Init {
        /// Target directory (default: docs/adr)
        dir: Option<String>,
    },
    /// List ADRs with title and status
    List,
    /// Rename an ADR file and update heading
    Rename {
        /// Git remote name (e.g., origin) or 'local' for offline use
        remote: String,
        /// Work item ID
        id: String,
        /// New title
        new_title: String,
    },
    /// Show or update ADR status
    Status {
        /// Git remote name (e.g., origin) or 'local' for offline use
        remote: Option<String>,
        /// Work item ID
        id: Option<String>,
        /// New status to set
        new_status: Option<String>,
    },
    /// Check or execute lifecycle transition
    Lifecycle {
        /// Git remote name (e.g., origin) or 'local' for offline use
        remote: String,
        /// Work item ID
        id: String,
        /// Execute the transition automatically
        #[arg(long)]
        auto: bool,
        /// Sync work item state
        #[arg(long)]
        sync: bool,
    },
    /// Export ADR as Markdown to stdout
    Export {
        /// Git remote name (e.g., origin) or 'local' for offline use
        remote: String,
        /// Work item ID
        id: String,
    },
}

/// Extract the host portion from a git URL.
///
/// Handles SSH (`git@host:path`), `ssh://` (`ssh://git@host/path`),
/// and HTTPS (`https://host/path`) formats.
fn extract_host(url: &str) -> String {
    // SSH shorthand: git@github.com:org/repo.git
    if let Some(rest) = url.strip_prefix("git@") {
        if let Some(host) = rest.split(':').next() {
            return host.to_lowercase();
        }
    }
    // Scheme-based: https://host/path or ssh://git@host/path
    if let Some(rest) = url.split("://").nth(1) {
        let after_userinfo = if let Some(pos) = rest.find('@') {
            &rest[pos + 1..]
        } else {
            rest
        };
        if let Some(host) = after_userinfo.split('/').next() {
            // Strip port if present
            return host.split(':').next().unwrap_or(host).to_lowercase();
        }
    }
    url.to_lowercase()
}

/// Detect the adapter type from a git remote URL.
fn detect_adapter_from_url(url: &str) -> Result<String, String> {
    let host = extract_host(url);
    if host.contains("github.com") {
        Ok("gh".to_string())
    } else if host.contains("dev.azure.com") || host.contains("visualstudio.com") {
        Ok("ado".to_string())
    } else {
        Err(format!(
            "Could not detect adapter type for URL '{}'. \
             Supported hosts: github.com, dev.azure.com. \
             For other forges, a future configuration mechanism \
             will allow explicit adapter mapping.",
            url
        ))
    }
}

/// Detect the adapter type from a git remote name or the `local` keyword.
///
/// If `remote` is `"local"`, returns `"local"` immediately.
/// Otherwise, runs `git remote get-url <remote>` and matches the URL
/// against known host patterns.
fn detect_adapter(remote: &str) -> Result<String, String> {
    if remote == "local" {
        return Ok("local".to_string());
    }

    let output = std::process::Command::new("git")
        .args(["remote", "get-url", remote])
        .output()
        .map_err(|e| format!("failed to run git: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "git remote '{}' not found. Run 'git remote -v' to list available remotes.",
            remote
        ));
    }

    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    detect_adapter_from_url(&url)
}

fn validate_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("id is required".to_string());
    }
    if !id.chars().all(|c| c.is_alphanumeric()) {
        return Err(format!("id must be alphanumeric, got '{}'", id));
    }
    Ok(())
}

fn validate_dir(dir: &str) -> Result<(), String> {
    if dir.contains("..") {
        return Err(format!("directory '{}' contains '..' — path traversal not allowed", dir));
    }
    Ok(())
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn resolve_dir() -> String {
    if let Ok(dir) = fs::read_to_string(".adr/adr-dir") {
        return dir.trim().to_string();
    }
    if let Ok(dir) = fs::read_to_string(".adr-dir") {
        return dir.trim().to_string();
    }
    "docs/adr".to_string()
}

fn today() -> String {
    std::env::var("ADR_DATE").unwrap_or_else(|_| {
        std::process::Command::new("date")
            .arg("+%Y-%m-%d")
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_else(|_| "1970-01-01".to_string())
    })
}

fn find_adr_file(dir: &str, remote: &str, id: &str) -> Result<String, String> {
    let prefix = format!("{}-{}-", remote, id);
    let dir_path = Path::new(dir);
    if !dir_path.is_dir() {
        return Err(format!("ADR directory not found: {}", dir));
    }
    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with(&prefix) && name.ends_with(".toml") {
            return Ok(entry.path().to_string_lossy().to_string());
        }
    }
    Err(format!("ADR {}-{} not found in {}", remote, id, dir))
}

fn cmd_new(remote: &str, id: &str, title: &str, dir: &str) -> Result<(), String> {
    let adapter = detect_adapter(remote)?;
    validate_id(id)?;
    validate_dir(dir)?;
    if title.is_empty() {
        return Err("title is required".to_string());
    }

    let slug = slugify(title);
    let file_path = format!("{}/{}-{}-{}.toml", dir, adapter, id, slug);

    let prefix = format!("{}-{}-", adapter, id);
    if Path::new(dir).is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && name.ends_with(".toml") {
                return Err(format!(
                    "ADR for {}-{} already exists: {}",
                    adapter,
                    id,
                    entry.path().display()
                ));
            }
        }
    }

    fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    let date = today();
    let adr = generate_template(&adapter, id, title, &date);
    let toml_str = serialize_adr(&adr).map_err(|e| e.to_string())?;
    fs::write(&file_path, toml_str).map_err(|e| e.to_string())?;
    println!("{}", file_path);
    Ok(())
}

fn cmd_init(dir: &str) -> Result<(), String> {
    validate_dir(dir)?;

    let seed_file = format!("{}/0001-record-architecture-decisions.toml", dir);
    if Path::new(&seed_file).exists() {
        return Err("ADR directory already initialized.".to_string());
    }

    fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    if dir != "docs/adr" {
        fs::write(".adr-dir", dir).map_err(|e| e.to_string())?;
    }

    let date = today();
    let mut adr = generate_template("local", "0001", "Record architecture decisions", &date);
    adr.meta.title = "1. Record architecture decisions".to_string();
    adr.meta.status = "Accepted".to_string();
    adr.meta.work_item = String::new();
    adr.context.body = "We need to record the architectural decisions made on this project.".to_string();
    adr.options = vec![
        AdrOption {
            name: "No documentation".to_string(),
            body: "Keep decisions informal, undocumented.".to_string(),
        },
        AdrOption {
            name: "Use ADRs".to_string(),
            body: "Record decisions as Architecture Decision Records.".to_string(),
        },
    ];
    adr.evaluation_checkpoint.assessment = "Proceed".to_string();
    adr.evaluation_checkpoint.items = vec![
        CheckpointItem { label: "All options evaluated at comparable depth".to_string(), checked: true },
        CheckpointItem { label: "Decision drivers are defined and referenced in option analysis".to_string(), checked: true },
        CheckpointItem { label: "No unacknowledged experimentation gaps".to_string(), checked: true },
    ];
    adr.decision = Decision {
        chosen_option: Some(1),
        justification: Some("We will use Architecture Decision Records, as described by Michael Nygard.".to_string()),
        body: None,
    };
    adr.consequences = vec![Consequence {
        kind: "positive".to_string(),
        body: "Decisions are documented and discoverable.".to_string(),
    }];
    adr.quality_strategy.backwards_compatible = true;
    adr.conclusion_checkpoint.assessment = "Ready for review".to_string();
    adr.conclusion_checkpoint.items = vec![
        CheckpointItem { label: "Decision justified (Y-statement or equivalent)".to_string(), checked: true },
        CheckpointItem { label: "Consequences include positive, negative, and neutral outcomes".to_string(), checked: true },
        CheckpointItem { label: "Quality Strategy reviewed".to_string(), checked: true },
        CheckpointItem { label: "Links to related ADRs populated".to_string(), checked: true },
    ];
    adr.deliverables = None;

    let file_path = format!("{}/0001-record-architecture-decisions.toml", dir);
    let toml_str = serialize_adr(&adr).map_err(|e| e.to_string())?;
    fs::write(&file_path, toml_str).map_err(|e| e.to_string())?;
    println!("{}", file_path);
    Ok(())
}

fn cmd_list() -> Result<(), String> {
    let dir = resolve_dir();
    let dir_path = Path::new(&dir);
    if !dir_path.is_dir() {
        return Err(format!("ADR directory not found: {}", dir));
    }

    let mut entries: Vec<(String, String, String)> = Vec::new();

    for entry in fs::read_dir(dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".toml") {
            continue;
        }

        let content = fs::read_to_string(entry.path()).map_err(|e| e.to_string())?;
        match parse_adr(&content, &name) {
            Ok(adr) => {
                let sort_key = adr.meta.date.clone();
                entries.push((sort_key, adr.meta.title, adr.meta.status));
            }
            Err(e) => {
                eprintln!("WARNING: skipping {}: {}", name, e);
            }
        }
    }

    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, title, status) in &entries {
        println!("{}\t[{}]", title, status);
    }
    Ok(())
}

fn cmd_rename(remote: &str, id: &str, new_title: &str) -> Result<(), String> {
    let adapter = detect_adapter(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let old_path = find_adr_file(&dir, &adapter, id)?;
    let slug = slugify(new_title);
    let new_path = format!("{}/{}-{}-{}.toml", dir, adapter, id, slug);

    if old_path == new_path {
        println!("No rename needed: {}", Path::new(&old_path).file_name().unwrap().to_string_lossy());
        return Ok(());
    }
    if Path::new(&new_path).exists() {
        return Err(format!("target file already exists: {}", new_path));
    }

    let content = fs::read_to_string(&old_path).map_err(|e| e.to_string())?;
    let mut adr = parse_adr(&content, &old_path).map_err(|e| e.to_string())?;
    adr.meta.title = format!("{}-{}. {}", adapter, id, new_title);
    adr.meta.last_updated = today();

    let toml_str = serialize_adr(&adr).map_err(|e| e.to_string())?;
    let tmp_path = format!("{}.tmp", new_path);
    fs::write(&tmp_path, &toml_str).map_err(|e| e.to_string())?;
    fs::rename(&tmp_path, &new_path).map_err(|e| e.to_string())?;
    if old_path != new_path {
        fs::remove_file(&old_path).map_err(|e| e.to_string())?;
    }

    let old_name = Path::new(&old_path).file_name().unwrap().to_string_lossy();
    let new_name = Path::new(&new_path).file_name().unwrap().to_string_lossy();
    println!("Renamed: {} → {}", old_name, new_name);
    Ok(())
}

fn cmd_status(remote: Option<&str>, id: Option<&str>, new_status: Option<&str>) -> Result<(), String> {
    let dir = resolve_dir();

    match (remote, id) {
        (None, _) | (_, None) => cmd_list(),
        (Some(r), Some(i)) => {
            let adapter = detect_adapter(r)?;
            validate_id(i)?;
            let file_path = find_adr_file(&dir, &adapter, i)?;
            let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
            let mut adr = parse_adr(&content, &file_path).map_err(|e| e.to_string())?;

            match new_status {
                None => {
                    println!("{}", adr.meta.status);
                    Ok(())
                }
                Some(status) => {
                    adr.meta.status = status.to_string();
                    if let Err(errors) = adr.validate() {
                        return Err(errors.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("; "));
                    }
                    let toml_str = serialize_adr(&adr).map_err(|e| e.to_string())?;
                    let tmp_path = format!("{}.tmp", file_path);
                    fs::write(&tmp_path, toml_str).map_err(|e| e.to_string())?;
                    fs::rename(&tmp_path, &file_path).map_err(|e| e.to_string())?;
                    let name = Path::new(&file_path).file_name().unwrap().to_string_lossy();
                    println!("Updated: {} → {}", name, status);
                    Ok(())
                }
            }
        }
    }
}

fn cmd_lifecycle(remote: &str, id: &str, _auto: bool, _sync: bool) -> Result<(), String> {
    let adapter = detect_adapter(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let file_path = find_adr_file(&dir, &adapter, id)?;
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let adr = parse_adr(&content, &file_path).map_err(|e| e.to_string())?;

    let cache_path = ".adr/var/work-items.jsonl";
    if !Path::new(cache_path).exists() {
        println!("No cached work item for {}-{}.", adapter, id);
        println!("Current ADR status: {}", adr.meta.status);
        return Ok(());
    }

    let cache_content = fs::read_to_string(cache_path).map_err(|e| e.to_string())?;
    let mut wi_state = None;
    for line in cache_content.lines() {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
            if val.get("remote").and_then(|v| v.as_str()) == Some(&adapter)
                && val.get("id").and_then(|v| v.as_str()) == Some(id)
            {
                wi_state = val.get("state").and_then(|v| v.as_str()).map(|s| s.to_string());
            }
        }
    }

    match wi_state {
        None => {
            println!("No cached work item for {}-{}.", adapter, id);
            println!("Current ADR status: {}", adr.meta.status);
        }
        Some(state) => {
            let expected = match state.as_str() {
                "open" => "Prototype",
                "active" => "Proposed",
                "resolved" => "Accepted",
                "closed" => "Delivered",
                _ => "unknown",
            };
            let name = Path::new(&file_path).file_name().unwrap().to_string_lossy();
            println!("ADR: {}", name);
            println!("  ADR status:       {}", adr.meta.status);
            println!("  Work item state:  {}", state);
            println!("  Expected status:  {}", expected);

            if adr.meta.status == expected {
                println!("  → In sync. No action needed.");
            } else {
                println!("  → Recommended: Transition to {}", expected);
                if _auto {
                    println!("  → --auto not yet implemented. Transition manually.");
                } else {
                    println!("  Run with --auto to execute this transition.");
                }
            }
        }
    }
    Ok(())
}

fn cmd_export(remote: &str, id: &str) -> Result<(), String> {
    let adapter = detect_adapter(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let file_path = find_adr_file(&dir, &adapter, id)?;
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let adr = parse_adr(&content, &file_path).map_err(|e| e.to_string())?;

    println!("# {}", adr.meta.title);
    println!();
    println!("Date: {}", adr.meta.date);
    println!("Status: {}", adr.meta.status);
    println!("Last Updated: {}", adr.meta.last_updated);
    if !adr.meta.work_item.is_empty() {
        println!("Work-Item: {}", adr.meta.work_item);
    }
    if !adr.meta.links.is_empty() {
        println!("Links: {}", adr.meta.links.join(", "));
    }

    if !adr.context.body.is_empty() {
        println!();
        println!("## Context");
        println!();
        println!("{}", adr.context.body);
    }

    if !adr.options.is_empty() {
        println!();
        println!("## Options");
        for opt in &adr.options {
            println!();
            println!("### {}", opt.name);
            if !opt.body.is_empty() {
                println!();
                println!("{}", opt.body);
            }
        }
    }

    let has_decision = adr.decision.chosen_option.is_some()
        || adr.decision.body.as_ref().map_or(false, |b| !b.is_empty());
    if has_decision {
        println!();
        println!("## Decision");
        println!();
        if let Some(idx) = adr.decision.chosen_option {
            if let Some(opt) = adr.options.get(idx) {
                println!("Chose **{}** (Option {})", opt.name, idx + 1);
            }
            if let Some(ref j) = adr.decision.justification {
                if !j.is_empty() {
                    println!();
                    println!("{}", j);
                }
            }
        } else if let Some(ref body) = adr.decision.body {
            println!("{}", body);
        }
    }

    let positive: Vec<_> = adr.consequences.iter().filter(|c| c.kind == "positive").collect();
    let negative: Vec<_> = adr.consequences.iter().filter(|c| c.kind == "negative").collect();
    let neutral: Vec<_> = adr.consequences.iter().filter(|c| c.kind == "neutral").collect();

    if !positive.is_empty() || !negative.is_empty() || !neutral.is_empty() {
        println!();
        println!("## Consequences");
    }
    if !positive.is_empty() {
        println!();
        println!("**Positive:**");
        for c in &positive {
            println!("- {}", c.body);
        }
    }
    if !negative.is_empty() {
        println!();
        println!("**Negative:**");
        for c in &negative {
            println!("- {}", c.body);
        }
    }
    if !neutral.is_empty() {
        println!();
        println!("**Neutral:**");
        for c in &neutral {
            println!("- {}", c.body);
        }
    }

    Ok(())
}

/// Dispatch an author subcommand, printing errors to stderr and exiting on failure.
pub fn run_author(command: AuthorCommands) {
    let result = match command {
        AuthorCommands::New { remote, id, title, dir } => cmd_new(&remote, &id, &title, &dir),
        AuthorCommands::Init { dir } => {
            let d = dir.unwrap_or_else(|| resolve_dir());
            cmd_init(&d)
        }
        AuthorCommands::List => cmd_list(),
        AuthorCommands::Rename { remote, id, new_title } => cmd_rename(&remote, &id, &new_title),
        AuthorCommands::Status { remote, id, new_status } => {
            cmd_status(remote.as_deref(), id.as_deref(), new_status.as_deref())
        }
        AuthorCommands::Lifecycle { remote, id, auto, sync } => cmd_lifecycle(&remote, &id, auto, sync),
        AuthorCommands::Export { remote, id } => cmd_export(&remote, &id),
    };

    if let Err(e) = result {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_adapter_local() {
        assert_eq!(detect_adapter("local").unwrap(), "local");
    }

    #[test]
    fn test_github_ssh_url() {
        assert_eq!(detect_adapter_from_url("git@github.com:org/repo.git").unwrap(), "gh");
    }

    #[test]
    fn test_github_https_url() {
        assert_eq!(detect_adapter_from_url("https://github.com/org/repo").unwrap(), "gh");
    }

    #[test]
    fn test_github_https_with_port() {
        assert_eq!(detect_adapter_from_url("https://github.com:443/org/repo").unwrap(), "gh");
    }

    #[test]
    fn test_ado_https_url() {
        assert_eq!(detect_adapter_from_url("https://dev.azure.com/org/project/_git/repo").unwrap(), "ado");
    }

    #[test]
    fn test_ado_ssh_url() {
        assert_eq!(detect_adapter_from_url("git@ssh.dev.azure.com:v3/org/project/repo").unwrap(), "ado");
    }

    #[test]
    fn test_ado_visualstudio_url() {
        assert_eq!(detect_adapter_from_url("https://org.visualstudio.com/project/_git/repo").unwrap(), "ado");
    }

    #[test]
    fn test_ssh_scheme_github_url() {
        assert_eq!(detect_adapter_from_url("ssh://git@github.com/org/repo").unwrap(), "gh");
    }

    #[test]
    fn test_unknown_host_returns_error() {
        let result = detect_adapter_from_url("https://gitlab.example.com/org/repo");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Could not detect adapter type"), "Error was: {}", err);
        assert!(err.contains("gitlab.example.com"), "Error was: {}", err);
    }

    #[test]
    fn test_unknown_host_returns_error_custom() {
        let result = detect_adapter_from_url("https://code.example.com/org/repo");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Could not detect adapter type"), "Error was: {}", err);
    }

    #[test]
    fn test_extract_host_ssh() {
        assert_eq!(extract_host("git@github.com:org/repo.git"), "github.com");
    }

    #[test]
    fn test_extract_host_https() {
        assert_eq!(extract_host("https://github.com/org/repo"), "github.com");
    }

    #[test]
    fn test_extract_host_ssh_scheme() {
        assert_eq!(extract_host("ssh://git@github.com/org/repo"), "github.com");
    }

    #[test]
    fn test_extract_host_with_port() {
        assert_eq!(extract_host("https://github.com:443/org/repo"), "github.com");
    }

    #[test]
    fn test_extract_host_case_insensitive() {
        assert_eq!(extract_host("https://GitHub.COM/org/repo"), "github.com");
    }
}
