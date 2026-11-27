# Aktis

**GPU render studio for Minecraft Java Edition worlds.**

Aktis (Greek *ἀκτίς*, "ray of light") renders your Minecraft worlds on the graphics card, with
two engines:

- **Path Tracer:** photoreal, physically based lighting with PBR resource-pack support.
- **Shader Pack:** your favourite OptiFine/Iris shader packs, outside the game.

Runs on **macOS (Apple Silicon)** and **Windows**.

> **Status: early development (M0, scaffold).** Nothing renders yet; see the
> [roadmap](docs/roadmap.md).

## Features (planned)

- Open any Java Edition world and select an area to render
- Vanilla look from your own Minecraft installation, with resource packs stacked on top
- LabPBR materials (normal and specular maps)
- Progressive GPU path tracing with denoising
- OptiFine/Iris shader-pack rendering
- Simple UI with sensible defaults
- Animation and video rendering (later)

## Requirements

- Minecraft Java Edition installed (Aktis reads vanilla textures from your copy; it never
  ships Mojang assets)
- A GPU with Metal (macOS) or DirectX 12 / Vulkan (Windows) support

## Building

```sh
git clone https://github.com/TriBinary/aktis.git
cd aktis
cargo run --release
```

More in [docs/dev/getting-started.md](docs/dev/getting-started.md).

## Documentation

Start at [docs/README.md](docs/README.md). Contributors (human or AI) should also read
[CLAUDE.md](CLAUDE.md) and [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your
option.

Minecraft is a trademark of Mojang Studios. Aktis is not affiliated with or endorsed by Mojang
or Microsoft.
