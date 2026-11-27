# Glossary

| Term | Meaning |
|---|---|
| **Client jar** | The Minecraft Java Edition `versions/<v>/<v>.jar` file; a zip with vanilla assets. |
| **Resource pack** | Zip or folder that overrides assets (textures, models, blockstates). |
| **Asset stack** | Ordered list of sources (jar at the bottom, packs above); the top wins. |
| **Blockstate** | JSON mapping a block's property values to models (`assets/<ns>/blockstates`). |
| **Block model** | JSON describing cuboid elements and texture faces (`assets/<ns>/models`). |
| **LabPBR** | Common PBR convention for resource packs: `_n` (normal + AO + height) and `_s` (smoothness, F0/metal, porosity, emission) textures. |
| **Region file** | `.mca` file holding 32×32 chunks of a world. |
| **NBT** | Minecraft's binary tag format, used for chunks and `level.dat`. |
| **Section** | 16×16×16 part of a chunk with a block-state palette. |
| **Selection** | Block-coordinate box chosen for rendering. |
| **Scene** | Camera, sun, sky and fog settings: plain, serializable, animation-ready data. |
| **Engine** | A rendering backend implementing `RenderEngine` (Path Tracer, Shader Pack). |
| **spp** | Samples per pixel, accumulated progressively by the path tracer. |
| **Shader pack** | OptiFine/Iris package of GLSL programs (`shaders/`) that defines a real-time pipeline. |
| **Program (shader pack)** | One named stage such as `gbuffers_terrain`, `shadow`, `composite1`, `final`. |
| **colortex / depthtex / shadowtex** | Named render targets that shader-pack programs read and write. |
| **SPIR-V** | Intermediate shader format; GLSL is compiled to it, then translated to MSL/HLSL/WGSL. |
| **ADR** | Architecture Decision Record, in `docs/decisions/`. |
