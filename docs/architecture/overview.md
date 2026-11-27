# Architecture overview

## Layers

```
                ┌───────────────────────────────┐
                │           aktis-app           │  egui UI, project files, render queue
                └───────┬───────────────┬───────┘
                        │               │
              ┌─────────▼──────┐ ┌──────▼─────────┐
              │    aktis-pt    │ │    aktis-sp    │  the two engines (never depend on each other)
              │  path tracer   │ │  shader packs  │
              └─────────┬──────┘ └──────┬─────────┘
                        │               │
                ┌───────▼───────────────▼───────┐
                │         aktis-render          │  wgpu context, RenderEngine trait, upload, readback
                └───────┬───────────────┬───────┘
                        │               │
              ┌─────────▼──────┐ ┌──────▼─────────┐
              │  aktis-scene   │ │  aktis-world   │  scene data · world data
              └────────────────┘ └──────┬─────────┘
                                        │
                                ┌───────▼────────┐
                                │  aktis-assets  │  jar + packs → models, textures, PBR
                                └────────────────┘
```

Dependency direction is strictly downward. Crate details: [crates.md](crates.md).

## Data flow (stills)

1. **Assets:** the user points Aktis at their Minecraft install; `aktis-assets` builds an
   asset stack (jar + resource packs) and resolves blockstates → models → textures.
2. **World:** `aktis-world` opens a save and reads the chunks inside the selection.
3. **Scene:** `aktis-scene` holds camera, sun, sky and fog (saved as TOML).
4. **Upload:** `aktis-render` turns world + assets into GPU buffers that both engines share
   (voxel grid and texture atlas for the path tracer; meshes for rasterization).
5. **Render:** the active engine renders into a texture shown in the egui viewport. The path
   tracer accumulates samples progressively.
6. **Export:** `aktis-render` reads the image back; the app saves it (PNG first).

Animation (M7) will add a timeline that produces a `Scene` per frame and loops steps 5–6. No
other layer needs to change; that's why scene values are plain data.

## Platform rules

| Rule | Why |
|---|---|
| Use wgpu only; no direct Metal/D3D/Vulkan calls | One codebase for both platforms |
| Check `adapter.features()` / `limits()` before using optional features, and provide a fallback | Metal and DX12 differ (e.g. binding limits, hardware ray queries) |
| No hardware ray-tracing requirement | Voxel traversal works on every GPU; hardware RT may be an optional speed-up later |
| Keep shader-pack translation backend-neutral (GLSL → SPIR-V → naga) | Same pipeline on Metal and DX12 |
| Paths via `std::path` + `directories`; never hard-code `/` or `\` | Windows and macOS file layouts differ |
| Find the Minecraft folder per OS (`~/Library/Application Support/minecraft`, `%APPDATA%\.minecraft`) | Default asset discovery |

## Key decisions

See [../decisions/README.md](../decisions/README.md).
