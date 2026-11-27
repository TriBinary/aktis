# 0004: Shader packs via GLSL → SPIR-V → naga

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

Shader packs are written in legacy OpenGL GLSL (versions 120–4xx, compatibility features).
wgpu's naga GLSL front-end only accepts modern, Vulkan-style GLSL, so it can't read packs
directly.

## Decision

Preprocess and patch pack GLSL into core-profile GLSL with explicit bindings. Compile it with
glslang (via the `shaderc` crate) to SPIR-V, then translate it with naga to the active backend.

## Consequences

- `shaderc` builds glslang from C++ source: contributors and CI need CMake, Python 3 and a C++
  compiler (all present on GitHub's macOS and Windows runners). Build time goes up.
- The patcher is the hardest part; it is tested per pack, against a compatibility table.

## Alternatives considered

- naga GLSL front-end directly: doesn't support legacy GLSL.
- SPIRV-Cross for translation: an extra C++ dependency; naga already ships with wgpu.
- Writing an own GLSL front-end: too large.
