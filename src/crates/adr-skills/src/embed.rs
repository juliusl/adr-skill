use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../../../src/skills/"]
pub struct SkillAssets;

#[derive(Embed)]
#[folder = "../../../src/agents/"]
pub struct AgentAssets;
