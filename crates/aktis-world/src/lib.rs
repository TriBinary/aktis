//! Minecraft world loading for Aktis.
//!
//! Responsibilities (see `docs/subsystems/world.md`):
//! - Open a Java Edition save folder and read region (`.mca`) files.
//! - Decode chunk sections into block-state palettes, biomes and block entities.
//! - Expose a selected region of the world to the renderers.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A block state such as `minecraft:oak_stairs[facing=north,half=bottom]`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockState {
    /// Block identifier, e.g. `minecraft:oak_stairs`.
    pub name: String,
    /// Block state properties, sorted by key.
    pub properties: BTreeMap<String, String>,
}

/// Inclusive block-coordinate box selecting the part of the world to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    /// Minimum corner (x, y, z).
    pub min: glam::IVec3,
    /// Maximum corner (x, y, z).
    pub max: glam::IVec3,
}

/// Errors produced while loading a world.
#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    /// The folder does not look like a Minecraft save (no `level.dat`).
    #[error("not a Minecraft world: {0}")]
    NotAWorld(PathBuf),
    /// Filesystem error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// The feature is part of the roadmap but not implemented yet.
    #[error("not implemented yet: {0}")]
    NotImplemented(&'static str),
}

/// Handle to an opened Minecraft save folder.
#[derive(Debug)]
pub struct World {
    root: PathBuf,
}

impl World {
    /// Opens a save folder. Only checks that `level.dat` exists for now.
    pub fn open(root: impl AsRef<Path>) -> Result<Self, WorldError> {
        let root = root.as_ref().to_path_buf();
        if !root.join("level.dat").is_file() {
            return Err(WorldError::NotAWorld(root));
        }
        Ok(Self { root })
    }

    /// Root folder of the save.
    pub fn root(&self) -> &Path {
        &self.root
    }
}
