# Render core (`aktis-render`)

## Purpose

Shared GPU layer: the wgpu device/queue, the `RenderEngine` trait, uploading world data, and
reading images back.

## Status

**In progress.** Implemented: `GpuContext`, `FrameInput`, `RenderError` and the `RenderEngine`
trait. Upload and readback are `Planned`.

## Design

- **One device.** The app gets the wgpu device from eframe's render state and wraps it in
  `GpuContext`, so engines render into textures the egui viewport can show without copies.
- **`RenderEngine` trait:** `name()` and `render(gpu, input)`. Progressive engines do one
  sample pass per call. The trait will gain resize/reset/output-texture methods in M1–M2;
  keep both engines in sync when it changes.
- **World upload (planned):** two representations built from the same world + assets:
  - voxel grid (chunk → 16³ brick) + material table + texture arrays → path tracer
  - meshed geometry (per-section vertex buffers) → preview and shader-pack engine
- **Readback (planned):** copy the output texture into a buffer, map it, encode PNG
  (16-bit/EXR later).

## Open questions

- Output colour pipeline: linear HDR internally with tonemapping at the end? (Default: yes;
  `Rgba16Float` accumulation, ACES/AgX tonemap options.)
