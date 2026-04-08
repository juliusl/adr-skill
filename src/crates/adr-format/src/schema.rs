use serde::{Deserialize, Serialize};
use std::fmt;

/// Valid ADR status values.
const VALID_STATUSES: &[&str] = &[
    "Prototype",
    "Proposed",
    "Ready",
    "Planned",
    "Accepted",
    "Delivered",
    "Deprecated",
    "Superseded",
];

/// Top-level ADR document in wi-full-agent-adr TOML format.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Adr {
    pub meta: Meta,
    pub context: Section,
    #[serde(default)]
    pub options: Vec<AdrOption>,
    pub evaluation_checkpoint: Checkpoint,
    pub decision: Decision,
    #[serde(default)]
    pub consequences: Vec<Consequence>,
    /// Deprecated per ADR-0057 design rule 4 — plan data lives in separate files.
    /// Retained for backward-compatible deserialization of ADR-0051 format files.
    #[serde(default)]
    pub deliverables: Option<Deliverables>,
    pub quality_strategy: QualityStrategy,
    pub conclusion_checkpoint: Checkpoint,
    /// Deprecated per ADR-0057 design rule 4 — plan data lives in separate files.
    /// Retained for backward-compatible deserialization of ADR-0051 format files.
    #[serde(default)]
    pub plan: Option<Plan>,
    pub comments: Option<Comments>,
}

/// ADR metadata — title, date, status, work-item reference, links.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Meta {
    pub title: String,
    pub date: String,
    pub status: String,
    pub last_updated: String,
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub work_item: String,
    #[serde(default)]
    pub links: Vec<String>,
}

/// A prose section with a body field.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    #[serde(default)]
    pub body: String,
}

/// A named option with a prose body.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdrOption {
    pub name: String,
    #[serde(default)]
    pub body: String,
}

/// A single checkpoint checklist item (per ADR-0057 design rule 1).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckpointItem {
    pub label: String,
    #[serde(default)]
    pub checked: bool,
}

/// Checkpoint with mandatory assessment and checklist items.
/// Supports both new format (items array) and legacy format (named boolean fields).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checkpoint {
    pub assessment: String,
    #[serde(default)]
    pub items: Vec<CheckpointItem>,
    // Legacy fields — accepted for backward compat with ADR-0051 format
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_options_evaluated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_drivers_referenced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_experimentation_gaps: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_justified: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consequences_complete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality_strategy_reviewed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links_populated: Option<bool>,
    #[serde(default)]
    pub validation_needs: String,
    #[serde(default)]
    pub pre_review_notes: String,
}

/// Decision with chosen option index and justification (per ADR-0057 design rule 2).
/// Supports both new format (chosen_option + justification) and legacy format (body).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Decision {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chosen_option: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub justification: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Consequence {
    #[serde(rename = "type")]
    pub kind: String,
    pub body: String,
}

/// Deliverables checklist.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Deliverables {
    #[serde(default)]
    pub items: Vec<DeliverableItem>,
}

/// A single deliverable item.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeliverableItem {
    pub description: String,
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub artifact: String,
}

/// Quality strategy flags — flat checklist per ADR-0051 design rule 10.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QualityStrategy {
    #[serde(default)]
    pub major_semantic_changes: bool,
    #[serde(default)]
    pub minor_semantic_changes: bool,
    #[serde(default)]
    pub fuzz_testing: bool,
    #[serde(default)]
    pub unit_testing: bool,
    #[serde(default)]
    pub load_testing: bool,
    #[serde(default)]
    pub performance_testing: bool,
    #[serde(default)]
    pub backwards_compatible: bool,
    #[serde(default)]
    pub integration_tests: bool,
    #[serde(default)]
    pub tooling: bool,
    #[serde(default)]
    pub user_documentation: bool,
    #[serde(default)]
    pub additional_concerns: String,
}

/// Implementation plan embedded in the ADR.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Plan {
    #[serde(default)]
    pub stages: Vec<PlanStage>,
}

/// A stage in the plan.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanStage {
    pub name: String,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub tasks: Vec<PlanTask>,
}

/// A task within a plan stage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanTask {
    pub description: String,
    #[serde(default)]
    pub done: bool,
}

