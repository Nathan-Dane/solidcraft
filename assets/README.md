# SolidCraft application resources

Resource audit for [#433](https://github.com/bherbruck/solidcraft/issues/433),
against `main` at `0b1e615`. Paths below are current; destinations are proposed.
This revision inventories resources without relocating them.

## Proposed structure

Group resources by purpose, then platform where needed. Separate editable work,
shipped exports and archived designs. Fonts, catalogs and templates that are
already their own source need no duplicate `source/` and `exports/` copies.

```text
assets/
  README.md
  manifest.json                        # resource IDs, sources, exports, consumers
  icons/
    macos/
      source/
        SolidCraft.icon/               # current editable Composer project
          icon.json
          Assets/                      # approved SVG layers
      exports/
        Assets.car                     # committed for older Xcode runners
        icon.icns                      # matching legacy macOS fallback
  fonts/
    liberation-sans/
      LiberationSans-Regular.ttf
      OFL.txt
    biz-udp-gothic/
      BIZUDPGothic-Regular.ttf
      OFL.txt
  locales/
    en.json
    de.json
    ja.json
    ko.json
  platforms/
    macos/
      Info.plist.in
      README.txt
    linux/
      ai.storyteller.solidcraft.desktop
      ai.storyteller.solidcraft.mime.xml
      ai.storyteller.solidcraft.metainfo.xml.in
    windows/
      solidcraft.wxs
      solidcraft.en-us.wxl
  archive/
    icons/
      macos-v1/
        source/icon.vectorcraft        # original editable work, retained
        exports/                       # previous SVG, PNG and icon.iconset/
```

The tree is the migration target; `manifest.json` is not implemented yet.
Rust drawing/generation code stays in its owning modules and is indexed by the
manifest. Shared icon code remains `apps/app_icon.rs`; UI code stays in `ui-egui`.
Build logic stays in `packaging/` and `xtask/`. Root licence files stay canonical.

### Design workflow

Open the resource's master from `source/`; export to its documented destination.
Keep any original `.vectorcraft`, `.ai`, `.afdesign`, `.psd` or other work files
alongside that resource's master, with linked inputs and provenance. Identify one
authoritative entry point and the role of each supporting file. Never replace an
editable project with only its flattened export.

The available native sources are the [VectorCraft document](icon.vectorcraft)
and [Composer bundle](SolidCraft.icon/). No Illustrator or other native artwork
files were found in the checkout.
The VectorCraft document belongs to the earlier design; the current master is
`SolidCraft.icon`, including its SVG layers and appearance settings. Preserve both
without treating the earlier document as the current master.

Commit deliberate native work and required shipped exports together. Save
temporary renders, tool caches, previews and package staging under `target/` or
`dist/`. Production consumers select explicit current paths, never the archive
or a recursive copy of all `assets/`.

Provide one regeneration entry point per generated resource, invoked from the
repository root: proposed `cargo xtask assets --generate icon.app.macos` compiles
in `target/`, verifies the result, then updates both checked-in icon exports and
their recorded source/output digests. The existing commands below remain usable
until that helper is implemented.

### Code workflow and validation

Use one named constant at each owning crate's resource boundary (`fonts.rs`,
`i18n.rs`); application code uses those existing APIs. Anchor embedded paths at
`CARGO_MANIFEST_DIR`, with forward slashes, rather than the source file's depth:

```rust
static DEFAULT_FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/fonts/liberation-sans/LiberationSans-Regular.ttf"
));
```

