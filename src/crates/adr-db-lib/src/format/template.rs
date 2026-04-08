use super::schema::*;

/// Generate a default ADR template for the given remote, id, and title.
pub fn generate_template(remote: &str, id: &str, title: &str, date: &str) -> Adr {
    let work_item = format!("{}#{}", remote, id);
    let identifier = format!("{}-{}", remote, id);
    let display_title = format!("{}-{}. {}", remote, id, title);

    Adr {
        meta: Meta {
            title: display_title,
            date: date.to_string(),
            status: "Prototype".to_string(),
            last_updated: date.to_string(),
            identifier,
            work_item,
            links: Vec::new(),
        },
        context: Section {
            body: String::new(),
        },
        options: Vec::new(),
        evaluation_checkpoint: Checkpoint {
            assessment: "[Proceed | Pause for validation | Skipped — <rationale>]".to_string(),
            items: vec![
                CheckpointItem { label: "All options evaluated at comparable depth".to_string(), checked: false },
                CheckpointItem { label: "Decision drivers are defined and referenced in option analysis".to_string(), checked: false },
                CheckpointItem { label: "No unacknowledged experimentation gaps".to_string(), checked: false },
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
            chosen_option: None,
            justification: None,
            body: None,
        },
        consequences: Vec::new(),
        deliverables: None,
        quality_strategy: QualityStrategy {
            major_semantic_changes: false,
            minor_semantic_changes: false,
            fuzz_testing: false,
            unit_testing: false,
            load_testing: false,
            performance_testing: false,
            backwards_compatible: false,
            integration_tests: false,
            tooling: false,
            user_documentation: false,
            additional_concerns: String::new(),
        },
        conclusion_checkpoint: Checkpoint {
            assessment: "[Ready for review | Needs work | Skipped — <rationale>]".to_string(),
            items: vec![
                CheckpointItem { label: "Decision justified (Y-statement or equivalent)".to_string(), checked: false },
                CheckpointItem { label: "Consequences include positive, negative, and neutral outcomes".to_string(), checked: false },
                CheckpointItem { label: "Quality Strategy reviewed".to_string(), checked: false },
                CheckpointItem { label: "Links to related ADRs populated".to_string(), checked: false },
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
        comments: Some(Comments {
            draft_worksheet: String::new(),
            revision_entries: Vec::new(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::{parse_adr, serialize_adr};

    #[test]
    fn template_has_correct_title() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        assert_eq!(adr.meta.title, "gh-42. Use PostgreSQL");
    }

    #[test]
    fn template_has_prototype_status() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        assert_eq!(adr.meta.status, "Prototype");
    }

    #[test]
    fn template_serializes_to_valid_toml() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        let toml_str = serialize_adr(&adr).expect("serialize failed");
        assert!(!toml_str.is_empty());
    }

    #[test]
    fn template_round_trips() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        let toml_str = serialize_adr(&adr).expect("serialize failed");
        let parsed = parse_adr(&toml_str, "test.toml").expect("parse failed");
        assert_eq!(adr, parsed);
    }

    #[test]
    fn template_has_nonempty_assessments() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        assert!(!adr.evaluation_checkpoint.assessment.is_empty());
        assert!(!adr.conclusion_checkpoint.assessment.is_empty());
    }

    #[test]
    fn template_work_item_format() {
        let adr = generate_template("gh", "42", "Use PostgreSQL", "2026-04-06");
        assert_eq!(adr.meta.work_item, "gh#42");
    }
}
