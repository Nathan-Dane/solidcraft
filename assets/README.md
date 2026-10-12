# SolidCraft application resources

Resource audit for [#433](https://github.com/bherbruck/solidcraft/issues/433),
against `main` at `8e0c3d2`. Paths below are current; destinations are proposed.
This revision inventories resources without relocating them.

## Files to collect

There are 21 current resource files and 13 historical icon files.
Paths are relative to the repository root. Each font's licence stays beside it.

| Resource / authoritative source | Current files | Proposed destination under `assets/` | Consumers |
| --- | --- | --- | --- |
| Approved macOS icon | `assets/SolidCraft.icon/icon.json`; `Assets/02-grid.svg`, `Assets/02-grid-dark.svg`, `Assets/03-solid-blue-cutout.svg` within that bundle | `icons/macos/source/SolidCraft.icon/` | Icon Composer, `actool` |
| Compiled macOS icons | `assets/Assets.car`, `assets/icon.icns`, generated together from the approved composition | `icons/macos/exports/` | `packaging/macos/package.sh` |
| Historical macOS artwork | `assets/icon.vectorcraft` (editable source), `assets/icon.svg`, `assets/icon.png`, all ten PNGs in `assets/icon.iconset/` | `icons/macos/history/` | No current packaging/runtime consumer; retain editable history separately, consider removing unused exports |
| Sketch text font | `crates/sketch/fonts/LiberationSans-Regular.ttf`, `OFL.txt` in that directory | `fonts/liberation-sans/` | `crates/sketch/src/text.rs` embeds the font; `xtask/src/licences.rs` reads its licence |
| Web Japanese UI font | `apps/solidcraft-web/fonts/BIZUDPGothic-Regular.ttf`, `OFL.txt` in that directory | `fonts/biz-udp-gothic/` | `apps/solidcraft-web/index.html` copies both with Trunk; `src/main.rs` fetches the font; `xtask/src/licences.rs` reads its licence |
| Offline translations | `crates/ui-egui/src/locales/en.json`, `de.json`, `ja.json`, `ko.json` in that directory | `locales/` | `crates/ui-egui/src/i18n.rs` embeds all four; desktop and web share the registry |
| macOS bundle and install text | `packaging/macos/Info.plist.in`, `packaging/macos/README.txt` | `platforms/macos/` | `packaging/macos/package.sh` substitutes the plist and copies the instructions |
| Linux / FreeBSD desktop metadata | `packaging/linux/ai.storyteller.solvecraft.desktop`, `ai.storyteller.solvecraft.mime.xml`, `ai.storyteller.solvecraft.metainfo.xml.in` in that directory | `platforms/linux/` | Linux and FreeBSD packaging, `flake.nix`, packaging lint; staged files feed Flatpak and Snap |
| Windows installer templates | `packaging/windows/solidcraft.wxs`, `packaging/windows/solidcraft.en-us.wxl` | `platforms/windows/` | `packaging/windows/package.ps1`, packaging lint |

Original artwork and translations use MIT OR Apache-2.0. Both bundled fonts
use SIL OFL 1.1, unmodified. Platform packaging templates are adapted from
PhotoCraft under Apache-2.0. [ATTRIBUTION.md](../ATTRIBUTION.md) records provenance;
add explicit rows for relocated locale catalogs and platform resources.

## Code and canonical files to index

| Resource | Authoritative location | Handling / consumers |
| --- | --- | --- |
| Shared app mark | `apps/app_icon.rs` | Candidate for `assets/icons/shared/app_icon.rs`; desktop, CLI and Windows `build.rs` include it directly; update their module paths and rebuild trigger |
| UI icons and glyphs | `crates/ui-egui/src/icons.rs`, `viewport.rs`, `timeline.rs`, `sketch_tools.rs` in that directory | Keep drawing code in its owning UI layer; inventory it here |
| Theme, appearances and rendering | `crates/ui-egui/src/theme.rs`, inline shaders in `gpu.rs`, `crates/doc/src/appearance.rs`, material definitions in `crates/doc/src/document.rs` | Keep original code in its owning layers; no external texture/environment artwork to relocate |
| Dependency fonts and shader | egui's built-in fonts; `vendor/egui-wgpu/src/egui.wgsl` | Keep dependency/vendor ownership; see `THIRD-PARTY-LICENSES.txt` and `ATTRIBUTION.md` |
| Optional desktop fonts | System font paths and `CRAFT_FONTS_DIR`, discovered by `crates/ui-egui/src/fonts.rs` | External resources; no files to collect |
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
