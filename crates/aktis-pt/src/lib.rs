//! Aktis path-tracing engine.
//!
//! Progressive, physically based rendering in wgpu compute shaders: voxel ray traversal
//! (chunk → block), PBR materials from LabPBR resource packs, sun/sky sampling and
//! denoising (see `docs/subsystems/path-tracer.md`).

use aktis_render::{FrameInput, GpuContext, RenderEngine, RenderError};

/// Path-tracer settings.
#[derive(Debug, Clone, PartialEq)]
pub struct PathTracerSettings {
    /// Samples per pixel to accumulate before the render counts as finished.
    pub target_spp: u32,
    /// Maximum ray bounces.
    pub max_bounces: u32,
}

impl Default for PathTracerSettings {
    fn default() -> Self {
        Self {
            target_spp: 256,
            max_bounces: 5,
        }
    }
}

/// The path-tracing engine.
#[derive(Debug, Default)]
pub struct PathTracer {
    /// Current settings.
    pub settings: PathTracerSettings,
}

impl RenderEngine for PathTracer {
    fn name(&self) -> &'static str {
        "Path Tracer"
    }

    fn render(&mut self, _gpu: &GpuContext, _input: &FrameInput<'_>) -> Result<(), RenderError> {
        Err(RenderError::NotImplemented("path tracer (milestone M2)"))
    }
}
