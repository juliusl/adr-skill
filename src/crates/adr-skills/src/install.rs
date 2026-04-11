use crate::embed::SkillAssets;
use crate::embed::AgentAssets;
use std::fs;
use std::path::{Component, Path, PathBuf};

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

/// Install all skill definitions to `<base>/skills/`.
pub fn install_skills(prefix: &Option<PathBuf>) {
    let base = resolve_base(prefix);
    let skills_dir = base.join("skills");

    // Remove old skills
    if skills_dir.exists() {
        fs::remove_dir_all(&skills_dir).unwrap_or_else(|e| {
            eprintln!("Warning: could not remove {}: {e}", skills_dir.display());
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
    println!("Installed {count} skill files to {}", skills_dir.display());
}

/// Install all agent definitions to `<base>/agents/`.
pub fn install_agents(prefix: &Option<PathBuf>) {
    let base = resolve_base(prefix);
    let agents_dir = base.join("agents");

    // Remove old agents to prevent stale files
    if agents_dir.exists() {
        fs::remove_dir_all(&agents_dir).unwrap_or_else(|e| {
            eprintln!("Warning: could not remove {}: {e}", agents_dir.display());
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
    println!("Installed {count} agent files to {}", agents_dir.display());
}

/// Install both skills and agents.
pub fn install_all(prefix: &Option<PathBuf>) {
    install_skills(prefix);
    install_agents(prefix);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_install_skills_creates_files() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_skills(&prefix);

        let skills_dir = tmp.path().join("skills");
        assert!(skills_dir.exists());

        // Verify all 4 skill directories exist
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
        install_agents(&prefix);

        let agents_dir = tmp.path().join("agents");
        assert!(agents_dir.exists());

        // Verify agent files exist
        let agent_count = fs::read_dir(&agents_dir).unwrap().count();
        assert!(agent_count > 0, "No agent files installed");
    }

    #[test]
    fn test_install_all_creates_both() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());
        install_all(&prefix);

        assert!(tmp.path().join("skills").exists());
        assert!(tmp.path().join("agents").exists());
    }

    #[test]
    fn test_reinstall_is_idempotent() {
        let tmp = TempDir::new().unwrap();
        let prefix = Some(tmp.path().to_path_buf());

        install_skills(&prefix);
        let first_count: usize = count_files(&tmp.path().join("skills"));

        install_skills(&prefix);
        let second_count: usize = count_files(&tmp.path().join("skills"));

        assert_eq!(first_count, second_count, "Re-install changed file count");
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
