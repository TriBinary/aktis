# 0005: Docs-with-every-change and review-before-commit workflow

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

Aktis is built largely by AI coding agents across many sessions. Each session starts without
memory of earlier ones, so the docs are the project's memory. The maintainer reviews all
changes before they enter history.

## Decision

- Every change updates the relevant docs and `CHANGELOG.md` in the same change (`CLAUDE.md` §4).
- Agents ask clarifying questions before non-trivial work.
- Agents never commit; they end each task with Conventional Commit message(s) for the
  maintainer.
- CI enforces "code change ⇒ docs/CHANGELOG change" on pull requests (bypass label: `no-docs`).

## Consequences

- Slightly slower tasks; much better continuity between sessions.
- The docs check can be bypassed for pure refactors with the label.

## Alternatives considered

- Docs written at milestone ends: they drift, and agents lose context.
