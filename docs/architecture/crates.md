# Crates

| Crate | Kind | Role | Depends on (internal) | Status |
|---|---|---|---|---|
| `aktis-assets` | lib | Client jar + resource packs; blockstates, models, textures, LabPBR | — | In progress (types only) |
| `aktis-world` | lib | Save folders, region/NBT, chunks, block states, selection | assets | In progress (types only) |
| `aktis-scene` | lib | Camera, sun, sky, fog; TOML scene files | — | Working (data + TOML) |
| `aktis-render` | lib | wgpu context, `RenderEngine` trait, world upload, readback | scene, world | In progress (trait only) |
| `aktis-pt` | lib | Path-tracing engine | render, scene, world, assets | Planned (stub) |
| `aktis-sp` | lib | Shader-pack engine | render, scene, world, assets | Planned (stub) |
| `aktis-app` | bin `aktis` | egui desktop app | all | In progress (window shell) |

## Rules

- Engines (`aktis-pt`, `aktis-sp`) never depend on each other. Shared GPU code belongs in
  `aktis-render`.
- Only `aktis-app` may depend on `eframe`/`egui`.
- Only `aktis-render` and the engines may depend on `wgpu`.
- New crates need an ADR and an entry here.
