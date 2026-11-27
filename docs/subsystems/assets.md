# Assets (`aktis-assets`)

## Purpose

Provide every block's geometry and textures (including PBR maps) from the user's Minecraft
installation and resource packs.

## Status

**In progress.** Implemented: `ResourceLocation` (parsing, asset paths) and `AssetStack`
(priority ordering). Everything else is `Planned`.

## Design

| Piece | Status | Notes |
|---|---|---|
| Minecraft install discovery | Planned | Default per OS; user can override. Pick a version from `versions/`. |
| Client jar reader | Planned | Read `assets/**` from the jar zip; never extract it into the repo. |
| Resource pack reader | Planned | Zip or folder; read `pack.mcmeta` for format/description. |
| Asset stack lookup | Working (ordering only) | Top of stack wins, like the in-game pack screen. |
| Blockstates | Planned | `variants` and `multipart`; weighted random variants (seeded by position). |
| Block models | Planned | Parent inheritance, texture variables (`#side`), elements, rotations, `uvlock`, tint indices. |
| Texture atlas | Planned | Pack textures into GPU arrays; handle `.mcmeta` animations (first frame for stills). |
| LabPBR | Planned | `_n` (normal, AO, height) and `_s` (smoothness, F0/metal, porosity/SSS, emission). |
| Biome tints | Planned | Grass/foliage/water colormaps. |
| Special-case blocks | Planned | Water/lava, chests, signs, banners, beds etc., which have no plain JSON model. |

**Legal:** assets are read at runtime only. See ADR 0002.

## Open questions

- Which Minecraft versions to support first? (Default: the latest release plus one older
  version.)
- How to load modded assets (Fabric/Forge mod jars as extra sources)?
