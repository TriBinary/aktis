# Changelog

All notable changes to Aktis are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project
uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Cargo workspace with crates `aktis-assets`, `aktis-world`, `aktis-scene`, `aktis-render`,
  `aktis-pt`, `aktis-sp` and `aktis-app`.
- `RenderEngine` trait with stub Path Tracer and Shader Pack engines.
- Scene data (camera, sun, sky, fog) with a TOML format.
- Resource location parsing and resource-pack priority stack.
- egui application shell (`aktis`).
- Documentation system (`docs/`), `CLAUDE.md` agent guide, and ADRs 0001–0005.
- GitHub Actions: CI on macOS and Windows, docs check, tag-based releases; Dependabot.
