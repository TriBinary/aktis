# Testing

## Layout

- **Unit tests:** `#[cfg(test)] mod tests` next to the code.
- **Integration tests:** `crates/<crate>/tests/*.rs`.
- **Fixtures:** `crates/<crate>/tests/fixtures/`, **self-made only** (ADR 0002).

## Real-asset tests (opt-in)

Tests that need a real client jar or world read paths from environment variables and skip
when they are unset:

| Variable | Meaning |
|---|---|
| `AKTIS_TEST_CLIENT_JAR` | Path to a Minecraft client jar |
| `AKTIS_TEST_WORLD` | Path to a save folder |

These never run in CI.

## GPU tests

CI runners have no usable GPU. GPU tests must request an adapter and **return early (pass) if
none is found**, logging that they were skipped.

## Current tests

| Crate | What's covered |
|---|---|
| aktis-assets | Resource location parsing/paths; asset stack priority |
| aktis-scene | TOML round-trip; defaults for missing fields |
