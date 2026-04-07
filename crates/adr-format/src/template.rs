use crate::schema::*;

/// Generate a default ADR template for the given remote, id, and title.
pub fn generate_template(remote: &str, id: &str, title: &str, date: &str) -> Adr {
    let work_item = format!("{}#{}", remote, id);
    let display_title = format!("{}-{}. {}", remote, id, title);

    Adr {
        meta: Meta {
            title: display_title,
            date: date.to_string(),
            status: "Prototype".to_string(),
            last_updated: date.to_string(),
            work_item,
            links: Vec::new(),
        },
        context: Section {
            body: String::new(),
        },
        options: Vec::new(),
        evaluation_checkpoint: Checkpoint {
            assessment: "[Proceed | Pause for validation | Skipped — <rationale>]".to_string(),
            all_options_evaluated: false,
            decision_drivers_referenced: false,
            no_experimentation_gaps: false,
            decision_justified: false,
            consequences_complete: false,
            quality_strategy_reviewed: false,
            links_populated: false,
            validation_needs: String::new(),
            pre_review_notes: String::new(),
        },
        decision: Section {
            body: String::new(),
        },
        consequences: Vec::new(),
        deliverables: Some(Deliverables {
            items: vec![DeliverableItem {
                description: "[Expected artifact or outcome]".to_string(),
                done: false,
                artifact: String::new(),
            }],
        }),
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
            all_options_evaluated: false,
            decision_drivers_referenced: false,
            no_experimentation_gaps: false,
            decision_justified: false,
            consequences_complete: false,
            quality_strategy_reviewed: false,
            links_populated: false,
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
    use crate::{parse_adr, serialize_adr};

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
