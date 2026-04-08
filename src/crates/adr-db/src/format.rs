use std::fs;
use std::path::Path;

use clap::Subcommand;

use adr_db_lib::{generate_template, parse_adr, serialize_adr};
use adr_db_lib::format::schema::{AdrOption, CheckpointItem, Consequence, Decision};

/// ADR format subcommand definitions (TOML document management).
#[derive(Subcommand)]
pub enum FormatCommands {
    /// Create a new ADR with work-item-referenced naming
    New {
        /// Remote identifier (gh, ado, gitea, local)
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
        /// Remote identifier
        remote: String,
        /// Work item ID
        id: String,
        /// New title
        new_title: String,
    },
    /// Show or update ADR status
    Status {
        /// Remote identifier
        remote: Option<String>,
        /// Work item ID
        id: Option<String>,
        /// New status to set
        new_status: Option<String>,
    },
    /// Check or execute lifecycle transition
    Lifecycle {
        /// Remote identifier
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
        /// Remote identifier
        remote: String,
        /// Work item ID
        id: String,
    },
}

const VALID_REMOTES: &[&str] = &["gh", "ado", "gitea", "local"];

fn validate_remote(remote: &str) -> Result<(), String> {
    if VALID_REMOTES.contains(&remote) {
        Ok(())
    } else {
        Err(format!(
            "unknown remote '{}'. Allowed: {}",
            remote,
            VALID_REMOTES.join(", ")
        ))
    }
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
    validate_remote(remote)?;
    validate_id(id)?;
    validate_dir(dir)?;
    if title.is_empty() {
        return Err("title is required".to_string());
    }

    let slug = slugify(title);
    let file_path = format!("{}/{}-{}-{}.toml", dir, remote, id, slug);

    let prefix = format!("{}-{}-", remote, id);
    if Path::new(dir).is_dir() {
        for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(&prefix) && name.ends_with(".toml") {
                return Err(format!(
                    "ADR for {}-{} already exists: {}",
                    remote,
                    id,
                    entry.path().display()
                ));
            }
        }
    }

    fs::create_dir_all(dir).map_err(|e| e.to_string())?;

    let date = today();
    let adr = generate_template(remote, id, title, &date);
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
    validate_remote(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let old_path = find_adr_file(&dir, remote, id)?;
    let slug = slugify(new_title);
    let new_path = format!("{}/{}-{}-{}.toml", dir, remote, id, slug);

    if old_path == new_path {
        println!("No rename needed: {}", Path::new(&old_path).file_name().unwrap().to_string_lossy());
        return Ok(());
    }
    if Path::new(&new_path).exists() {
        return Err(format!("target file already exists: {}", new_path));
    }

    let content = fs::read_to_string(&old_path).map_err(|e| e.to_string())?;
    let mut adr = parse_adr(&content, &old_path).map_err(|e| e.to_string())?;
    adr.meta.title = format!("{}-{}. {}", remote, id, new_title);
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
            validate_remote(r)?;
            validate_id(i)?;
            let file_path = find_adr_file(&dir, r, i)?;
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
    validate_remote(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let file_path = find_adr_file(&dir, remote, id)?;
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let adr = parse_adr(&content, &file_path).map_err(|e| e.to_string())?;

    let cache_path = ".adr/var/work-items.jsonl";
    if !Path::new(cache_path).exists() {
        println!("No cached work item for {}-{}.", remote, id);
        println!("Current ADR status: {}", adr.meta.status);
        return Ok(());
    }

    let cache_content = fs::read_to_string(cache_path).map_err(|e| e.to_string())?;
    let mut wi_state = None;
    for line in cache_content.lines() {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(line) {
            if val.get("remote").and_then(|v| v.as_str()) == Some(remote)
                && val.get("id").and_then(|v| v.as_str()) == Some(id)
            {
                wi_state = val.get("state").and_then(|v| v.as_str()).map(|s| s.to_string());
            }
        }
    }

    match wi_state {
        None => {
            println!("No cached work item for {}-{}.", remote, id);
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
    validate_remote(remote)?;
    validate_id(id)?;

    let dir = resolve_dir();
    let file_path = find_adr_file(&dir, remote, id)?;
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

/// Dispatch a format subcommand, printing errors to stderr and exiting on failure.
pub fn run_format(command: FormatCommands) {
    let result = match command {
        FormatCommands::New { remote, id, title, dir } => cmd_new(&remote, &id, &title, &dir),
        FormatCommands::Init { dir } => {
            let d = dir.unwrap_or_else(|| resolve_dir());
            cmd_init(&d)
        }
        FormatCommands::List => cmd_list(),
        FormatCommands::Rename { remote, id, new_title } => cmd_rename(&remote, &id, &new_title),
        FormatCommands::Status { remote, id, new_status } => {
            cmd_status(remote.as_deref(), id.as_deref(), new_status.as_deref())
        }
        FormatCommands::Lifecycle { remote, id, auto, sync } => cmd_lifecycle(&remote, &id, auto, sync),
        FormatCommands::Export { remote, id } => cmd_export(&remote, &id),
    };

    if let Err(e) = result {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}
