# Contributing to Aktis

Thanks for helping! The short version:

1. Fork `TriBinary/aktis` and branch from `main`.
2. Make your change **and update the docs** in the same PR (the table in `CLAUDE.md` §4 shows
   which file to update). Add an entry under `## [Unreleased]` in `CHANGELOG.md`.
3. Run:
   ```sh
   cargo fmt --all
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. Use [Conventional Commits](https://www.conventionalcommits.org/), e.g.
   `feat(pt): add next event estimation for the sun`.
5. Open a PR. CI must pass, including the docs check.

## Ground rules

- No Mojang assets in the repository; test fixtures must be self-made.
- Don't copy code from GPL/LGPL projects (Chunky, Iris); use them as references only.
- Keep macOS and Windows working.
- Discuss big changes in an issue first; significant decisions get an ADR (`docs/decisions/`).

By contributing, you agree that your contributions are dual-licensed under MIT and
Apache-2.0, as described in the README.

Full details: [docs/dev/workflow.md](docs/dev/workflow.md).
