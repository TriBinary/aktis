# Path tracer (`aktis-pt`)

## Purpose

Photoreal, progressive, GPU path tracing of Minecraft scenes, with PBR materials.

## Status

**Planned.** `PathTracer` and `PathTracerSettings` exist as stubs; `render` returns
`NotImplemented`.

## Design (planned)

| Piece | Notes |
|---|---|
| Traversal | Two-level DDA: chunk/brick occupancy, then blocks; then exact ray-vs-model-quad tests for non-full blocks. No hardware RT required (ADR 0001). |
| Integrator | Unidirectional path tracing with next event estimation for sun and emissive blocks; Russian roulette. |
| Materials | Vanilla albedo + LabPBR (roughness from smoothness, F0/metals, normal maps, emission). |
| Sky | Analytic sky model plus optional HDR skymaps. |
| Accumulation | `Rgba32Float` buffer; one or more spp per frame; reset when the scene changes. |
| Denoise | OIDN on final output (M4). |
| Shaders | WGSL in `crates/aktis-pt/shaders/`. |

## Open questions

- Water and glass: refraction model and caustics budget?
- Spectral vs RGB rendering? (Default: RGB.)
