//! Scene description for Aktis.
//!
//! A scene is plain, serializable data: camera, sun, sky and fog. Renderers read it;
//! they never own it. Every field is a value that a future animation system can
//! interpolate over time (see `docs/subsystems/scene.md`).

use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Everything needed to describe how a world should be rendered.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Scene {
    /// Camera placement and lens.
    pub camera: Camera,
    /// Sun direction and strength.
    pub sun: Sun,
    /// Sky appearance.
    pub sky: Sky,
    /// Atmospheric fog.
    pub fog: Fog,
}

/// Camera placement and lens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Camera {
    /// Position in world (block) coordinates.
    pub position: Vec3,
    /// Yaw in degrees (Minecraft convention).
    pub yaw: f32,
    /// Pitch in degrees (Minecraft convention).
    pub pitch: f32,
    /// Vertical field of view in degrees.
    pub fov_y: f32,
}

/// Sun direction and strength.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sun {
    /// Azimuth in degrees.
    pub azimuth: f32,
    /// Altitude above the horizon in degrees.
    pub altitude: f32,
    /// Radiance multiplier.
    pub intensity: f32,
}

/// Sky appearance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sky {
    /// Radiance multiplier for sky light.
    pub intensity: f32,
}

/// Atmospheric fog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Fog {
    /// Fog density; `0.0` disables fog.
    pub density: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 80.0, 0.0),
            yaw: 0.0,
            pitch: -20.0,
            fov_y: 70.0,
        }
    }
}

impl Default for Sun {
    fn default() -> Self {
        Self {
            azimuth: 135.0,
            altitude: 45.0,
            intensity: 1.0,
        }
    }
}

impl Default for Sky {
    fn default() -> Self {
        Self { intensity: 1.0 }
    }
}

impl Default for Fog {
    fn default() -> Self {
        Self { density: 0.0 }
    }
}

/// Errors produced while reading or writing scene files.
#[derive(Debug, thiserror::Error)]
pub enum SceneError {
    /// The scene file could not be parsed.
    #[error(transparent)]
    Parse(#[from] toml::de::Error),
    /// The scene could not be serialized.
    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),
}

impl Scene {
    /// Parses a scene from TOML (the `.aktis` scene file format).
    pub fn from_toml(text: &str) -> Result<Self, SceneError> {
        Ok(toml::from_str(text)?)
    }

    /// Serializes the scene to TOML.
    pub fn to_toml(&self) -> Result<String, SceneError> {
        Ok(toml::to_string_pretty(self)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toml_roundtrip() {
        let scene = Scene::default();
        let text = scene.to_toml().unwrap();
        assert_eq!(Scene::from_toml(&text).unwrap(), scene);
    }

    #[test]
    fn missing_fields_use_defaults() {
        let scene = Scene::from_toml("[sun]\naltitude = 10.0\n").unwrap();
        assert_eq!(scene.sun.altitude, 10.0);
        assert_eq!(scene.camera, Camera::default());
    }
}
