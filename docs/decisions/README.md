# Architecture Decision Records

An ADR records one significant decision: context, the decision itself, and its consequences.
ADRs are never deleted; a newer ADR can supersede an older one.

To add one: copy `0000-template.md` to `NNNN-short-title.md` (next number) and add a row below.

| # | Title | Status |
|---|---|---|
| [0001](0001-rust-wgpu-egui.md) | Rust + wgpu + egui as the stack | Accepted |
| [0002](0002-no-bundled-mojang-assets.md) | Never bundle Mojang assets | Accepted |
| [0003](0003-two-render-engines.md) | Two engines: path tracer and shader-pack engine | Accepted |
| [0004](0004-shader-pack-translation.md) | Shader packs via GLSL → SPIR-V → naga | Accepted |
| [0005](0005-docs-and-review-workflow.md) | Docs-with-every-change and review-before-commit workflow | Accepted |
