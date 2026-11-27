//! Minecraft asset loading for Aktis.
//!
//! Responsibilities (see `docs/subsystems/assets.md`):
//! - Read vanilla assets from the user's own Minecraft client jar (never bundled).
//! - Stack resource packs on top, in priority order.
//! - Resolve blockstates → block models → baked quads with texture references.
//! - Load PBR maps following the LabPBR convention (`_n` normal, `_s` specular).

mod location;
mod source;

pub use location::{ResourceLocation, ResourceLocationError};
pub use source::{AssetSource, AssetStack};

/// Errors produced while loading assets.
#[derive(Debug, thiserror::Error)]
pub enum AssetError {
    /// The requested asset does not exist in any source of the stack.
    #[error("asset not found: {0}")]
    NotFound(ResourceLocation),
    /// Filesystem error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// The feature is part of the roadmap but not implemented yet.
    #[error("not implemented yet: {0}")]
    NotImplemented(&'static str),
}
