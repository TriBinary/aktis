# Shader packs (`aktis-sp`)

## Purpose

Render Aktis scenes with existing OptiFine/Iris shader packs (BSL, Complementary, …) by
re-creating their pipeline outside Minecraft.

## Status

**Planned.** `ShaderPackEngine` and `ShaderPackSource` stubs exist. Work starts at M5.

## Design (planned)

### Pipeline to emulate

| Stage group | Programs | Notes |
|---|---|---|
| Shadow | `shadow` | Renders the shadow map(s) from the sun's view. |
| Geometry | `gbuffers_terrain`, `gbuffers_water`, `gbuffers_entities`, `gbuffers_skybasic`, … | Fill `colortex*` + depth. Fallback chain when a program is missing (e.g. `gbuffers_water` → `gbuffers_terrain` → `gbuffers_textured`). |
| Deferred | `deferred`, `deferred1…` | Full-screen passes after opaque geometry. |
| Composite | `composite`, `composite1…` | Full-screen post passes. |
| Final | `final` | Writes the output image. |

Also needed: `shaders.properties` (settings, blend modes, custom textures, buffer formats),
pack option menus (`#define` toggles), `colortex0–15`, `depthtex0–2`, `shadowtex0/1`,
`shadowcolor0/1`, and dozens of built-in inputs (`gbufferModelView`, `sunPosition`,
`cameraPosition`, `frameTimeCounter`, `worldTime`, …).

### Shader translation (ADR 0004)

1. **Preprocess:** resolve `#include`, apply pack options as `#define`s, inject Aktis
   declarations.
2. **Patch:** rewrite legacy GLSL (`gl_FragData`, `texture2D`, `gl_ModelViewMatrix`,
   `attribute`/`varying`, built-in inputs) into core-profile GLSL with explicit bindings.
3. **Compile:** glslang (via `shaderc`) → SPIR-V.
4. **Translate:** naga SPIR-V → the backend (MSL on Metal, HLSL/DXIL or SPIR-V on Windows).

### Compatibility strategy

- Test packs in order: BSL → Complementary → others. Keep a compatibility table here.
- Temporal effects (TAA, accumulated clouds) need several warm-up frames per still.
- Iris (LGPL-3.0) is the behavioural reference; no code copied.

## Compatibility table

| Pack | Version | Status | Notes |
|---|---|---|---|
| — | — | — | Nothing tested yet |

## Open questions

- How to expose a pack's option menu in Aktis's simple UI?
- Fallback plan: an optional Fabric + Iris backend controlled by Aktis, if emulation stalls?
