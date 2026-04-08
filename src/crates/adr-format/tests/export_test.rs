use adr_format::*;
use std::fs;
use std::process::Command;

fn build_test_adr() -> Adr {
    Adr {
        meta: Meta {
            title: "gh-42. Use PostgreSQL".to_string(),
            date: "2026-04-06".to_string(),
            status: "Accepted".to_string(),
            last_updated: "2026-04-06".to_string(),
            identifier: "gh-42".to_string(),
            work_item: "gh#42".to_string(),
            links: vec!["ADR-0034".to_string(), "ADR-0037".to_string()],
        },
        context: Section {
            body: "We need a database for storing audit events.".to_string(),
        },
        options: vec![
            AdrOption {
                name: "PostgreSQL".to_string(),
                body: "Mature relational database with strong JSONB support.".to_string(),
            },
            AdrOption {
                name: "SQLite".to_string(),
                body: "Embedded database, zero deployment overhead.".to_string(),
            },
        ],
        evaluation_checkpoint: Checkpoint {
            assessment: "Proceed".to_string(),
            items: vec![
                CheckpointItem { label: "All options evaluated at comparable depth".to_string(), checked: true },
                CheckpointItem { label: "Decision drivers are defined and referenced".to_string(), checked: true },
                CheckpointItem { label: "No unacknowledged experimentation gaps".to_string(), checked: true },
            ],
            all_options_evaluated: None,
            decision_drivers_referenced: None,
            no_experimentation_gaps: None,
            decision_justified: None,
            consequences_complete: None,
            quality_strategy_reviewed: None,
            links_populated: None,
            validation_needs: String::new(),
            pre_review_notes: String::new(),
        },
        decision: Decision {
            chosen_option: Some(0),
            justification: Some("We chose PostgreSQL for event storage.".to_string()),
            body: None,
        },
        consequences: vec![
            Consequence { kind: "positive".to_string(), body: "Strong JSONB support.".to_string() },
            Consequence { kind: "negative".to_string(), body: "Requires running database.".to_string() },
            Consequence { kind: "neutral".to_string(), body: "Team has expertise.".to_string() },
        ],
        deliverables: None,
        quality_strategy: QualityStrategy {
            major_semantic_changes: false,
            minor_semantic_changes: true,
            fuzz_testing: false,
            unit_testing: true,
            load_testing: false,
            performance_testing: false,
            backwards_compatible: true,
            integration_tests: false,
            tooling: false,
            user_documentation: false,
            additional_concerns: String::new(),
        },
        conclusion_checkpoint: Checkpoint {
            assessment: "Ready for review".to_string(),
            items: vec![
                CheckpointItem { label: "Decision justified (Y-statement or equivalent)".to_string(), checked: true },
                CheckpointItem { label: "Consequences include positive, negative, and neutral outcomes".to_string(), checked: true },
                CheckpointItem { label: "Quality Strategy reviewed".to_string(), checked: true },
                CheckpointItem { label: "Links to related ADRs populated".to_string(), checked: true },
            ],
            all_options_evaluated: None,
            decision_drivers_referenced: None,
            no_experimentation_gaps: None,
            decision_justified: None,
            consequences_complete: None,
            quality_strategy_reviewed: None,
            links_populated: None,
            validation_needs: String::new(),
            pre_review_notes: String::new(),
        },
        plan: None,
        comments: None,
    }
}

#[test]
fn export_snapshot() {
    let dir = tempfile::tempdir().unwrap();
    let adr_dir = dir.path().join("docs/adr");
    fs::create_dir_all(&adr_dir).unwrap();

    // Write a fully populated TOML ADR
    let adr = build_test_adr();
    let toml_str = serialize_adr(&adr).unwrap();
    let adr_path = adr_dir.join("gh-42-use-postgresql.toml");
    fs::write(&adr_path, &toml_str).unwrap();

    // Write .adr/adr-dir to point to our test directory
    let adr_meta = dir.path().join(".adr");
    fs::create_dir_all(&adr_meta).unwrap();
    fs::write(adr_meta.join("adr-dir"), adr_dir.to_string_lossy().as_ref()).unwrap();

    // Run export via the binary
    let binary = env!("CARGO_BIN_EXE_adr-format");
    let output = Command::new(binary)
        .current_dir(dir.path())
        .args(["export", "gh", "42"])
        .output()
        .expect("failed to run adr-format");

    assert!(output.status.success(), "export failed: {}", String::from_utf8_lossy(&output.stderr));

    let md = String::from_utf8(output.stdout).unwrap();

    // Verify included sections
    assert!(md.contains("# gh-42. Use PostgreSQL"));
    assert!(md.contains("Date: 2026-04-06"));
    assert!(md.contains("Status: Accepted"));
    assert!(md.contains("Work-Item: gh#42"));
    assert!(md.contains("Links: ADR-0034, ADR-0037"));
    assert!(md.contains("## Context"));
    assert!(md.contains("We need a database"));
    assert!(md.contains("## Options"));
    assert!(md.contains("### PostgreSQL"));
    assert!(md.contains("### SQLite"));
    assert!(md.contains("## Decision"));
    assert!(md.contains("Chose **PostgreSQL** (Option 1)"));
    assert!(md.contains("We chose PostgreSQL for event storage."));
    assert!(md.contains("## Consequences"));
    assert!(md.contains("**Positive:**"));
    assert!(md.contains("**Negative:**"));
    assert!(md.contains("**Neutral:**"));

    // Verify excluded sections
    assert!(!md.contains("Evaluation Checkpoint"));
    assert!(!md.contains("Conclusion Checkpoint"));
    assert!(!md.contains("Quality Strategy"));
    assert!(!md.contains("Deliverables"));
    assert!(!md.contains("Plan"));
    assert!(!md.contains("Comments"));
}

