use serde::{Deserialize, Serialize};
use std::fmt;

/// Valid ADR status values.
const VALID_STATUSES: &[&str] = &[
    "Prototype",
    "Proposed",
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
    pub decision: Section,
    #[serde(default)]
    pub consequences: Vec<Consequence>,
    pub deliverables: Option<Deliverables>,
    pub quality_strategy: QualityStrategy,
    pub conclusion_checkpoint: Checkpoint,
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

/// Checkpoint with mandatory assessment and checklist items.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Checkpoint {
    pub assessment: String,
    #[serde(default)]
    pub all_options_evaluated: bool,
    #[serde(default)]
    pub decision_drivers_referenced: bool,
    #[serde(default)]
    pub no_experimentation_gaps: bool,
    #[serde(default)]
    pub decision_justified: bool,
    #[serde(default)]
    pub consequences_complete: bool,
    #[serde(default)]
    pub quality_strategy_reviewed: bool,
    #[serde(default)]
    pub links_populated: bool,
    #[serde(default)]
    pub validation_needs: String,
    #[serde(default)]
    pub pre_review_notes: String,
}

/// A consequence with a type tag (positive, negative, neutral).
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
                all_options_evaluated: true,
                decision_drivers_referenced: true,
                no_experimentation_gaps: true,
                decision_justified: false,
                consequences_complete: false,
                quality_strategy_reviewed: false,
                links_populated: false,
                validation_needs: String::new(),
                pre_review_notes: String::new(),
            },
            decision: Section {
                body: "We chose PostgreSQL.".to_string(),
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
            deliverables: Some(Deliverables {
                items: vec![DeliverableItem {
                    description: "Schema migration".to_string(),
                    done: false,
                    artifact: String::new(),
                }],
            }),
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
                all_options_evaluated: false,
                decision_drivers_referenced: false,
                no_experimentation_gaps: false,
                decision_justified: true,
                consequences_complete: true,
                quality_strategy_reviewed: true,
                links_populated: true,
                validation_needs: String::new(),
                pre_review_notes: String::new(),
            },
            plan: Some(Plan {
                stages: vec![PlanStage {
                    name: "schema-definition".to_string(),
                    status: "done".to_string(),
                    tasks: vec![
                        PlanTask {
                            description: "Define section tables".to_string(),
                            done: true,
                        },
                        PlanTask {
                            description: "Validate round-trip".to_string(),
                            done: false,
                        },
                    ],
                }],
            }),
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
    fn plan_stage_schema() {
        let adr = sample_adr();
        let plan = adr.plan.as_ref().unwrap();
        assert_eq!(plan.stages.len(), 1);
        assert_eq!(plan.stages[0].name, "schema-definition");
        assert_eq!(plan.stages[0].status, "done");
        assert_eq!(plan.stages[0].tasks.len(), 2);
        assert!(plan.stages[0].tasks[0].done);
        assert!(!plan.stages[0].tasks[1].done);
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
}
