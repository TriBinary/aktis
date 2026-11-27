# Aktis documentation

This is the entry point for all project docs. Humans and agents both keep these files current:
**every code change updates the docs in the same change** (see `CLAUDE.md` §4 for the
change → doc table).

## Map

| Area | Document | What's in it |
|---|---|---|
| Project | [vision.md](vision.md) | What Aktis is, goals, non-goals |
| | [roadmap.md](roadmap.md) | Milestones and their status |
| | [glossary.md](glossary.md) | Terms used across the codebase |
| Architecture | [architecture/overview.md](architecture/overview.md) | Layers, data flow, platform rules |
| | [architecture/crates.md](architecture/crates.md) | Each crate: role, dependencies, status |
| | [architecture/dependencies.md](architecture/dependencies.md) | Every external crate and why |
| Subsystems | [subsystems/assets.md](subsystems/assets.md) | Client jar, resource packs, models, PBR |
| | [subsystems/world.md](subsystems/world.md) | Save folders, regions, chunks |
| | [subsystems/scene.md](subsystems/scene.md) | Scene data and file format |
| | [subsystems/render-core.md](subsystems/render-core.md) | GPU context, engine trait, upload |
| | [subsystems/path-tracer.md](subsystems/path-tracer.md) | Path-tracing engine |
| | [subsystems/shader-packs.md](subsystems/shader-packs.md) | Shader-pack engine |
| | [subsystems/app.md](subsystems/app.md) | Desktop UI |
| Decisions | [decisions/README.md](decisions/README.md) | Architecture Decision Records (ADRs) |
| Development | [dev/getting-started.md](dev/getting-started.md) | Toolchain, build, run |
| | [dev/workflow.md](dev/workflow.md) | Fork workflow, commits, reviews |
| | [dev/testing.md](dev/testing.md) | Test layout and fixtures |
| | [dev/ci-and-release.md](dev/ci-and-release.md) | GitHub Actions and releasing |

## Conventions

- **Status words:** `Planned` → `In progress` → `Working` → `Stable`.
- Every subsystem doc has the same sections: *Purpose*, *Status*, *Design*, *Open questions*.
- Describe what exists. Unbuilt designs are labelled `Planned`.
- Significant decisions get an ADR; docs link to the ADR rather than repeating the reasoning.
