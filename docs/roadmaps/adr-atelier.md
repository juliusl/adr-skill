# adr-atelier roadmap

A Rust based local browser based tool that can explore, manage, and orchestrate adr based development based on adr-db-lib and the high-level agent skills. Integrates w/ the local agent via acp (agent client protocol) via the agent-client-protocol crate. UI and visualization is based on Lepto and using adr-db-lib to access local data.

The tool should be able to handle multiple project directories since each adr.db is stored in each directory.

## Features
- Explore and Navigate ADR's stored in each project w/ a wiki-like experience, where referenced ADR's and items can be navigated to, and the review/revise dialouge chain is transparent
- Manage ADR workflows by updating status's when checkpoints are met, and kick off orchestration sessions w/ the /solve-adr skill by accessing the local ACP server
- Review implementation history by viewing task status, qa findings, and commit diffs
- Configure project preferences from a single portal
- Based on a new agent forward adr template "wi-full-agent-adr" format

## Constraints
- Single binary "adr-atelier" that launches the server and handles all data fetching tasks
- Binary should use "cap-std" for safe file handling
- Binary should be cautious about user-input to defend against prompt injection and database injection to be secure from the ground up

## Roadmap

This roadmap is a list of milestones. With each milestone are groups of user-stories and objectives that satisfy the milestone.

Each item under a milestone are high-level goals, but do not completely describe, design, or explore the problem, however changes to items should be iterative or provide a clear benefit over the original item.
If an item has a gap, then it should be clear signal that an additional ADR is needed to cover the gap.

### Milestone 1
- Add "wi-full-agent-adr" format which replaces "wi-nygard-agent-adr" format, See "wi-full-agent-adr" section for details

### Milestone 2
- Enable additional adr-db tables to store new data
- Enable strong semantic-versioning discipline in skills, w/ crate version of adr-db-lib as a +<adr-db-lib> addition to the skill version. Keeps skills aware of the lib version of tooling while decoupling the versioning of the skill itself

### Milestone 3
- Skills should auto-detect adr-atelier enablement and keep adr-db in sync as part of their workflows
- Initial implementation of rust project that enables user/project preference management and db item browsing
 - Should also be able to configure draft preferences from the tool
- UX should be simple enough for users to understand what a preference does

### Milestone 4
- Enable local-gitea integration in adr-atelier, view and create issues and pr's directly from the tool w/ gitea remotes
- View and interact w/ "roadmaps" from the tool (Using this roadmap as a model)
  - Roadmap should be a first-class item stored in adr-db
  - User should be able to see current milestone progress

### Milestone 5
- Enable ACP integration and orchestration kick off
- Enable new "atelier" participation mode, 
  - Enable agents to write open questions to adr-db that the user may respond to
  - Allow commenting and reviews for sections of the adr template
- Enable one-click solve-adr orchestration kick off directly from road-map, tool should format the new prompt using the milestone and roadmap and provide any supporting information

### Milestone 7
- Package adr-atelier and enable installing tools and skills from the package, default to "wi-full-agent-adr" format and "atelier" participation modes as defaults
  - On a fresh or non-fresh machine, this should be able to be configured by the package
- Use signing or other provenance mechanisms on installed skill documents as well as any documents ingested by adr-db

### Milestone 8
- Enable persona generation for interaction and authoring hooks by analyzing local turn data

--- 

# Work-Item based full-agent ADR Format (wi-full-agent-adr)

After several rounds of implementation w/ nygard, nygard-agent, and wi-nygard-agent format a couple of gaps and key areas of improvement were identified, which drove the design of this new format. Key improvements are:

- Uses TOML instead of Markdown as the markup language, this provides a user-friendly markup language which also has strongly-typed deserialization libraries. This bridges the gap between tooling and removes the need for fuzzy processing of the current markdown format. The key observation that drove this was noticing that agents began to drift while writing procedural reports after completeing work even though a template was in place.
- Checkpoints are no longer optional. The previous format allowed optional checkpoints and the main observation was that the agent sessions were conservative about doing extra worked and had a strong bias to skip steps. By making checkpoints mandatory this supports the "full" part of this format and makes it more appropriate for an automated adr based workflow.
- All components of the ADR are strongly defined to make the workflow procedural.
- Enables storing plan and status updates in a single file instead of using multiple sparse files and task conventions
- Enables a straight-forward conversion to Markdown which enables optional storing of ADR's as documents without cluttering the document w/ mutable data