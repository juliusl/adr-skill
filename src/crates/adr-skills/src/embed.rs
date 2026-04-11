use rust_embed::Embed;

/// Embedded skill definitions from `src/skills/`.
#[derive(Embed)]
#[folder = "../../../src/skills/"]
pub struct SkillAssets;

/// Embedded agent definitions from `src/agents/`.
#[derive(Embed)]
#[folder = "../../../src/agents/"]
pub struct AgentAssets;
