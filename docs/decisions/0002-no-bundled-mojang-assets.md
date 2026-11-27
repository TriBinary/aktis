# 0002: Never bundle Mojang assets

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

Minecraft textures, models and code belong to Mojang/Microsoft and may not be redistributed.

## Decision

Aktis reads vanilla assets at runtime from the user's own Minecraft installation (the client
jar). The repository, releases and test fixtures contain no Mojang assets. Fixtures are
self-made.

## Consequences

- Users must have Minecraft Java Edition installed (or point Aktis at a client jar).
- Tests that need real assets are opt-in, using an environment variable that points to a local
  jar (see `docs/dev/testing.md`).

## Alternatives considered

- Downloading assets from Mojang's servers at first launch: possible later, if allowed, but
  still never committed.
