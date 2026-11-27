# Getting started

## Requirements

| Tool | Version | Notes |
|---|---|---|
| Rust | stable, ≥ MSRV in `Cargo.toml` (1.95) | `rust-toolchain.toml` selects stable with rustfmt + clippy |
| macOS | Apple Silicon recommended | Xcode Command Line Tools (`xcode-select --install`) |
| Windows | 10/11 | Visual Studio Build Tools (C++ workload) |
| Minecraft Java Edition | any recent | Needed at runtime for vanilla assets |
| CMake + Python 3 | — | Only once `shaderc` is enabled (M5) |

IDE: RustRover (JetBrains) or any editor with rust-analyzer.

## Build and run

```sh
cargo run                 # debug build of the app (dependencies are optimised, see Cargo.toml profiles)
cargo run --release       # full optimisation
RUST_LOG=aktis=debug cargo run   # verbose logs (PowerShell: $env:RUST_LOG="aktis=debug")
```

## Checks (same as CI)

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
