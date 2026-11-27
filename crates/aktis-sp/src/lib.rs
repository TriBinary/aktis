//! Aktis shader-pack engine.
//!
//! Re-creates the OptiFine/Iris rendering pipeline so that existing shader packs
//! (BSL, Complementary, …) can render Aktis scenes. Pack GLSL is preprocessed,
//! patched, compiled to SPIR-V and translated to the native backend
//! (see `docs/subsystems/shader-packs.md`).

use std::path::PathBuf;

use aktis_render::{FrameInput, GpuContext, RenderEngine, RenderError};

/// A shader pack chosen by the user (a `.zip` or a folder containing `shaders/`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShaderPackSource {
    /// Path to the pack.
    pub path: PathBuf,
}

/// The shader-pack engine.
#[derive(Debug, Default)]
pub struct ShaderPackEngine {
    /// The active pack, if any.
    pub pack: Option<ShaderPackSource>,
}

impl RenderEngine for ShaderPackEngine {
    fn name(&self) -> &'static str {
        "Shader Pack"
    }

    fn render(&mut self, _gpu: &GpuContext, _input: &FrameInput<'_>) -> Result<(), RenderError> {
        if self.pack.is_none() {
            return Err(RenderError::MissingInput("no shader pack selected"));
        }
        Err(RenderError::NotImplemented(
            "shader-pack engine (milestone M5)",
        ))
    }
}
