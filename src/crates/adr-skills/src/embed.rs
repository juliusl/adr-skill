//! Embedded assets — skill and agent files compiled into the binary via `rust_embed`.

use rust_embed::Embed;

/// Embedded skill definitions from `src/skills/`.
#[derive(Embed)]
#[folder = "../../../src/skills/"]
pub struct SkillAssets;

/// Embedded agent definitions from `src/agents/`.
#[derive(Embed)]
#[folder = "../../../src/agents/"]
pub struct AgentAssets;