#[test]
fn export_is_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let adr_dir = dir.path().join("docs/adr");
    fs::create_dir_all(&adr_dir).unwrap();

    let adr = build_test_adr();
    let toml_str = serialize_adr(&adr).unwrap();
    fs::write(adr_dir.join("gh-42-use-postgresql.toml"), &toml_str).unwrap();

    let adr_meta = dir.path().join(".adr");
    fs::create_dir_all(&adr_meta).unwrap();
    fs::write(adr_meta.join("adr-dir"), adr_dir.to_string_lossy().as_ref()).unwrap();

    let binary = env!("CARGO_BIN_EXE_adr-format");
    let run = || {
        Command::new(binary)
            .current_dir(dir.path())
            .args(["export", "gh", "42"])
            .output()
            .expect("failed to run")
    };

    let out1 = run();
    let out2 = run();
    assert_eq!(out1.stdout, out2.stdout, "export is not idempotent");
}

#[test]
fn export_omits_empty_work_item() {
    let dir = tempfile::tempdir().unwrap();
    let adr_dir = dir.path().join("docs/adr");
    fs::create_dir_all(&adr_dir).unwrap();

    let mut adr = build_test_adr();
    adr.meta.work_item = String::new();
    // Need to use local remote for empty work_item
    adr.meta.title = "local-0001. Test".to_string();
    let toml_str = serialize_adr(&adr).unwrap();
    let adr_path = adr_dir.join("local-0001-test.toml");
    fs::write(&adr_path, &toml_str).unwrap();

    let adr_meta = dir.path().join(".adr");
    fs::create_dir_all(&adr_meta).unwrap();
    fs::write(adr_meta.join("adr-dir"), adr_dir.to_string_lossy().as_ref()).unwrap();

    let binary = env!("CARGO_BIN_EXE_adr-format");
    let output = Command::new(binary)
        .current_dir(dir.path())
        .args(["export", "local", "0001"])
        .output()
        .expect("failed to run");

    let md = String::from_utf8(output.stdout).unwrap();
    assert!(!md.contains("Work-Item:"));
}

#[test]
fn export_omits_empty_consequences() {
    let dir = tempfile::tempdir().unwrap();
    let adr_dir = dir.path().join("docs/adr");
    fs::create_dir_all(&adr_dir).unwrap();

    let mut adr = build_test_adr();
    // Keep only positive consequences — negative and neutral should be omitted
    adr.consequences = vec![
        Consequence { kind: "positive".to_string(), body: "Good thing.".to_string() },
    ];
    let toml_str = serialize_adr(&adr).unwrap();
    let adr_path = adr_dir.join("gh-42-use-postgresql.toml");
    fs::write(&adr_path, &toml_str).unwrap();

    let adr_meta = dir.path().join(".adr");
    fs::create_dir_all(&adr_meta).unwrap();
    fs::write(adr_meta.join("adr-dir"), adr_dir.to_string_lossy().as_ref()).unwrap();

    let binary = env!("CARGO_BIN_EXE_adr-format");
    let output = Command::new(binary)
        .current_dir(dir.path())
        .args(["export", "gh", "42"])
        .output()
        .expect("failed to run");

    let md = String::from_utf8(output.stdout).unwrap();
    assert!(md.contains("**Positive:**"));
    assert!(!md.contains("**Negative:**"));
    assert!(!md.contains("**Neutral:**"));
}
