# External dependencies

All versions are pinned in the root `Cargo.toml` under `[workspace.dependencies]`. Crates
reference them with `dep.workspace = true`.

**Status:** `Used` = a crate depends on it now · `Planned` = declared in the workspace, not yet
used.

## GPU & UI

| Crate | Version | Used by | Status | Purpose / notes |
|---|---|---|---|---|
| `wgpu` | 30 | render, pt, sp | Used | GPU API over Metal / DX12 / Vulkan. **Must match the wgpu major version used by `eframe`** (egui-wgpu shares the device). |
| `eframe` | 0.37 | app | Used | egui windowing + wgpu integration. Features: `wgpu`, `default_fonts`, `persistence` (+ `x11`, `wayland` on Linux). |
| `egui` | 0.37 | app | Used | Immediate-mode UI. Since 0.37, `eframe::App::ui(&mut Ui, …)` replaces `update(&Context, …)`, and `egui::Panel` replaces `SidePanel`. |
| `naga` | 30 | sp | Planned | Shader translation/validation (SPIR-V → backend). Same major version as wgpu. |
| `bytemuck` | 1 | render, pt | Used | Safe casting of GPU buffer structs. |
| `pollster` | 1 | — | Planned | Blocking on wgpu futures in tools/tests. |
| `shaderc` | 0.10 | sp | Planned | GLSL → SPIR-V via glslang for shader packs. Builds from source: needs CMake, Python 3 and a C++ toolchain (see ADR 0004). |

## Data & formats

| Crate | Version | Used by | Status | Purpose / notes |
|---|---|---|---|---|
| `glam` | 0.34 | world, scene, pt | Used | Vector/matrix math (`serde`, `bytemuck` features). |
| `serde` | 1 | assets, world, scene | Used | Serialization derive. |
| `serde_json` | 1 | assets | Used | Blockstate/model JSON. |
| `toml` | 1 | scene | Used | Scene/project files. |
| `zip` | 9 | assets | Used | Reading the client jar and zipped packs (`deflate` only). |
| `image` | 0.25 | assets | Used | PNG decode/encode (`png` only). |
| `fastanvil` | 0.32 | world | Used | Region (`.mca`) reading and chunk decoding. |
| `fastnbt` | 2 | world | Used | NBT via serde. |

## Rendering extras

| Crate | Version | Used by | Status | Purpose / notes |
|---|---|---|---|---|
| `oidn` | 2 | pt | Planned | Intel Open Image Denoise. Needs the OIDN runtime library at build/run time; packaging to be decided in M4. |

## Utilities

| Crate | Version | Used by | Status | Purpose / notes |
|---|---|---|---|---|
| `thiserror` | 2 | all libs | Used | Error enums in libraries. |
| `anyhow` | 1 | app | Used | Error handling in the binary only. |
| `log` | 0.4 | most | Used | Logging facade. |
| `env_logger` | 0.11 | app | Used | Log output (`RUST_LOG=aktis=debug`). |
| `rayon` | 1 | — | Planned | Parallel chunk decoding/meshing. |
| `directories` | 6 | — | Planned | OS-specific config/cache/Minecraft paths. |
| `rfd` | 0.18 | — | Planned | Native file/folder dialogs. |

## Adding a dependency

1. Add it to `[workspace.dependencies]` with minimal features.
2. Reference it from the crate with `.workspace = true`.
3. Add a row here (purpose, features, platform notes).
4. If it brings native code, a heavy build, or licence concerns, write an ADR.
5. Dependency licences must be compatible with MIT/Apache-2.0 distribution.
