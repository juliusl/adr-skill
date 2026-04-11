//! Project initialization — bootstraps `.adr/` directory with `.gitignore` and `preferences.toml`.

use std::fs;
use std::path::Path;

/// Configuration for project initialization, populated from CLI flags.
pub struct InitConfig {
    /// Participation mode (guided or autonomous).
    pub participation: String,
    /// Enable auto-commit on task completion.
    pub auto_commit: bool,
    /// Enable auto-delegate to implement-adr.
    pub auto_delegate: bool,
    /// Author scope (project or user).
    pub scope: String,
    /// TPM agent name.
    pub tpm: String,
    /// Review agent name.
    pub review: String,
    /// Tech writer agent name.
    pub tech_writer: String,
    /// UX review agent name.
    pub ux_review: String,
    /// DX review agent name.
    pub dx_review: String,
    /// Code review agents (comma-separated).
    pub code_review: String,
}

impl InitConfig {
    /// Build an InitConfig from the shared CLI args struct.
    pub fn from_args(args: &crate::InitArgs) -> Self {
        Self {
            participation: args.participation.to_string(),
            auto_commit: args.auto_commit,
            auto_delegate: args.auto_delegate,
            scope: args.scope.clone(),
            tpm: args.tpm.clone(),
            review: args.review.clone(),
            tech_writer: args.tech_writer.clone(),
            ux_review: args.ux_review.clone(),
            dx_review: args.dx_review.clone(),
            code_review: args.code_review.clone(),
        }
    }
}

fn validate_toml_value(value: &str, field: &str) {
    if value.contains('\n') || value.contains('\r') {
        eprintln!("Error: --{field} must not contain newlines");
        std::process::exit(1);
    }
    if value.contains('"') {
        eprintln!("Error: --{field} must not contain double-quote characters");
        std::process::exit(1);
    }
}

fn format_code_review_array(raw: &str) -> String {
    let items: Vec<&str> = raw
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if items.is_empty() {
        return "[]".to_string();
    }
    let quoted: Vec<String> = items.iter().map(|s| format!("\"{s}\"")).collect();
    format!("[{}]", quoted.join(", "))
}

/// Bootstrap `.adr/` directory with `.gitignore` and `preferences.toml`.
pub fn init_project(path: &Path, config: &InitConfig) {
    validate_toml_value(&config.participation, "participation");
    validate_toml_value(&config.scope, "scope");
    validate_toml_value(&config.tpm, "tpm");
    validate_toml_value(&config.review, "review");
    validate_toml_value(&config.tech_writer, "tech-writer");
    validate_toml_value(&config.ux_review, "ux-review");
    validate_toml_value(&config.dx_review, "dx-review");
    validate_toml_value(&config.code_review, "code-review");

    let adr_dir = path.join(".adr");
    fs::create_dir_all(&adr_dir).unwrap_or_else(|e| {
        eprintln!("Error: could not create {}: {e}", adr_dir.display());
        std::process::exit(1);
    });

    let gitignore = adr_dir.join(".gitignore");
    if gitignore.exists() {
        println!("{} already exists — skipping", gitignore.display());
    } else {
        fs::write(&gitignore, "var/\nusr/\n").unwrap_or_else(|e| {
            eprintln!("Error: could not write {}: {e}", gitignore.display());
            std::process::exit(1);
        });
        println!("Created {}", gitignore.display());
    }

    let prefs = adr_dir.join("preferences.toml");
    if prefs.exists() {
        println!("{} already exists — skipping", prefs.display());
    } else {
        let code_review_array = format_code_review_array(&config.code_review);
        let content = format!(
            r#"[author]
scope = "{scope}"

[author.dispatch]
review = "{review}"
tech_writer = "{tech_writer}"
ux_review = "{ux_review}"
dx_review = "{dx_review}"
tpm = "{tpm}"

[implement]
participation = "{participation}"
auto_commit = {auto_commit}

[solve]
participation = "{participation}"
auto_delegate = {auto_delegate}

[solve.dispatch]
code_review = {code_review}
"#,
            scope = config.scope,
            review = config.review,
            tech_writer = config.tech_writer,
            ux_review = config.ux_review,
            dx_review = config.dx_review,
            tpm = config.tpm,
            participation = config.participation,
            auto_commit = config.auto_commit,
            auto_delegate = config.auto_delegate,
            code_review = code_review_array,
        );
        fs::write(&prefs, content).unwrap_or_else(|e| {
            eprintln!("Error: could not write {}: {e}", prefs.display());
            std::process::exit(1);
        });
        println!("Created {} (scope = {}, participation = {})", prefs.display(), config.scope, config.participation);
    }
}

