# App (`aktis-app`)

## Purpose

The desktop application (binary `aktis`): opening worlds and packs, a viewport, a simple
settings panel, rendering and export.

## Status

**In progress.** A window with an engine selector, sun sliders and a placeholder viewport.

## Design

- **eframe 0.37 + wgpu.** Implement `eframe::App::ui`. Engines render into wgpu textures shown
  in the central panel.
- **Layout (planned):** left = project (world, selection, asset stack); centre = viewport;
  right = scene + engine settings; bottom = render progress / queue.
- **Simplicity rule:** each panel shows basic settings first; advanced ones go in collapsible
  "Advanced" sections.
- **Persistence:** window state through eframe `persistence`; projects as TOML files.

## Open questions

- Translations (English + Chinese)? (Default: English first, keep strings in one module so
  i18n can be added later.)
