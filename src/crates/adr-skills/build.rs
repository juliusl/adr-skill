fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = std::path::Path::new(&manifest_dir)
        .parent()
        .expect("expected adr-skills crate dir")
        .parent()
        .expect("expected src/crates dir")
        .parent()
        .expect("expected repo root (src/)");

    let skills_dir = workspace_root.join("src/skills");
    let agents_dir = workspace_root.join("src/agents");

    if !skills_dir.exists()
        || skills_dir
            .read_dir()
            .map(|mut d| d.next().is_none())
            .unwrap_or(true)
    {
        panic!("src/skills/ is missing or empty — RustEmbed requires skill files at compile time");
    }
    if !agents_dir.exists()
        || agents_dir
            .read_dir()
            .map(|mut d| d.next().is_none())
            .unwrap_or(true)
    {
        panic!("src/agents/ is missing or empty — RustEmbed requires agent files at compile time");
    }

    // Re-run on any file change inside embedded directories.
    // Directory entries catch additions/deletions; file entries catch modifications.
    for dir in [&skills_dir, &agents_dir] {
        println!("cargo::rerun-if-changed={}", dir.display());
        for entry in walkdir(dir) {
            println!("cargo::rerun-if-changed={}", entry.display());
        }
    }

    // Embed git commit SHA
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();
    if let Ok(output) = output {
        let sha = String::from_utf8_lossy(&output.stdout).trim().to_string();
        println!("cargo::rustc-env=GIT_COMMIT_SHA={sha}");
    } else {
        println!("cargo::rustc-env=GIT_COMMIT_SHA=unknown");
    }
}

fn walkdir(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = vec![];
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(walkdir(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}
