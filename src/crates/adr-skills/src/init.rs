use std::fs;
use std::path::Path;

pub fn init_project(path: &Path) {
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
        let content = r#"[author]
scope = "user"

[author.dispatch]
review = "juliusl-editor-v5"
tech_writer = "juliusl-tech-writer-v1"
ux_review = "juliusl-ux-reviewer-v1"
dx_review = "juliusl-dx-reviewer-v1"
tpm = "juliusl-tpm-v1"

[implement]
participation = "autonomous"
auto_commit = true

[solve]
participation = "autonomous"
auto_delegate = true

[solve.dispatch]
code_review = ["juliusl-code-reviewer-analytics-v5", "juliusl-code-reviewer-sweep-v5"]
"#;
        fs::write(&prefs, content).unwrap_or_else(|e| {
            eprintln!("Error: could not write {}: {e}", prefs.display());
            std::process::exit(1);
        });
        println!("Created {} (scope = user, full automation)", prefs.display());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_init_creates_expected_files() {
        let tmp = TempDir::new().unwrap();
        init_project(tmp.path());

        assert!(tmp.path().join(".adr").exists());
        assert!(tmp.path().join(".adr/.gitignore").exists());
        assert!(tmp.path().join(".adr/preferences.toml").exists());

        let gitignore = fs::read_to_string(tmp.path().join(".adr/.gitignore")).unwrap();
        assert!(gitignore.contains("var/"));
        assert!(gitignore.contains("usr/"));

        let prefs = fs::read_to_string(tmp.path().join(".adr/preferences.toml")).unwrap();
        assert!(prefs.contains("[author]"));
        assert!(prefs.contains("[solve]"));
    }

    #[test]
    fn test_reinit_skips_existing() {
        let tmp = TempDir::new().unwrap();
        init_project(tmp.path());

        // Modify preferences to verify it's not overwritten
        let prefs_path = tmp.path().join(".adr/preferences.toml");
        fs::write(&prefs_path, "custom content").unwrap();

        init_project(tmp.path());

        let content = fs::read_to_string(&prefs_path).unwrap();
        assert_eq!(content, "custom content", "Re-init overwrote existing file");
    }
}
