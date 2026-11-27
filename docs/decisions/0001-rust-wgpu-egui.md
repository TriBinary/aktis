# 0001: Rust + wgpu + egui as the stack

- **Status:** Accepted
- **Date:** 2026-11-27

## Context

Aktis must render on the GPU on macOS (Apple Silicon) and Windows. CUDA/OptiX don't exist on
Apple Silicon, and macOS OpenGL stops at 4.1 with no compute shaders. Development is largely
AI-assisted, so a strict compiler that catches mistakes early is valuable.

## Decision

- **Language:** Rust (2024 edition).
- **GPU:** wgpu (Metal / DX12 / Vulkan), with WGSL compute and render shaders.
- **UI:** egui via eframe, sharing the wgpu device with the renderers.
- **Ray tracing:** software voxel traversal in compute shaders; hardware RT is not required.

## Consequences

- One codebase for both platforms; the egui viewport can show engine output directly.
- wgpu has no stable cross-platform hardware ray tracing, but voxel scenes don't need it.
- eframe pins the wgpu major version; wgpu upgrades follow eframe releases.

## Alternatives considered

- **Swift + Metal:** best on Mac, but macOS-only.
- **Java/LWJGL (Chunky's ecosystem):** OpenGL on macOS can't do compute shaders.
- **C++ + Vulkan (MoltenVK):** heavier to maintain and less safe for AI-written code.
- **Web (WebGPU/Three.js):** limited for large worlds and long renders.
