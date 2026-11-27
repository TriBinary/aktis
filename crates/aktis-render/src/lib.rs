//! Shared GPU layer for Aktis.
//!
//! Responsibilities (see `docs/subsystems/render-core.md`):
//! - Own the wgpu device and queue (shared with the egui viewport).
//! - Define the [`RenderEngine`] trait that both engines implement.
//! - Upload world data to the GPU and read rendered images back.

use aktis_scene::Scene;

/// The wgpu device and queue used by every engine.
#[derive(Debug, Clone)]
pub struct GpuContext {
    /// Logical GPU device.
    pub device: wgpu::Device,
    /// Command queue for `device`.
    pub queue: wgpu::Queue,
}

/// Everything an engine needs to produce one frame or one progressive pass.
#[derive(Debug)]
pub struct FrameInput<'a> {
    /// The scene to render.
    pub scene: &'a Scene,
    /// Output resolution in pixels (width, height).
    pub resolution: (u32, u32),
}

/// Errors produced by render engines.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// The engine needs data that has not been uploaded yet (e.g. no world loaded).
    #[error("missing input: {0}")]
    MissingInput(&'static str),
    /// The feature is part of the roadmap but not implemented yet.
    #[error("not implemented yet: {0}")]
    NotImplemented(&'static str),
}

/// A rendering backend. Aktis has two: the path tracer and the shader-pack engine.
pub trait RenderEngine {
    /// Human-readable engine name shown in the UI.
    fn name(&self) -> &'static str;

    /// Renders one frame (or one progressive sample pass) for `input`.
    fn render(&mut self, gpu: &GpuContext, input: &FrameInput<'_>) -> Result<(), RenderError>;
}