/// Default config matching the embedded defaults.
pub fn default_config() -> InitConfig {
    InitConfig {
        participation: "autonomous".to_string(),
        auto_commit: true,
        auto_delegate: true,
        scope: "user".to_string(),
        tpm: "juliusl-tpm-v2".to_string(),
        review: "juliusl-editor-v5".to_string(),
        tech_writer: "juliusl-tech-writer-v1".to_string(),
        ux_review: "juliusl-ux-reviewer-v1".to_string(),
        dx_review: "juliusl-dx-reviewer-v1".to_string(),
        code_review: "juliusl-code-reviewer-analytics-v5,juliusl-code-reviewer-sweep-v5".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_expected_files() {
        let tmp = TempDir::new().unwrap();
        let config = default_config();
        init_project(tmp.path(), &config);

        assert!(tmp.path().join(".adr").exists());
        assert!(tmp.path().join(".adr/.gitignore").exists());
        assert!(tmp.path().join(".adr/preferences.toml").exists());

        let gitignore = fs::read_to_string(tmp.path().join(".adr/.gitignore")).unwrap();
        assert!(gitignore.contains("var/"));
        assert!(gitignore.contains("usr/"));

        let prefs = fs::read_to_string(tmp.path().join(".adr/preferences.toml")).unwrap();
        assert!(prefs.contains("[author]"));
        assert!(prefs.contains("[solve]"));
        assert!(prefs.contains("tpm = \"juliusl-tpm-v2\""));
    }

    #[test]
    fn test_reinit_skips_existing() {
        let tmp = TempDir::new().unwrap();
        let config = default_config();
        init_project(tmp.path(), &config);

        // Modify preferences to verify it's not overwritten
        let prefs_path = tmp.path().join(".adr/preferences.toml");
        fs::write(&prefs_path, "custom content").unwrap();

        init_project(tmp.path(), &config);

        let content = fs::read_to_string(&prefs_path).unwrap();
        assert_eq!(content, "custom content", "Re-init overwrote existing file");
    }

    #[test]
    fn test_custom_flags_produce_correct_toml() {
        let tmp = TempDir::new().unwrap();
        let config = InitConfig {
            participation: "guided".to_string(),
            auto_commit: false,
            auto_delegate: false,
            scope: "project".to_string(),
            tpm: "custom-tpm".to_string(),
            review: "custom-reviewer".to_string(),
            tech_writer: "custom-writer".to_string(),
            ux_review: "custom-ux".to_string(),
            dx_review: "custom-dx".to_string(),
            code_review: "reviewer-a,reviewer-b".to_string(),
        };
        init_project(tmp.path(), &config);

        let prefs = fs::read_to_string(tmp.path().join(".adr/preferences.toml")).unwrap();
        assert!(prefs.contains("participation = \"guided\""));
        assert!(prefs.contains("auto_commit = false"));
        assert!(prefs.contains("scope = \"project\""));
        assert!(prefs.contains("tpm = \"custom-tpm\""));
        assert!(prefs.contains(r#"code_review = ["reviewer-a", "reviewer-b"]"#));
    }

    #[test]
    fn test_code_review_comma_edge_cases() {
        // Empty entries are filtered out
        assert_eq!(format_code_review_array("a,,b"), r#"["a", "b"]"#);
        assert_eq!(format_code_review_array(","), "[]");
        assert_eq!(format_code_review_array(""), "[]");
        assert_eq!(format_code_review_array("  a , b  "), r#"["a", "b"]"#);
    }

    #[test]
    fn test_default_config_produces_valid_toml() {
        let tmp = TempDir::new().unwrap();
        let config = default_config();
        init_project(tmp.path(), &config);

        let prefs = fs::read_to_string(tmp.path().join(".adr/preferences.toml")).unwrap();
        // Verify key structural elements
        assert!(prefs.contains("[author]"));
        assert!(prefs.contains("[author.dispatch]"));
        assert!(prefs.contains("[implement]"));
        assert!(prefs.contains("[solve]"));
        assert!(prefs.contains("[solve.dispatch]"));
    }
}
