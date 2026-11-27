# Scene (`aktis-scene`)

## Purpose

Describe *how* to render: camera, sun, sky and fog, as plain, serializable data that the
renderers read and that a future animation system can interpolate.

## Status

**Working** for the current fields (defaults, TOML round-trip, missing fields fall back to
defaults).

## Design

- Every field is a plain value (number, vector, enum). No GPU handles, no references.
- `#[serde(default)]` everywhere, so old scene files keep loading when fields are added.
- **Animation-ready:** M7 will add a timeline that interpolates these values per frame. Keep
  new fields interpolatable (numbers/vectors/colours), or document how they should step.

## File format

TOML, extension `.aktis-scene.toml` (planned). Example:

```toml
[camera]
position = [0.0, 80.0, 0.0]
yaw = 0.0
pitch = -20.0
fov_y = 70.0

[sun]
azimuth = 135.0
altitude = 45.0
intensity = 1.0

[sky]
intensity = 1.0

[fog]
density = 0.0
```

Breaking changes to this format need a CHANGELOG entry and a migration note here.

## Open questions

- Will a project file bundle scene + world path + asset stack + engine settings? (Default:
  yes, `project.aktis.toml`, decided in M1.)
