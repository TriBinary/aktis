# CI and releases

## Workflows (`.github/workflows/`)

| Workflow | Trigger | What it does |
|---|---|---|
| `ci.yml` | push to `main`, every PR | `fmt --check`, `clippy -D warnings`, `test` on **macOS (arm64)** and **Windows (x64)**; docs check on PRs |
| `release.yml` | push of tag `v*.*.*` (**upstream only**) | Builds release binaries, packages them, creates a GitHub Release with notes taken from `CHANGELOG.md` |

Forks run CI too (useful before opening a PR), but `release.yml` is skipped unless
`github.repository == 'TriBinary/aktis'`.

### Docs check

On pull requests, CI fails if files under `crates/` or `Cargo.toml` changed and nothing under
`docs/`, `CHANGELOG.md`, `README.md` or `CLAUDE.md` changed. To bypass it (e.g. pure internal
refactor), add the `no-docs` label to the PR.

## Release artifacts

| Platform | Target | File |
|---|---|---|
| macOS Apple Silicon | `aarch64-apple-darwin` | `aktis-<version>-macos-arm64.tar.gz` |
| macOS Intel | `x86_64-apple-darwin` | `aktis-<version>-macos-x64.tar.gz` |
| Windows x64 | `x86_64-pc-windows-msvc` | `aktis-<version>-windows-x64.zip` |

Each archive contains the binary plus `README.md`, `CHANGELOG.md` and the licences. A
`SHA256SUMS.txt` file is attached as well. Binaries are **not signed or notarized yet**. On
macOS, users must right-click → Open (or `xattr -d com.apple.quarantine aktis`). Signing,
notarization and a `.app` bundle are planned.

## How to release

1. On a branch: bump `version` in the root `Cargo.toml` (`[workspace.package]`), move the
   `## [Unreleased]` entries in `CHANGELOG.md` under `## [X.Y.Z] - YYYY-MM-DD`, and merge via PR.
2. Tag the merge commit on upstream and push the tag:
   ```sh
   git fetch upstream
   git tag -a vX.Y.Z upstream/main -m "Aktis vX.Y.Z"
   git push upstream vX.Y.Z
   ```
3. `release.yml` checks that the tag matches the Cargo version, builds, and publishes the
   release. Tags containing `-` (e.g. `v0.2.0-beta.1`) are marked as pre-releases.

## Dependabot

`.github/dependabot.yml` opens weekly PRs for Cargo and GitHub Actions updates. Remember that
`wgpu` and `naga` must stay on the major version `eframe` uses.
