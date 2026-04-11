//! Install logic — writes embedded skill and agent files to the target directory.

use crate::embed::SkillAssets;
use crate::embed::AgentAssets;
use std::fs;
use std::path::{Component, Path, PathBuf};

const VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_COMMIT_SHA"), ")");

fn resolve_base(prefix: &Option<PathBuf>) -> PathBuf {
    if let Some(p) = prefix {
        if p.as_os_str().is_empty() {
            eprintln!("Error: --prefix cannot be empty");
            std::process::exit(1);
        }
        if p == Path::new("/") {
            eprintln!("Error: --prefix cannot be the filesystem root");
            std::process::exit(1);
        }
        if p.components().any(|c| c == Component::ParentDir) {
            eprintln!("Error: --prefix must not contain '..' components");
            std::process::exit(1);
        }
        p.clone()
    } else {
        dirs::home_dir()
            .expect("Could not determine home directory")
            .join(".copilot")
    }
}

fn read_version_marker(dir: &Path) -> Option<String> {
    match fs::read_to_string(dir.join(".version")) {
        Ok(v) => Some(v.trim().to_string()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => {
            eprintln!("Warning: could not read version marker at {}: {e}", dir.display());
            None
        }
    }
}

fn write_version_marker(dir: &Path) {
    fs::write(dir.join(".version"), VERSION).unwrap_or_else(|e| {
        eprintln!("Warning: could not write version marker: {e}");
    });
}

fn should_install(dir: &Path, dry_run: bool, force: bool, label: &str) -> bool {
    // dry-run takes precedence over force
    if dry_run {
        let count = count_embedded_skills_or_agents(label);
        println!("Dry run: would install {count} {label} files to {}", dir.display());
        if let Some(installed) = read_version_marker(dir) {
            println!("  Installed version: {installed}");
            println!("  Embedded version:  {VERSION}");
            if installed == VERSION {
                println!("  Status: already up-to-date");
            } else {
                println!("  Status: version mismatch — would update");
            }
        } else {
            println!("  No version marker found — would install fresh");
        }
        return false;
    }

    if force {
        return true;
    }

    if let Some(installed) = read_version_marker(dir) {
        if installed == VERSION {
            println!("Already up-to-date ({label}, version: {VERSION})");
            return false;
        }
    }

    true
}

fn count_embedded_skills_or_agents(label: &str) -> usize {
    match label {
        "skill" => SkillAssets::iter().count(),
        "agent" => AgentAssets::iter().count(),
        _ => unreachable!("unknown asset label: {label}"),
    }
}

/// Install all skill definitions to `<base>/skills/`.
pub fn install_skills(prefix: &Option<PathBuf>, dry_run: bool, force: bool) {
    let base = resolve_base(prefix);
    let skills_dir = base.join("skills");

    if !should_install(&skills_dir, dry_run, force, "skill") {
        return;
    }

    // Remove old skills — fatal if this fails
    if skills_dir.exists() {
        fs::remove_dir_all(&skills_dir).unwrap_or_else(|e| {
            eprintln!("Error: could not remove {}: {e}", skills_dir.display());
            eprintln!("No files were changed.");
            std::process::exit(1);
        });
    }

    let mut count = 0;
    for path in SkillAssets::iter() {
        let file_path = skills_dir.join(path.as_ref());
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Error: could not create directory {}: {e}", parent.display());
                eprintln!("Installed {count} files before failure");
                std::process::exit(1);
            });
        }
        let data = SkillAssets::get(path.as_ref()).unwrap();
        fs::write(&file_path, data.data.as_ref()).unwrap_or_else(|e| {
            eprintln!("Error: could not write {}: {e}", file_path.display());
            eprintln!("Installed {count} files before failure");
            std::process::exit(1);
        });
        count += 1;
    }
    write_version_marker(&skills_dir);
    println!("Installed {count} skill files to {}", skills_dir.display());
}

