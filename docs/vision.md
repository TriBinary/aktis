# Vision

## What Aktis is

Aktis (Greek *ἀκτίς*, "ray of light") is a desktop render studio for Minecraft Java Edition
worlds. You open a world, choose a camera and lighting, and render images on the GPU, with
either:

- **Path Tracer:** physically based, photoreal light transport with PBR resource-pack support.
- **Shader Pack:** existing OptiFine/Iris shader packs (e.g. BSL, Complementary), re-hosted
  outside the game.

It's inspired by [Chunky](https://chunky-dev.github.io/docs/), but with GPU rendering by
default, a simpler UI, and an architecture that leaves room for animation later.

## Goals

1. **GPU first.** Rendering runs on the GPU through wgpu (Metal on macOS, DX12/Vulkan on
   Windows).
2. **Cross-platform.** macOS (Apple Silicon) and Windows are first-class.
3. **Faithful assets.** Vanilla look from the user's own client jar; resource packs, including
   LabPBR materials, stack on top.
4. **Shader-pack support.** Popular packs render correctly; compatibility grows over time.
5. **Simple UI.** Sensible defaults; advanced settings tucked away.
6. **Stills first, animation-ready.** Scene data is plain values that can later be keyframed.
7. **Small worlds fast; big worlds possible.** Optimise for builds and small areas; stream and
   use lower detail for large maps later.

## Non-goals (for now)

- Bedrock Edition worlds.
- Editing worlds (Aktis is read-only).
- Running inside the Minecraft client.
- Supporting every shader pack ever written.
- Linux release builds (it may well compile there; not tested in CI).

## Legal constraints

- Mojang assets are never redistributed: see
  [ADR 0002](decisions/0002-no-bundled-mojang-assets.md).
- No code is copied from GPL/LGPL projects (Chunky, Iris); they are references only.
