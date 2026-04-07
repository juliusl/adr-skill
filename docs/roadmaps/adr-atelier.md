# adr-atelier Roadmap

This document provides a roadmap of milestones for `adr-atelier`.

`adr-atelier` will be a Rust based tool that can explore, manage, and orchestrate adr based development based on adr-db-lib and the high-level agent skills.

## Features
- Based on a new agent forward adr template "wi-full-agent-adr" format
- Explore and Navigate ADR's stored in each project w/ a wiki-like experience, where referenced ADR's and items can be navigated to, and the review/revise dialouge chain is transparent
- Manage ADR workflows by updating status's when checkpoints are met, and kick off orchestration sessions w/ the /solve-adr skill by accessing the local ACP server
- Review implementation history by viewing task status, qa findings, and commit diffs
- Configure project preferences from a single portal

## Constraints
- Single binary "adr-atelier" that launches the server and handles all data fetching tasks
- Binary should use "cap-std" for safe file handling
- Binary should be cautious about user-input to defend against prompt injection and database injection to be secure from the ground up
- The tool should be able to handle multiple project directories since each adr.db is stored in each directory
- Integration w/ the local agent should be done via ACP (agent client protocol) using the `agent-client-protocol` crate

## Milestones

Each milestone is a group of high-level goals, but do not completely describe, design, or explore the problem, however changes to items should be iterative or provide a clear benefit over the original item.
If an item has a gap, then it should be clear signal that an additional ADR is needed to cover the gap.

This is meant to be a living document and will be adapt to reflect new findings, constraints, or requirements.

### Milestone 1 <!-- status: complete -->
- Add "wi-full-agent-adr" format which replaces "wi-nygard-agent-adr" format, See "Work-Item based full-agent ADR Format (wi-full-agent-adr)" section for details

### Milestone 2
- Enable additional adr-db tables to store new data
- Enable strong semantic-versioning discipline in skills, w/ crate version of adr-db-lib as a +<adr-db-lib> addition to the skill version. Keeps skills aware of the lib version of tooling while decoupling the versioning of the skill itself

### Milestone 3
- (Prereq) See "UI Framework Selection" below - this should be passed to `/author-adr` to record this decision
- Initial implementation of rust project that enables user/project preference management and db item browsing
 - Should also be able to configure draft preferences from the tool
- UX should be simple enough for users to understand what a preference does
- Skills should auto-detect adr-atelier enablement and keep adr-db in sync as part of their workflows
- An initial sync gesture should exist in order to initialize the tool from existing directories, could be in the form of a templated prompt that the solve-adr skill understands
  - Having the crates from this repo installed is a pre-requisite for adr-atelier to work w/ the adr-skill set. Skills should assist with meeting this pre-req

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

# UI Framework Selection

Several Rust UI frameworks were considered for adr-atelier. The primary constraints driving the decision were the single-binary deployment model, the dev-tool nature of the product, and the need for graph and tree visualization in later milestones.

### Candidates

**Leptos** was the initial candidate. It is a reactive, signal-based web framework with SSR support, but it works against the single-binary constraint — even with SSR, serving a Leptos app requires asset pipelines, hydration, and a web server, which adds complexity that doesn't belong in a local dev tool.

**Yew** is a React-inspired component framework targeting WASM. It shares Leptos's fundamental problem: it is designed for browser delivery, so a native binary is not a natural target. It also relies on JS interop for anything outside of rendering, and its ecosystem for data visualization and graph editors is thin compared to the alternatives.

**egui** (via eframe) was selected. It is an immediate-mode GUI framework that compiles to a native binary with no runtime dependencies, no asset serving, and no JS involvement. eframe also targets `wasm32-unknown-unknown` with the same codebase, leaving the door open for a browser-hosted version later. The immediate-mode model is well-suited to a tool that surfaces live data from `adr-db` and responds to agent state changes.

### Why egui fits adr-atelier

- The single binary constraint is a first-class eframe use case — the binary launches, renders, and exits with no sidecar processes beyond the ACP server it manages itself.
- The wiki-style ADR navigation, roadmap visualization (Milestone 4), and agent participation graph (Milestone 5) all have direct egui ecosystem support via `egui_node_graph2`, `egui-snarl`, and `egui_graphs` without reaching outside the Rust ecosystem.
- The utilitarian aesthetic of egui is appropriate for a developer-facing tool where information density and clarity matter more than visual polish.
- `cap-std` for safe file handling and async ACP integration are both straightforward to wire into eframe's native execution model.

### Trade-offs accepted

egui's immediate-mode model requires more explicit state management discipline than a reactive framework like Leptos or Yew, where signals and components handle invalidation automatically. For a tool with complex nested state (multiple project directories, live agent sessions, ACP message queues), this means investing in a clean top-level state struct and being deliberate about re-render boundaries. This is a known cost and considered acceptable given the benefits above.