/// Install all agent definitions to `<base>/agents/`.
pub fn install_agents(prefix: &Option<PathBuf>, dry_run: bool, force: bool) {
    let base = resolve_base(prefix);
    let agents_dir = base.join("agents");

    if !should_install(&agents_dir, dry_run, force, "agent") {
        return;
    }

    // Remove old agents — fatal if this fails
    if agents_dir.exists() {
        fs::remove_dir_all(&agents_dir).unwrap_or_else(|e| {
            eprintln!("Error: could not remove {}: {e}", agents_dir.display());
            eprintln!("No files were changed.");
            std::process::exit(1);
        });
    }

    fs::create_dir_all(&agents_dir).unwrap_or_else(|e| {
        eprintln!("Error: could not create {}: {e}", agents_dir.display());
        std::process::exit(1);
    });

    let mut count = 0;
    for path in AgentAssets::iter() {
        let file_path = agents_dir.join(path.as_ref());
        let data = AgentAssets::get(path.as_ref()).unwrap();
        fs::write(&file_path, data.data.as_ref()).unwrap_or_else(|e| {
            eprintln!("Error: could not write {}: {e}", file_path.display());
            eprintln!("Installed {count} files before failure");
            std::process::exit(1);
        });
        count += 1;
    }
    write_version_marker(&agents_dir);
    println!("Installed {count} agent files to {}", agents_dir.display());
}

/// Install both skills and agents.
pub fn install_all(prefix: &Option<PathBuf>, dry_run: bool, force: bool) {
    install_skills(prefix, dry_run, force);
    install_agents(prefix, dry_run, force);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_install_skills_creates_files() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_skills(&prefix, false, false);

        let skills_dir = tmp.path().join("skills");
        assert!(skills_dir.exists());

        for skill in ["author-adr", "implement-adr", "prototype-adr", "solve-adr"] {
            assert!(
                skills_dir.join(skill).exists(),
                "Missing skill directory: {skill}"
            );
        }
    }

    #[test]
    fn test_install_agents_creates_files() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_agents(&prefix, false, false);

        let agents_dir = tmp.path().join("agents");
        assert!(agents_dir.exists());

        let agent_count = fs::read_dir(&agents_dir).unwrap().count();
        assert!(agent_count > 0, "No agent files installed");
    }

    #[test]
    fn test_install_all_creates_both() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_all(&prefix, false, false);

        assert!(tmp.path().join("skills").exists());
        assert!(tmp.path().join("agents").exists());
    }

    #[test]
    fn test_reinstall_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());

        install_skills(&prefix, false, false);
        let first_count: usize = count_files(&tmp.path().join("skills"));

        install_skills(&prefix, false, true); // force to actually reinstall
        let second_count: usize = count_files(&tmp.path().join("skills"));

        assert_eq!(first_count, second_count, "Re-install changed file count");
    }

    #[test]
    fn test_version_marker_written() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_skills(&prefix, false, false);

        let version_file = tmp.path().join("skills/.version");
        assert!(version_file.exists(), ".version file not created");

        let content = fs::read_to_string(&version_file).unwrap();
        assert_eq!(content.trim(), VERSION);
    }

    #[test]
    fn test_version_match_skips_install() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());

        // First install
        install_skills(&prefix, false, false);
        let first_count = count_files(&tmp.path().join("skills"));

        // Write a marker file to detect if files were rewritten
        let marker = tmp.path().join("skills/author-adr/.test-marker");
        fs::write(&marker, "test").unwrap();

        // Second install — should skip due to version match
        install_skills(&prefix, false, false);

        // Marker should still exist (files were not removed/rewritten)
        assert!(marker.exists(), "Version-matched install should not rewrite files");
        assert_eq!(count_files(&tmp.path().join("skills")), first_count + 1); // +1 for marker
    }

    #[test]
    fn test_force_overwrites_matching_version() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());

        install_skills(&prefix, false, false);

        let marker = tmp.path().join("skills/author-adr/.test-marker");
        fs::write(&marker, "test").unwrap();

        // Force install — should overwrite despite version match
        install_skills(&prefix, false, true);

        // Marker should be gone (dir was removed and rewritten)
        assert!(!marker.exists(), "Force install should overwrite files");
    }

    #[test]
    fn test_dry_run_does_not_write() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());

        install_skills(&prefix, true, false);

        // Nothing should have been written
        assert!(!tmp.path().join("skills").exists(), "Dry run should not create files");
    }

    #[test]
    fn test_version_read_error_treated_as_missing() {
        let tmp = TempDir::new().unwrap();
        // read_version_marker on a non-existent dir returns None
        let result = read_version_marker(&tmp.path().join("nonexistent"));
        assert!(result.is_none());
    }

    fn count_files(dir: &Path) -> usize {
        let mut count = 0;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    count += count_files(&path);
                } else {
                    count += 1;
                }
            }
        }
        count
    }
}