/// Comments section — draft worksheet and revision entries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comments {
    #[serde(default)]
    pub draft_worksheet: String,
    #[serde(default)]
    pub revision_entries: Vec<RevisionEntry>,
}

/// A revision entry in the comments section.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevisionEntry {
    pub id: String,
    pub date: String,
    pub body: String,
}

/// Validation errors for ADR documents.
#[derive(Debug)]
pub enum ValidationError {
    InvalidStatus(String),
    EmptyAssessment(&'static str),
    MutualExclusivity(String),
    InvalidWorkItem(String),
    InvalidOptionIndex { chosen: usize, count: usize },
    DeserializationError { path: String, detail: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidStatus(s) => {
                write!(f, "invalid status '{}': expected one of {:?}", s, VALID_STATUSES)
            }
            ValidationError::EmptyAssessment(which) => {
                write!(f, "{} checkpoint assessment is empty — checkpoints are mandatory", which)
            }
            ValidationError::MutualExclusivity(msg) => write!(f, "{}", msg),
            ValidationError::InvalidWorkItem(s) => {
                write!(f, "invalid work_item '{}': expected '{{remote}}#{{id}}' format", s)
            }
            ValidationError::InvalidOptionIndex { chosen, count } => {
                write!(
                    f,
                    "decision.chosen_option {} is out of bounds — {} option(s) defined",
                    chosen, count
                )
            }
            ValidationError::DeserializationError { path, detail } => {
                write!(f, "failed to parse '{}': {}", path, detail)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

impl Adr {
    /// Validate the ADR against schema rules.
    pub fn validate(&self) -> Result<(), Vec<ValidationError>> {
        let mut errors = Vec::new();

        // Status must be a known value
        if !VALID_STATUSES.contains(&self.meta.status.as_str()) {
            errors.push(ValidationError::InvalidStatus(self.meta.status.clone()));
        }

        // Checkpoints are mandatory — assessment must not be empty
        if self.evaluation_checkpoint.assessment.trim().is_empty() {
            errors.push(ValidationError::EmptyAssessment("Evaluation"));
        }
        if self.conclusion_checkpoint.assessment.trim().is_empty() {
            errors.push(ValidationError::EmptyAssessment("Conclusion"));
        }

        // Mutual exclusivity: major and minor semantic changes
        if self.quality_strategy.major_semantic_changes
            && self.quality_strategy.minor_semantic_changes
        {
            errors.push(ValidationError::MutualExclusivity(
                "major_semantic_changes and minor_semantic_changes cannot both be true".to_string(),
            ));
        }

        // Work item format validation (if non-empty)
        if !self.meta.work_item.is_empty() {
            let parts: Vec<&str> = self.meta.work_item.splitn(2, '#').collect();
            if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
                errors.push(ValidationError::InvalidWorkItem(self.meta.work_item.clone()));
            }
        }

        // Decision: chosen_option must be in bounds (per ADR-0057 design rule 2)
        if let Some(idx) = self.decision.chosen_option {
            if idx >= self.options.len() {
                errors.push(ValidationError::InvalidOptionIndex {
                    chosen: idx,
                    count: self.options.len(),
                });
            }
        }

        // TODO: ADR-0057 design rule 3 — index stability enforcement.
        // Once implemented in adr-format (ADR-0052), reject option reordering
        // on ADRs with status != Prototype.

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

/// Deserialize an ADR from a TOML string with a file path for error context.
pub fn parse_adr(toml_str: &str, file_path: &str) -> Result<Adr, ValidationError> {
    toml::from_str(toml_str).map_err(|e| ValidationError::DeserializationError {
        path: file_path.to_string(),
        detail: e.to_string(),
    })
}

/// Serialize an ADR to a TOML string.
pub fn serialize_adr(adr: &Adr) -> Result<String, String> {
    toml::to_string_pretty(adr).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_adr() -> Adr {
        Adr {
            meta: Meta {
                title: "gh-42. Use PostgreSQL".to_string(),
                date: "2026-04-06".to_string(),
                status: "Prototype".to_string(),
                last_updated: "2026-04-06".to_string(),
                identifier: "gh-42".to_string(),
                work_item: "gh#42".to_string(),
                links: vec!["ADR-0034".to_string()],
            },
            context: Section {
                body: "We need a database.".to_string(),
            },
            options: vec![
                AdrOption {
                    name: "PostgreSQL".to_string(),
                    body: "Mature relational database.".to_string(),
                },
                AdrOption {
                    name: "SQLite".to_string(),
                    body: "Embedded database.".to_string(),
                },
            ],
            evaluation_checkpoint: Checkpoint {
                assessment: "Proceed".to_string(),
                items: vec![
                    CheckpointItem { label: "All options evaluated at comparable depth".to_string(), checked: true },
                    CheckpointItem { label: "Decision drivers are defined and referenced in option analysis".to_string(), checked: true },
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
                justification: Some("We chose PostgreSQL.".to_string()),
                body: None,
            },
            consequences: vec![
                Consequence {
                    kind: "positive".to_string(),
                    body: "Strong JSONB support.".to_string(),
                },
                Consequence {
                    kind: "negative".to_string(),
                    body: "Requires running database process.".to_string(),
                },
                Consequence {
                    kind: "neutral".to_string(),
                    body: "Team has expertise.".to_string(),
                },
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
            comments: Some(Comments {
                draft_worksheet: "**Framing:** Test framing.".to_string(),
                revision_entries: vec![RevisionEntry {
                    id: "R1".to_string(),
                    date: "2026-04-06".to_string(),
                    body: "Addressed review findings.".to_string(),
                }],
            }),
        }
    }

    #[test]
    fn round_trip_serialize_deserialize() {
        let adr = sample_adr();
        let toml_str = serialize_adr(&adr).expect("serialize failed");
        let parsed = parse_adr(&toml_str, "test.toml").expect("parse failed");
        assert_eq!(adr, parsed);
    }

    #[test]
    fn invalid_status_rejected() {
        let mut adr = sample_adr();
        adr.meta.status = "InvalidStatus".to_string();
        let result = adr.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidStatus(_))));
    }

    #[test]
    fn ready_status_valid() {
        let mut adr = sample_adr();
        adr.meta.status = "Ready".to_string();
        assert!(adr.validate().is_ok());
    }

    #[test]
    fn empty_assessment_rejected() {
        let mut adr = sample_adr();
        adr.evaluation_checkpoint.assessment = "".to_string();
        let result = adr.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::EmptyAssessment("Evaluation"))));
    }

    #[test]
    fn mutual_exclusivity_rejected() {
        let mut adr = sample_adr();
        adr.quality_strategy.major_semantic_changes = true;
        adr.quality_strategy.minor_semantic_changes = true;
        let result = adr.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::MutualExclusivity(_))));
    }

    #[test]
    fn valid_adr_passes_validation() {
        let adr = sample_adr();
        assert!(adr.validate().is_ok());
    }

    #[test]
    fn checkpoint_items_schema() {
        let adr = sample_adr();
        assert_eq!(adr.evaluation_checkpoint.items.len(), 3);
        assert!(adr.evaluation_checkpoint.items[0].checked);
        assert_eq!(
            adr.evaluation_checkpoint.items[0].label,
            "All options evaluated at comparable depth"
        );
        assert_eq!(adr.conclusion_checkpoint.items.len(), 4);
        assert!(adr.conclusion_checkpoint.items[0].checked);
    }

    #[test]
    fn revision_entries_schema() {
        let adr = sample_adr();
        let comments = adr.comments.as_ref().unwrap();
        assert_eq!(comments.revision_entries.len(), 1);
        assert_eq!(comments.revision_entries[0].id, "R1");
        assert_eq!(comments.revision_entries[0].date, "2026-04-06");
    }

    #[test]
    fn optional_sections_absent() {
        let mut adr = sample_adr();
        adr.deliverables = None;
        adr.plan = None;
        adr.comments = None;

        let toml_str = serialize_adr(&adr).expect("serialize failed");
        let parsed = parse_adr(&toml_str, "test.toml").expect("parse failed");
        assert_eq!(parsed.deliverables, None);
        assert_eq!(parsed.plan, None);
        assert_eq!(parsed.comments, None);
    }

    #[test]
    fn invalid_work_item_format() {
        let mut adr = sample_adr();
        adr.meta.work_item = "gh42".to_string(); // missing #
        let result = adr.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidWorkItem(_))));
    }

    #[test]
    fn deserialization_error_includes_path() {
        let result = parse_adr("not valid toml {{{", "docs/adr/test.toml");
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("docs/adr/test.toml"), "error should include file path: {}", msg);
    }

    #[test]
    fn work_item_empty_remote_rejected() {
        let mut adr = sample_adr();
        adr.meta.work_item = "#42".to_string();
        assert!(adr.validate().is_err());
    }

    #[test]
    fn work_item_empty_id_rejected() {
        let mut adr = sample_adr();
        adr.meta.work_item = "gh#".to_string();
        assert!(adr.validate().is_err());
    }

    #[test]
    fn work_item_empty_string_valid() {
        let mut adr = sample_adr();
        adr.meta.work_item = String::new();
        assert!(adr.validate().is_ok());
    }

    #[test]
    fn malformed_toml_unclosed_multiline_string() {
        let input = r#"
[meta]
title = """unclosed
"#;
        let result = parse_adr(input, "test-unclosed.toml");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("test-unclosed.toml"));
    }

    #[test]
    fn malformed_toml_bare_equals() {
        let input = "= no key here\n";
        let result = parse_adr(input, "test-bare.toml");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("test-bare.toml"));
    }

    #[test]
    fn malformed_toml_invalid_escape() {
        let input = r#"
[meta]
title = "bad \q escape"
"#;
        let result = parse_adr(input, "test-escape.toml");
        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("test-escape.toml"));
    }

    #[test]
    fn unknown_top_level_field_rejected() {
        let adr = sample_adr();
        let mut toml_str = serialize_adr(&adr).unwrap();
        toml_str.push_str("\n[unknown_section]\nfoo = \"bar\"\n");
        let result = parse_adr(&toml_str, "test-unknown.toml");
        assert!(result.is_err(), "unknown top-level field should be rejected");
    }

    #[test]
    fn backward_compat_old_checkpoint_format() {
        // Old ADR-0051 format with named boolean fields instead of items array
        let input = r#"
[meta]
title = "Test"
date = "2026-04-06"
status = "Proposed"
last_updated = "2026-04-06"

[context]
body = "Test context."

[evaluation_checkpoint]
assessment = "Proceed"
all_options_evaluated = true
decision_drivers_referenced = true
no_experimentation_gaps = true

[decision]
body = "We chose option A."

[quality_strategy]
unit_testing = true

[conclusion_checkpoint]
assessment = "Ready for review"
decision_justified = true
consequences_complete = true
quality_strategy_reviewed = true
links_populated = true
"#;
        let adr = parse_adr(input, "old-format.toml").expect("old format should parse");
        assert_eq!(adr.evaluation_checkpoint.all_options_evaluated, Some(true));
        assert!(adr.evaluation_checkpoint.items.is_empty());
        assert_eq!(adr.decision.body.as_deref(), Some("We chose option A."));
        assert_eq!(adr.decision.chosen_option, None);
    }

    #[test]
    fn backward_compat_old_decision_format() {
        let mut adr = sample_adr();
        adr.decision = Decision {
            chosen_option: None,
            justification: None,
            body: Some("Legacy decision body.".to_string()),
        };
        let toml_str = serialize_adr(&adr).expect("serialize failed");
        let parsed = parse_adr(&toml_str, "test.toml").expect("parse failed");
        assert_eq!(parsed.decision.body.as_deref(), Some("Legacy decision body."));
        assert_eq!(parsed.decision.chosen_option, None);
    }

    #[test]
    fn chosen_option_out_of_bounds() {
        let mut adr = sample_adr();
        adr.decision.chosen_option = Some(99); // only 2 options
        let result = adr.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| matches!(e, ValidationError::InvalidOptionIndex { .. })));
    }

    #[test]
    fn meta_identifier_round_trip() {
        let adr = sample_adr();
        assert_eq!(adr.meta.identifier, "gh-42");
        let toml_str = serialize_adr(&adr).expect("serialize failed");
        let parsed = parse_adr(&toml_str, "test.toml").expect("parse failed");
        assert_eq!(parsed.meta.identifier, "gh-42");
    }
}
