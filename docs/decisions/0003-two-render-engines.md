# 0003: Two engines: path tracer and shader-pack engine

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

Users want both photoreal path tracing (like Chunky) and the look of popular OptiFine/Iris
shader packs. Shader packs are complete real-time rasterization pipelines, not materials, so
they can't run inside a path tracer.

## Decision

Two separate engines implement a shared `RenderEngine` trait:
`aktis-pt` (path tracer) and `aktis-sp` (shader-pack pipeline). They share world, asset and
scene data through `aktis-render`, and never depend on each other.

## Consequences

- The same scene/camera can be rendered by either engine.
- More code to maintain; the trait must suit both progressive and frame-based rendering.
- The path tracer ships first (M2–M4); shader packs follow (M5–M6).

## Alternatives considered

- Path tracer only, with LabPBR: simpler, but doesn't meet the shader-pack goal.
- Running real Minecraft + Iris (e.g. via a Fabric mod) as the shader backend: kept as a
  fallback idea (see `docs/subsystems/shader-packs.md`).
