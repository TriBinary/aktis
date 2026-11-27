# CLAUDE.md — Agent guide for Aktis

Aktis is a GPU-accelerated Minecraft (Java Edition) render studio, written in Rust. It has two
render engines: a **path tracer** (photoreal, PBR) and a **shader-pack engine** (OptiFine/Iris
packs such as BSL or Complementary). It targets **macOS (Apple Silicon, Metal)** and
**Windows (DX12/Vulkan)**.

Read this file fully before doing anything. Then read `docs/README.md`, plus the subsystem doc
for whatever you are about to touch.

---

## 1. Hard rules

1. **Never commit, push, tag, merge or rebase.** The maintainer reviews every change. Leave all
   changes uncommitted in the working tree. Read-only git commands (`status`, `diff`, `log`)
   are fine.
2. **Every change updates the documentation in the same task.** No exceptions. Section 4 says
   which docs to update. A task is not done until the docs match the code.
3. **End every task with a commit message** (section 5). If the change should be split into
   several commits, give one message per commit and list which files go in each.
4. **Ask before you build** when anything about the idea is unclear (section 2).
5. **Never add Mojang assets to the repository.** No textures, models, sounds, client jars or
   decompiled code. Vanilla assets are always read at runtime from the user's own Minecraft
   installation. Test fixtures must be self-made (handmade JSON/PNG, tiny generated worlds).
6. **Don't copy code from GPL or LGPL projects** (Chunky is GPL-3.0, Iris is LGPL-3.0). Reading
   them to understand behaviour is fine. Re-implement from that understanding and note the
   reference in the doc.
7. **Both platforms must keep working.** Don't use a wgpu feature or OS API that is
   unavailable on Metal or on DX12/Vulkan without a fallback. Check
   `docs/architecture/overview.md` → "Platform rules".

## 2. Clarify first

Before starting any non-trivial task (new feature, new crate, public API change, UI change, new
dependency, file-format change):

- Restate the goal in one or two sentences.
- Ask the questions whose answers would change the design. Number them and keep them short.
  For each, give the default you'd pick, so the maintainer can just answer "defaults".
- Wait for answers before writing code.

Don't ask for trivial fixes (typos, obvious bugs, a failing lint) or when the task already
answers the question. If new ambiguity shows up mid-task, stop and ask; don't guess silently.

## 3. Workflow for each task

1. Read the relevant docs (`docs/README.md` has the map).
2. Clarify (section 2).
3. Make the change. Keep it focused, with no drive-by refactors unless asked.
4. Run the checks:
   ```sh
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
5. Update the docs and `CHANGELOG.md` (section 4).
6. Report: what changed, how you verified it, any open questions or follow-ups, and the commit
   message(s).

## 4. Documentation system

All docs live in `docs/`; `docs/README.md` is the index. **Update docs as part of the change,
never "later".**

| You changed… | Update |
|---|---|
| Anything a user or contributor would notice | `CHANGELOG.md` → `## [Unreleased]` |
| A crate's behaviour, API or internals | `docs/subsystems/<subsystem>.md` (Status + Design sections) |
| Crate added, removed, renamed, or dependency direction changed | `docs/architecture/crates.md` and `docs/architecture/overview.md` |
| A dependency added, removed or majorly bumped | `docs/architecture/dependencies.md` |
| A significant or hard-to-reverse design choice | New ADR in `docs/decisions/` (copy `0000-template.md`) and list it in `docs/decisions/README.md` |
| A milestone task started or finished | `docs/roadmap.md` (status column) |
| Build steps, tools or environment | `docs/dev/getting-started.md` |
| Tests or test fixtures | `docs/dev/testing.md` |
| CI workflows or the release process | `docs/dev/ci-and-release.md` |
| A new term | `docs/glossary.md` |
| A scene or project file format | `docs/subsystems/scene.md` → "File format" |

Doc rules:
- Docs describe what **is**, not what you wish. Mark unbuilt parts as `Planned`.
- Every subsystem doc keeps a **Status** line (`Planned` / `In progress` / `Working` /
  `Stable`) and an **Open questions** list.
- Rust public items need `///` doc comments (the workspace lint `missing_docs` enforces this).
- Keep docs short and concrete; prefer tables and examples to prose.
- If code and docs disagree, fix whichever is wrong and say so in your report.

CI fails a pull request that changes code without touching `docs/` or `CHANGELOG.md`, unless
the PR has the `no-docs` label.

## 5. Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <summary in imperative mood, ≤ 72 chars>

<body: what and why, wrapped at 72 chars. Mention docs updated.>

<footer: e.g. Refs #12, BREAKING CHANGE: …>
```

- **Types:** `feat`, `fix`, `perf`, `refactor`, `docs`, `test`, `build`, `ci`, `chore`
- **Scopes:** `assets`, `world`, `scene`, `render`, `pt`, `sp`, `app`, `docs`, `ci`, `deps`

Present the message(s) in a fenced code block at the end of your report, ready to copy.

## 6. Repository layout

```
Cargo.toml              workspace; ALL dependency versions live in [workspace.dependencies]
crates/
  aktis-assets/         client jar + resource packs, blockstates/models, LabPBR textures
  aktis-world/          save folders, region/NBT, chunks, block states, selection
  aktis-scene/          camera/sun/sky/fog: plain serializable data (TOML)
  aktis-render/         shared wgpu context, RenderEngine trait, world upload, readback
  aktis-pt/             path-tracing engine (compute shaders)
  aktis-sp/             shader-pack engine (OptiFine/Iris pipeline)
  aktis-app/            egui desktop app, binary name `aktis`
docs/                   documentation system (index: docs/README.md)
.github/                CI, release workflow, templates
```

Dependency direction (no cycles, no shortcuts):
`app → {pt, sp} → render → {scene, world} → assets`. The app may depend on any crate. The
engines never depend on each other.

## 7. Code conventions

- Rust 2024 edition, stable toolchain, MSRV in `Cargo.toml`.
- Add dependencies **only** through `[workspace.dependencies]`, then `dep.workspace = true` in
  the crate. Record them in `docs/architecture/dependencies.md`.
- Library crates use `thiserror` error enums; only `aktis-app` uses `anyhow`.
- Log with `log` macros; no `println!` in library crates.
- WGSL shaders go in `crates/<crate>/shaders/*.wgsl` and are loaded with `include_str!`.
- No `unsafe` without a `// SAFETY:` comment and a mention in the report.
- Unit tests live next to the code; cross-crate tests go in `crates/<crate>/tests/`.
- GPU tests that need a real adapter must skip gracefully when none is available (CI has no GPU).

## 8. Git setup (for context only; never run write commands)

- `upstream` = `github.com/TriBinary/aktis` (main repository)
- `origin` = the maintainer's fork, where work is pushed and PRs come from
- Releases are made by pushing a `vX.Y.Z` tag to upstream (see `docs/dev/ci-and-release.md`).
