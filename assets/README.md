# SolidCraft application resources

Resource audit for [#433](https://github.com/bherbruck/solidcraft/issues/433),
against `main` at `2bb24d1`. Paths below are current; destinations are proposed.
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
    app/
      shared/
        source/app_icon.rs              # common non-macOS artwork master
        source/work/                   # native work files, when available
      macos/
        source/SolidCraft.icon/         # approved Composer project + SVG layers
        exports/Assets.car
        exports/icon.icns
      windows/
        exports/solidcraft-executable.ico # current DIB sizes/transparency mask
        exports/solidcraft.ico          # current PNG-based installer ICO
      linux/
        exports/hicolor/               # 16–512 px, existing desktop icon name
      freebsd/
        README.md                      # uses Linux hicolor exports, no duplicate
      web/
        README.md                      # browser resource bindings; no favicon yet
    ui/
      README.md                        # icon/glyph index and design conventions
      source/
        icons.rs                       # collected 24 × 24 Rust drawing library
        glyphs.rs                      # caption, timeline, badge/menu artwork
        navigation.rs                  # axis/cube artwork and arrows
        markers.rs                     # constraint/snap/dimension artwork
        pointer.rs                     # original agent pointer
        work/                          # native design documents, when available
  branding/
    web/source/og.png                   # original shared web/docs preview card
    web/source/work/                    # editable documents, when available
  styles/
    ui/palettes.rs                      # original palette/token defaults
    web/loading.css                    # current web loading/canvas styles
  appearances/library.rs               # built-in model appearance presets
  materials/library.rs                 # built-in physical material data/colours
  shaders/viewport.wgsl                 # original viewport shader
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
    freebsd/README.md                   # shares freedesktop metadata with Linux
    web/README.md                       # HTML/Trunk host and public URL bindings
  archive/
    icons/
      macos-v1/
        source/icon.vectorcraft        # original editable work, retained
        exports/                       # previous SVG, PNG and icon.iconset/
```

The tree is the migration target; `manifest.json` is not implemented yet.
Collect artwork masters, including Rust-drawn icons, under `assets/icons/`.
Compile them through their owning layer's existing interface: UI rendering stays
in `ui-egui`, and the shared app mark serves desktop, CLI and packaging builds.
Build logic stays in `packaging/` and `xtask/`. Root licence files stay canonical.

### All application icon platforms

All artwork and required package exports have a home under `icons/app/`.
Windows, Linux and FreeBSD use the **same** shared Rust master; platform folders
hold the format-specific exports and bindings. Preserve existing artwork and
both Windows ICO encodings. Generate in `target/resources/`, then commit the
required package exports alongside their recorded source/output digests.
Packaging and Nix copy these canonical exports rather than generating separate
artwork in their own folders. Temporary files, review sheets and package copies
remain in ignored build directories; Cargo build scripts still write only to
`OUT_DIR`.

| Platform / use | Canonical source or export | Existing consumer to update |
| --- | --- | --- |
| Windows executable, taskbar and switcher | `icons/app/windows/exports/solidcraft-executable.ico`; preserve current DIB representations at 16, 24, 32, 48, 64, 128, 256 px | `apps/solidcraft/build.rs` embeds the collected export; desktop runtime uses the shared master at 64 px |
| Windows installer and shortcuts | `icons/app/windows/exports/solidcraft.ico`; preserve current PNG representations at 16, 20, 24, 32, 40, 48, 64, 128, 256 px | `packaging/windows/package.ps1`, WiX `IconPath`, `SOLIDCRAFT_ICONS` override |
| Linux desktop, Nix, deb/rpm, tar, Flatpak and Snap | `icons/app/linux/exports/hicolor/<size>x<size>/apps/ai.storyteller.solidcraft.png`, sizes 16, 24, 32, 48, 64, 128, 256, 512 | `packaging/env.sh::make_icons`, Linux packaging, Flatpak/Snap staging, `flake.nix`; desktop runtime uses the shared master |
| AppImage | Alias/copy of the Linux 256 px export; no second artwork master | `packaging/linux/package.sh` root PNG and `.DirIcon`; `AppRun` installs the same hicolor files |
| FreeBSD desktop and packages | References the Linux hicolor exports and shared master | `packaging/freebsd/package.sh`; preserve installed `share/icons/hicolor` paths |
| macOS Finder, Dock and switcher | Composer project, `Assets.car`, matching `icon.icns` under `icons/app/macos/` | macOS packaging and bundle `CFBundleIconName`/`CFBundleIconFile`; retain precompiled exports for older Xcode runners |
| Web application | Shared UI icon collection; `branding/web/source/og.png` for social/link previews | `apps/solidcraft-web/index.html`, docs `head.hbs`, Pages build. No favicon or PWA icon currently exists; record that absence, do not invent shipped files |

The generic 1024 px PNG currently emitted by `make_icons` has no identified
package consumer. Keep this optional CLI export under `target/resources/`,
indexed as a generated utility output rather than another production master.

### UI icon collection

`icons/ui/source/icons.rs` becomes the canonical source of the existing toolbar,
browser, palette, menu and constraint icons. Preserve their IDs, 24 × 24 design
grid, original geometry and semantic ink/fill/accent colours. Keep
`crates/ui-egui/src/icons.rs` as the small rendering facade so existing
`icons::paint(...)` consumers continue to use one stable interface.

Collect reusable timeline transport, title-bar caption, pin/lock, disclosure and
menu artwork in `source/glyphs.rs`; navigation artwork in `navigation.rs`,
constraint/snap/dimension markers in `markers.rs`, and the agent pointer in
`pointer.rs`. Extract only drawing
definitions from their current widgets; widget state, layout, picking, camera
projection and commands remain in the UI. Include navigation-axis/cube artwork
and the original agent-pointer mark in this collection, with their consumers
recorded in the manifest. The UI layer compiles these masters; lower layers gain
no egui dependency.

`icons/ui/README.md` indexes every icon/glyph ID, source, consumer and licence.
Keep available native work files in `source/work/`; the current UI icon masters
are Rust drawing code, with no separate Illustrator/VectorCraft projects found.
Generate a labelled light/dark icon sheet under `target/` for design review,
using the same drawing masters and theme tokens as the app. Review exports are
derived previews; shipped UI icons continue to be drawn in Rust.

Validate command icon references and direct UI icon IDs against the collection,
preserve the existing fallback for unknown IDs, and attribute collected artwork
with its existing provenance. Do not duplicate masters in the facade or in
generated preview images.

### Other application design resources

| Resource to collect | Current authoritative source | Destination / consumer boundary |
| --- | --- | --- |
| UI palettes and default visual tokens | Palette/default values in `crates/ui-egui/src/theme.rs` | `styles/ui/palettes.rs`; `theme.rs` retains theme selection and its public `Tokens` interface |
| Web loading/canvas styling | Inline CSS in `apps/solidcraft-web/index.html` | `styles/web/loading.css`; the app HTML/Trunk entry point remains with its host and references this stylesheet |
| Model appearance presets | `LIBRARY` in `crates/doc/src/appearance.rs` | `appearances/library.rs`; document appearance behavior stays in `doc` |
| Physical material defaults | Material names, densities and default colours in `crates/doc/src/document.rs` | `materials/library.rs`; user-defined materials remain in user designs; no UI dependency in model data |
| Viewport shader | Inline `SHADER` in `crates/ui-egui/src/gpu.rs` | `shaders/viewport.wgsl`; GPU pipeline creation and runtime bindings stay in the UI renderer |
| Shared web/docs preview branding | `docs/book/src/og.png`, also referenced by the web app | `branding/web/source/og.png`; copy to the existing published `/solidcraft/og.png` path. Preserve the existing PNG when no editable master is available |

Root font/locales/platform directories below collect both bundled fonts and
licences, all four translation catalogs, macOS plist/install text, freedesktop
desktop/MIME/AppStream data and Windows installer/localization templates.
FreeBSD reuses the Linux metadata; web records its Trunk/HTML consumers. Retain
all native design documents and their linked inputs when available, including
the existing legacy VectorCraft document in the archive.

Dependency-owned fonts (Ubuntu Light, Hack, Noto Emoji and the emoji icon font)
and both vendored renderer shaders (`egui.wgsl`, `texture_copy.wgsl`) stay with
their dependencies and are explicitly indexed with their licences. System fonts,
optional font packs, user-imported fonts, canvases/decals and user material data
are external/user resources, not copies to add to the application collection.
OS-provided pointers, title-bar controls and native dialogs are system resources;
index their host bindings without copying operating-system artwork.

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
repository root. Proposed `cargo xtask assets --generate icon.app.macos` and
`--generate icon.app.windows` stage outputs in `target/resources/`, then update
the required checked-in exports and their source/output digests. Linux generation
also serves FreeBSD; UI preview generation uses the collected drawing library.
Record every platform's formats, sizes, producer, aliases and consumers in the
manifest. The existing commands below remain usable until those helpers exist.

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
paths and consumers. Record output formats/sizes, installed/public paths,
shared-resource aliases and ownership (`repository`, `dependency`, `system` or
`user`). Explicitly distinguish committed package exports from temporary outputs
and absent resources. Paths are repository-relative and exact; collected Rust
masters record their compiling layer and facade. `ATTRIBUTION.md` remains the
provenance record; font licences remain beside their fonts.

Extend `cargo xtask assets` to check every declared file and consumer, unique IDs,
full coverage of `assets/` (with explicit documentation exemptions), attribution,
Composer layer references and source/export separation. Include native work
formats, catalogs and metadata; reject missing or undeclared resources. Packaging
must fail if a required export is absent. Recorded digests detect changed sources
or exports on every platform. Validate macOS source/export agreement
with `actool` where available; do not compare timestamp-bearing catalogs bytewise
or require Apple tools on other platforms.

## Files to collect

There are 22 current non-code resource files (including the shared preview card)
and 13 historical icon files, plus the Rust artwork, presets, inline shader/CSS
and generated platform exports described above.
Paths are relative to the repository root. Each font's licence stays beside it.

| Resource / authoritative source | Current files | Proposed destination under `assets/` | Consumers |
| --- | --- | --- | --- |
| Approved macOS icon | `assets/SolidCraft.icon/icon.json`; `Assets/02-grid.svg`, `Assets/02-grid-dark.svg`, `Assets/03-solid-blue-cutout.svg` within that bundle | `icons/app/macos/source/SolidCraft.icon/` | Icon Composer, `actool` |
| Shared app mark | `apps/app_icon.rs` | `icons/app/shared/source/app_icon.rs` | Desktop and CLI icon APIs; Windows build script; packaging icon generation |
| UI icon library | `crates/ui-egui/src/icons.rs` | `icons/ui/source/icons.rs` | Stable `ui-egui::icons::paint` facade; toolbar, browser, palette, menus and constraint tools |
| Other UI glyph artwork | Drawing definitions in `timeline.rs`, `titlebar.rs`, `toolbar.rs`, `browser.rs`, `frame.rs`, `context_menu.rs`, `sketch_tools.rs`, `dim_view.rs`, `dialogs_assembly.rs`, `viewport.rs`, `navigation_cube.rs`, `agent_cursor.rs` under `crates/ui-egui/src/` | `icons/ui/source/{glyphs,navigation,markers,pointer}.rs`, with native work files in `source/work/` if available | Existing widgets through the UI rendering facade; behaviour and layout stay with the widgets |
| Compiled macOS icons | `assets/Assets.car`, `assets/icon.icns`, generated together from the approved composition | `icons/app/macos/exports/` | `packaging/macos/package.sh` |
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
| Icon rendering interfaces | `apps/app_icon.rs`, `crates/ui-egui/src/icons.rs` | Keep stable consumer interfaces; collect artwork under `assets/icons/app/shared/` and `assets/icons/ui/` |
| Theme, appearances and rendering | Owning modules listed above | Collect static design data/shader source; retain selection, document behavior and GPU bindings in their owning layers |
| Dependency fonts and shaders | egui's built-in fonts; `vendor/egui-wgpu/src/{egui,texture_copy}.wgsl` | Keep dependency/vendor ownership; see `THIRD-PARTY-LICENSES.txt` and `ATTRIBUTION.md` |
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
  Include Windows executable/installer icon bindings, Linux/FreeBSD/Nix icon
  copies, AppImage aliases, the web stylesheet and web/docs preview-card copying.
- Extend `xtask/src/assets.rs` beyond its current extension-based attribution scan:
  cover `.car`, `.vectorcraft`, Composer `icon.json`, catalogs and metadata;
  reject missing referenced files and unaccounted application resources.
- Validate with `cargo xtask ci`, desktop/web font and locale loading, Windows
  embedded icons, Linux packaging, and signed macOS bundles with both icon formats.
  Retain checked-in macOS compilations for older Xcode runners.

Build logic stays in `packaging/` and `xtask/`, including nfpm, Flatpak and Snap
manifests, AppRun and install scripts. The web HTML/Trunk config remains its build
entry point; mdBook's templates/config remain documentation build logic. The
shared preview card is collected as branding because the web app also uses it.
Documentation media (`docs/screenshots/`, generated book images),
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
