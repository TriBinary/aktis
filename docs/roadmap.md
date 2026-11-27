# Roadmap

Status: `Planned` · `In progress` · `Done`. Update this file whenever a milestone task starts or
finishes.

| # | Milestone | Scope | Status |
|---|---|---|---|
| M0 | Scaffold | Workspace, crate skeletons, docs system, CI and release workflows | Done |
| M1 | Viewer | Load vanilla jar + small world; rasterized fly-through preview in the app | Planned |
| M2 | Path tracer | Progressive compute-shader path tracing; sun and sky; PNG export | Planned |
| M3 | PBR & packs | Resource-pack stacking UI; LabPBR normal/specular maps in the path tracer | Planned |
| M4 | Polish | Denoising (OIDN), water, fog, emissive blocks, block light, entities | Planned |
| M5 | Shader packs MVP | Parse pack, GLSL → SPIR-V → native, gbuffers/composite/final; BSL renders | Planned |
| M6 | Shader-pack compatibility | Shadow pass, custom textures, more built-in inputs; Complementary renders | Planned |
| M7 | Animation | Keyframe timeline, frame sequences, video via ffmpeg (to be designed later) | Planned |
| M8 | Big worlds | Chunk streaming, lower detail for far terrain, memory limits | Planned |

## M0: Scaffold (done)

- [x] Cargo workspace with seven crates and workspace-level dependency versions
- [x] Engine trait (`aktis-render::RenderEngine`) with stub engines
- [x] Scene data with TOML round-trip
- [x] egui app shell opening a window
- [x] Documentation system and `CLAUDE.md`
- [x] CI (fmt, clippy, test, docs check) on macOS and Windows; tag-based release workflow

## Next up: M1

Tasks are detailed in a later session. Expect: jar discovery, blockstate/model resolution,
region reading, meshing, and a wgpu viewport inside egui.