[Rust includes](https://doc.rust-lang.org/std/macro.include_bytes.html) embed
resources at compile time. Keep sketch fonts independent of the UI; keep the
web Japanese font as an on-demand static file. No runtime repository lookup or
new dependency on a UI/resource-loader crate is needed. Shared resources require
the workspace source tree; standalone crate publication must bundle its inputs.

Packaging uses `$ROOT/assets/...`; Trunk copies from the new source directory
while preserving `./fonts/BIZUDPGothic-Regular.ttf` in the built site. Record
explicit [Cargo rebuild inputs](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
for generated resources. Normal builds consume checked-in exports and never
require Illustrator, VectorCraft or Icon Composer.

The proposed `manifest.json` is a validation/index file, not a runtime registry.
Each entry has a stable ID, purpose, platforms, status (`current` or `archive`),
master/source files, shipped exports, generator command or `none`, licence/notice
paths and consumers. Paths are repository-relative and exact; code-generated
entries point to their owning Rust module. `ATTRIBUTION.md` remains the provenance
record; font licences remain beside their fonts.

Extend `cargo xtask assets` to check every declared file and consumer, unique IDs,
full coverage of `assets/` (with explicit documentation exemptions), attribution,
Composer layer references and source/export separation. Include native work
formats, catalogs and metadata; reject missing or undeclared resources. Packaging
must fail if a required export is absent. Recorded digests detect changed sources
or exports on every platform. Validate macOS source/export agreement
with `actool` where available; do not compare timestamp-bearing catalogs bytewise
or require Apple tools on other platforms.

## Files to collect

There are 21 current resource files and 13 historical icon files.
Paths are relative to the repository root. Each font's licence stays beside it.

| Resource / authoritative source | Current files | Proposed destination under `assets/` | Consumers |
| --- | --- | --- | --- |
| Approved macOS icon | `assets/SolidCraft.icon/icon.json`; `Assets/02-grid.svg`, `Assets/02-grid-dark.svg`, `Assets/03-solid-blue-cutout.svg` within that bundle | `icons/macos/source/SolidCraft.icon/` | Icon Composer, `actool` |
| Compiled macOS icons | `assets/Assets.car`, `assets/icon.icns`, generated together from the approved composition | `icons/macos/exports/` | `packaging/macos/package.sh` |
| Historical macOS artwork | `assets/icon.vectorcraft` (editable source), `assets/icon.svg`, `assets/icon.png`, all ten PNGs in `assets/icon.iconset/` | `archive/icons/macos-v1/source/` and `exports/` | Preserve all 13 files as isolated design history; no current packaging/runtime consumer |
| Sketch text font | `crates/sketch/fonts/LiberationSans-Regular.ttf`, `OFL.txt` in that directory | `fonts/liberation-sans/` | `crates/sketch/src/fonts.rs` embeds the font; `xtask/src/licences.rs` reads its licence |
| Web Japanese UI font | `apps/solidcraft-web/fonts/BIZUDPGothic-Regular.ttf`, `OFL.txt` in that directory | `fonts/biz-udp-gothic/` | `apps/solidcraft-web/index.html` copies both with Trunk; `src/main.rs` fetches the font; `xtask/src/licences.rs` reads its licence |
| Offline translations | `crates/ui-egui/src/locales/en.json`, `de.json`, `ja.json`, `ko.json` in that directory | `locales/` | `crates/ui-egui/src/i18n.rs` embeds all four; desktop and web share the registry |
| macOS bundle and install text | `packaging/macos/Info.plist.in`, `packaging/macos/README.txt` | `platforms/macos/` | `packaging/macos/package.sh` substitutes the plist and copies the instructions |
| Linux / FreeBSD desktop metadata | `packaging/linux/ai.storyteller.solidcraft.desktop`, `ai.storyteller.solidcraft.mime.xml`, `ai.storyteller.solidcraft.metainfo.xml.in` in that directory | `platforms/linux/` | Linux and FreeBSD packaging, `flake.nix`, packaging lint; staged files feed Flatpak and Snap |
| Windows installer templates | `packaging/windows/solidcraft.wxs`, `packaging/windows/solidcraft.en-us.wxl` | `platforms/windows/` | `packaging/windows/package.ps1`, packaging lint |

Original artwork and translations use MIT OR Apache-2.0. Both bundled fonts
use SIL OFL 1.1, unmodified. Platform packaging templates are adapted from
PhotoCraft under Apache-2.0. [ATTRIBUTION.md](../ATTRIBUTION.md) records provenance;
add explicit rows for relocated locale catalogs and platform resources.

## Code and canonical files to index

| Resource | Authoritative location | Handling / consumers |
| --- | --- | --- |
| Shared app mark | `apps/app_icon.rs` | Keep its code ownership; desktop, CLI and Windows `build.rs` include it directly; index its generated exports and rebuild trigger |
| UI icons and glyphs | `crates/ui-egui/src/icons.rs`, `viewport.rs`, `navigation_cube.rs`, `timeline.rs`, `sketch_tools.rs` in that directory | Keep drawing code in its owning UI layer; inventory it here |
| Theme, appearances and rendering | `crates/ui-egui/src/theme.rs`, inline shaders in `gpu.rs`, `crates/doc/src/appearance.rs`, material definitions in `crates/doc/src/document.rs` | Keep original code in its owning layers; no external texture/environment artwork to relocate |
| Dependency fonts and shader | egui's built-in fonts; `vendor/egui-wgpu/src/egui.wgsl` | Keep dependency/vendor ownership; see `THIRD-PARTY-LICENSES.txt` and `ATTRIBUTION.md` |
| Optional desktop fonts | System font paths and `CRAFT_FONTS_DIR` in `crates/ui-egui/src/fonts.rs`; user sketch fonts and `SOLIDCRAFT_FONT_DIRS` in `crates/sketch/src/fonts.rs` | External resources; no files to collect |
| Licence and notice texts | Root `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`, `THIRD-PARTY-LICENSES.txt`, `ATTRIBUTION.md` | Keep canonical root files; engine embeds licence texts, About embeds attribution, packaging copies licences/notice; `cargo xtask licences` generates the third-party list |
| Package README | Root `README.md` | Keep canonical documentation; packaging copies it |

Shared mark, glyphs, theme and appearance code use MIT OR Apache-2.0;
dependency resources retain their own licences. WiX ships the root licence texts.

## Generation and integration checklist

- Shared PNG/ICO exports: `solidcraft-cli icon --size 256 --out target/icon.png`
  or `solidcraft-cli icon --out target/icon.ico`. `packaging/env.sh::make_icons`
  generates package sizes; Windows `build.rs` embeds an ICO in the executable.
  Keep generated outputs in ignored build/staging directories.
- Fonts, locales and metadata: edit the listed source files; no independent
  generation step. Preserve font bytes, message IDs, installer GUIDs, compatibility
  IDs, installed paths and the web URL `./fonts/BIZUDPGothic-Regular.ttf`.
- Update includes/rebuild triggers, Trunk copy paths, packaging input paths,
  `flake.nix`, `xtask/src/licences.rs`, `ATTRIBUTION.md`, `docs/localization.md`,
  `docs/compatibility.md` and `.github/workflows/packaging-lint.yml` paths/triggers.
- Extend `xtask/src/assets.rs` beyond its current extension-based attribution scan:
  cover `.car`, `.vectorcraft`, Composer `icon.json`, catalogs and metadata;
  reject missing referenced files and unaccounted application resources.
- Validate with `cargo xtask ci`, desktop/web font and locale loading, Windows
  embedded icons, Linux packaging, and signed macOS bundles with both icon formats.
  Retain checked-in macOS compilations for older Xcode runners.

Build logic stays in `packaging/` and `xtask/`, including nfpm, Flatpak and Snap
manifests, AppRun and install scripts. The web HTML/Trunk config remains its build
entry point. Documentation media (`docs/screenshots/`, generated book images),
test/oracle fixtures, examples, user designs and review captures keep their
purpose-specific locations; attribution checks still apply where appropriate.

## macOS icon regeneration

`SolidCraft.icon/` is the approved Icon Composer source, containing separate
grid and solid SVG layers and the owner's appearance settings. The saturated
blue solid has transparent geometry cutouts instead of painted edge lines.
The solid and grid are both scaled to 90%, with separate Default and Dark grid artwork.
Icon Composer supplies the system backdrop and Mono material effects.
Artwork is original and attributed in `ATTRIBUTION.md`.

`Assets.car` is compiled by Apple's `actool` and supplies the layered icon on
macOS 26 and later. It is checked in so the existing release workflow can still
package on older Xcode runners. After editing and saving `SolidCraft.icon`,
regenerate the catalog with Xcode 26 or later from the repository root:

```sh
mkdir -p target/macos-icon
xcrun actool assets/SolidCraft.icon \
  --compile target/macos-icon --platform macosx \
  --minimum-deployment-target 11.0 --app-icon SolidCraft \
  --output-partial-info-plist target/macos-icon/icon-info.plist \
  --output-format human-readable-text
cp target/macos-icon/Assets.car assets/Assets.car
cp target/macos-icon/SolidCraft.icns assets/icon.icns
```

`icon.icns` is the older-macOS fallback generated by the same Apple compiler
from the approved composition, retaining its layer sizes and positions.
Regenerate both compiled resources together with the commands above.
`icon.vectorcraft`, `icon.svg`, `icon.png` and `icon.iconset/` retain the earlier
VectorCraft artwork; packaging uses the Icon Composer compilation instead.

Only `packaging/macos/package.sh` consumes these resources. Its existing signed
app/DMG workflow copies the catalog and ICNS into `Contents/Resources`, and
`Info.plist` references `SolidCraft` through `CFBundleIconName` and
`CFBundleIconFile`. The macOS viewport opts out of eframe's runtime bitmap so
Finder, the Dock and the application switcher use the bundle icon. Other
platforms retain the existing code-drawn icon and packaging.

Design references: [Apple app-icon HIG](https://developer.apple.com/design/human-interface-guidelines/app-icons)
and [Icon Composer documentation](https://developer.apple.com/documentation/xcode/creating-your-app-icon-using-icon-composer).
No Apple artwork is shipped.
