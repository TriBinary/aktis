# World (`aktis-world`)

## Purpose

Read Java Edition save folders and give the renderers the blocks, biomes and block entities
inside a selection.

## Status

**In progress.** Implemented: `World::open` (checks for `level.dat`), and the `BlockState` and
`Selection` types. Chunk reading is `Planned`.

## Design

| Piece | Status | Notes |
|---|---|---|
| Save discovery | Planned | List worlds in `<minecraft>/saves`. |
| Dimensions | Planned | Overworld `region/`, Nether `DIM-1/region/`, End `DIM1/region/`. |
| Region reading | Planned | `fastanvil` reads `.mca`; `fastnbt` decodes chunk NBT. |
| Section palettes | Planned | Block-state palette + packed indices → `BlockState` ids. |
| Biomes | Planned | Needed for grass/foliage/water tint. |
| Block entities | Planned | Signs, chests, banners, heads… for special rendering. |
| Selection | Working (type only) | Inclusive block box; the app lets users drag it on a map. |
| Version range | Planned | Target 1.18+ chunk format first (−64..320 height); older formats later. |

## Open questions

- Should Aktis read worlds while the game has them open? (Default: yes, read-only; warn the
  user about stale data.)
