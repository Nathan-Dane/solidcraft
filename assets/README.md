# Application resources

`assets/` is the design resource home for SolidCraft ([#433](https://github.com/bherbruck/solidcraft/issues/433)).
[manifest.json](manifest.json) indexes exact files, ownership, sources/exports,
platform bindings, consumers, licences and regeneration commands.
[ATTRIBUTION.md](../ATTRIBUTION.md) records provenance.

```text
assets/
  icons/
    app/
      shared/source/                 # original Rust mark and executable ICO encoder
      macos/source/SolidCraft.icon/   # approved editable Composer project + SVG layers
      macos/exports/                 # Assets.car + icon.icns, preserved together
      windows/exports/               # executable DIB ICO + installer PNG ICO
      linux/exports/hicolor/         # canonical 16–512 px package images
      freebsd/                       # shares Linux images; no duplicate master
      web/                           # records current absence of favicon/PWA artwork
    ui/source/                       # drawing library, glyphs, navigation, markers, pointer
  branding/web/source/og.png          # existing shared social preview card
  styles/ui/palettes.rs               # light/dark defaults
  styles/web/loading.css             # web loading/canvas styles
  appearances/library.rs             # built-in visual presets
  materials/library.rs               # physical defaults and colours
  shaders/viewport.wgsl               # original viewport shader
  fonts/{liberation-sans,biz-udp-gothic}/ # unmodified fonts + accompanying OFL licences
  locales/{en,de,ja,ko}.json          # shared offline catalogs
  platforms/{macos,linux,windows}/    # bundle/desktop/MIME/AppStream/WiX templates
  platforms/{freebsd,web}/            # shared metadata and host bindings
  archive/icons/macos-v1/
    source/icon.vectorcraft          # earlier editable work, retained
    exports/                         # SVG, PNG and every historical iconset image
```

## Design workflow

Edit the authoritative source, then regenerate and review its exports. Original
Rust geometry, colours, IDs, captions and appearance variants are retained.
The current macOS master is `icons/app/macos/source/SolidCraft.icon/`; its linked
SVG layers, backdrop and Dark specialization remain together. The earlier
[VectorCraft document](archive/icons/macos-v1/source/icon.vectorcraft) is preserved
as history. No Illustrator or other native projects were available for the
current UI set or preview card. Keep any actual native work and linked inputs
under the resource's `source/work/`, and declare/attribute them; do not invent
placeholder documents or replace editable work with flattened images.

[UI artwork](icons/ui/README.md) lists the drawing masters and icon IDs. Use its
existing 24 × 24 grid and semantic ink/fill/accent colours. The labelled review
sheet includes both themes and the existing fallback artwork:

```sh
cargo xtask assets --generate icon.ui.sheet
```

The preview is `target/resources/ui-icons.png`. Temporary renders, caches and
package staging stay under ignored `target/` or `dist/` directories.

## Exports and platform bindings

From the repository root:

```sh
cargo xtask assets --generate icon.app.windows
cargo xtask assets --generate icon.app.linux
cargo xtask assets --generate icon.app.macos  # Xcode with Icon Composer support
cargo xtask assets
cargo xtask assets --check-macos             # compare Apple compiler renditions
```

Generation stages outputs under `target/resources/`, then updates the required
committed exports and their SHA-256 source/output digests. Review and commit
those changes together. When editing the shared mark, regenerate **both** Windows
and Linux exports. macOS generation emits the catalogue and fallback together.
Normal builds consume the committed files without requiring design applications
or Apple tools; build scripts write only to `OUT_DIR`.

| Platform | Binding |
| --- | --- |
| macOS | Bundle copies `Assets.car` and `icon.icns` from `icons/app/macos/exports/`; Composer is the editable master |
| Windows executable | Embeds `solidcraft-executable.ico`: original bottom-up BGRA DIBs and transparency masks, 16/24/32/48/64/128/256 px |
| Windows installer | `solidcraft.ico`: original PNG representations, 16/20/24/32/40/48/64/128/256 px; existing `SOLIDCRAFT_ICONS` override retained |
| Linux, Nix, deb/rpm, tar, Flatpak, Snap | Copy `icons/app/linux/exports/hicolor/`; installed icon name remains `ai.storyteller.solidcraft` |
| AppImage | Copies the Linux 256 px image and retains `.DirIcon` alias |
| FreeBSD | Reuses Linux images and freedesktop metadata |
| Web | Trunk copies Japanese font/licence to `fonts/`, includes collected CSS; current favicon/PWA absence retained |
| Docs and web previews | `xtask book` stages the shared card at the existing `/solidcraft/og.png` public path |

`solidcraft-cli icon` still supports arbitrary PNG/ICO exports. Its optional
1024 px output has no package consumer and is not another committed master.

## Code boundaries and validation

Artwork is collected by purpose, while compilation stays in its owning layer:
`ui-egui::icons::paint` is the stable L5 facade, palettes extend the existing
`Tokens` API, model presets stay compiled by L2 `doc`, and sketch text embeds
its font at L1. Projection, interaction, commands, theme selection, user data,
GPU pipeline creation and packaging logic stay in their existing modules.
Embedded data paths use `CARGO_MANIFEST_DIR`; normal runtime use does not read
the repository or parse the manifest. Shared resources require the workspace
source tree when building/publishing individual crates.

`cargo xtask assets` checks unique IDs, exact files/consumers/licences, full
collection coverage, attribution rows, safe paths, linked Composer layers and
source/export digests. New native work, catalogs and metadata must be declared;
only explicitly listed Markdown documentation is exempt. The owning UI tests
validate command and direct icon references, including explicitly indexed
existing fallback IDs. `cargo xtask ci` runs this gate alongside formatting,
licence, layering, clippy, tests and wasm checks.

Dependency-owned egui fonts and vendored renderer shaders stay with their
dependencies and are indexed with their licences. System fonts, optional font
packs, user fonts/canvases/decals/materials, native pointers/window controls and
dialogs retain external ownership. Root licence and notice texts stay canonical.
Archived artwork has no production consumer; packaging selects exact current
files and fails when required exports are absent.
