# UI icon status and workflow

How the icon redesign (PR #853) is organised, and how to see where every icon stands.

## Reviewing the icons: accept, comment, repeat

```bash
python3 assets/icons/ui/design/tools/icon_status.py --serve   # http://127.0.0.1:8770, saves design/review.json
python3 assets/icons/ui/design/tools/icon_status.py --notes   # the work list: icons with notes that are not final
python3 assets/icons/ui/design/tools/icon_status.py           # static page in target/icon-status.html (choices stay in the browser)
python3 assets/icons/ui/design/tools/icon_status.py --summary # counts only; --csv also rewrites design/inventory.csv
```

The page shows one card per icon id, grouped by where the app uses it: toolbar tab and panel (Solid ▸ Create, Sketch ▸ Constraints, …),
then icons the app shows elsewhere (browser, timeline, context menus), then commands that only appear in menus and the palette. A group
starts with a slim vertical marker in the same flow as the cards, so the grouping costs almost no space. Filters: All / Open / Notes /
Final, a tab filter, a search box, light/dark, and 16/24/48/64 px (the `small/` variant is used at 24 px and below).

- The check mark in a card's corner accepts the icon as final. Click it again to un-accept.
- The pencil opens a note; the text area unfolds over the page and takes no space when closed. A card with a note is outlined amber.
- `design/review.json` keeps `final` (id to the hash of its master SVG(s) at the time of accepting) and `notes`.
- A final icon is not touched again: `lint_icons.py` fails when a final icon's master differs from the stored hash, and
  `tools/author/gen.py` skips it. Un-accept it on the page to work on it.

The loop: the owner accepts some icons and writes notes on others; the agent runs `--notes`, redraws those icons using the notes (the page
refreshes to show them), and this repeats until every icon is final. Notes on an alias card (for example `surface_trim`) apply to its master drawn in
that accent.

Authoring aids: `tools/author/` holds the Python scripts that drew the masters (`lib.py` helpers, `i1.py`…`i8.py` icon functions, `gen.py` writes
the SVGs, `sheet.py` makes a large two-theme sheet). The SVGs remain the master; edit them directly if that is quicker.

## How the app draws icons

Command, menu, browser and timeline icons are these SVG masters. Nothing else draws them:
the old Rust icon drawing (`assets/icons/ui/source/icons.rs`) is gone.

- `crates/ui-egui/build.rs` embeds `svg/**`, `palette.toml` and `aliases.toml` at build time (a bad hex,
  an unknown accent or an alias to a missing master fails the build).
- `crates/ui-egui/src/icon_svg.rs` parses each master once and rasterises it with tiny-skia at the
  physical pixel size it is drawn, cached as a texture per (master, theme, accent, size). The
  `small/` variant is used at 24 px and below where one exists.
- `icons::paint(painter, rect, name, ..)` is unchanged for callers, but the ink, fill and accent colours
  they pass are ignored: the palette decides. Only the ink's opacity is kept, so dimmed buttons dim their
  icon. `icons::paint_on(.., Some(true), ..)` draws for a surface that is always dark (the application bar).
- A name with no master (and no alias) draws `placeholder`, the standard placeholder (a dotted frame
  around a plain block). Replacing it is just adding `svg/model/<icon-id>.svg`.

### Changing colours (one place)

Edit `assets/icons/ui/design/palette.toml`:

- `[roles.*]`: each colour role has a `light` and a `dark` hex. The SVGs store the light hex (or
  `currentColor` for the `line` role), so artwork never changes when a colour does.
- `[accents.*]`: an accent replaces the roles it lists, normally `active-1/2/3`. Proposed accents: `surface`
  (orange), `form` (purple), `mesh` (yellow), `sketch` (amber). Add an accent by adding a table.

### Same meaning, different accent

`aliases.toml` makes one icon id reuse another's artwork, optionally in an accent:
`surface_extrude = { master = "extrude", accent = "surface" }`. The five surface operations that have a
solid twin (extrude, revolve, sweep, loft, fillet) are done this way, so they stay identical to the solid
icon and differ only in colour. Add a row for any other pair (for example a mesh or form twin).

### Not replaced yet

The window caption buttons, transport controls, badges, pin, caret, disclosure arrow and the brand mark
(`glyphs.rs`), the navigation cube and axis tips, snap and dimension markers, and the agent pointer are
drawn controls and viewport graphics, not command icons, and are unchanged. The colour vision modes do
not reach the new icons yet.

## Artwork layout (`assets/icons/ui/design/`)

| Path | What |
|---|---|
| `svg/model/` (179) | Two-tone modelling, analysis, sketch and object masters |
| `svg/small/` (8) | Small-slot variants, only where the artwork differs (Section Analysis, Pattern, Interference, Joint, As-built, Isoparametric, Component, Curvature comb) |
| `svg/flat/` (19) | Single-colour application-bar and navigation icons, painted with `currentColor` |
| `svg/workspace/` (4) | Design, Animate, Render (planned), Drawing (planned) |
| `palette.toml` | Colour roles, each with a light and a dark hex, plus stroke weights |
| `aliases.toml` | App icon ids that reuse another master |
| `status.toml` | Masters that are stand-ins, to be redrawn |
| `inventory.csv` | Generated by `tools/icon_status.py --csv` |
| `tools/lint_icons.py` | Rules every master must pass |
| `tools/icon_status.py` | The status page (above) |
| `tools/preview.py` | Contact sheet in both themes at 16, 20, 26 and 64 px |

The SVGs are the single master; the earlier Python and JavaScript generators were retired.
The sources were the icon source handoff (authoritative) and the UX icon package; the mockup and
preview PNGs (5 MB) and the 1024 px brand `logo.svg` were not committed.

### Rules the artwork follows

- 64-unit grid, `viewBox="0 0 64 64"`, no `width`/`height`, no transforms, no text, no styles;
  only `path`, `circle`, `ellipse`, `g` (clip or opacity only), `defs`, `mask`, `clipPath`.
- Visible artwork centred on (32, 32) and at most 58 units across, so every icon has the same
  optical padding; strokes round-joined and round-capped; outlines 2.55, actions 4.25,
  navigation 5.6 (Parameters 4.2).
- Colour comes only from the roles in `palette.toml`. SVGs store the light hex; `currentColor` is the
  `line` role (line art sitting directly on the UI background, and every flat icon). Each role has a
  light and a dark hex, so dark mode is a role lookup, not different artwork.
- `gap` is the surface colour used for knock-outs between overlapping parts.
- File names are app icon ids; one master per concept, with `aliases.toml` for variants that share it.

## Other UI glyphs (not command icons)

| Source | Glyphs | Status |
|---|---|---|
| `glyphs.rs` | caret, disclosure | `flat/chevron.svg` covers them; not wired |
| `glyphs.rs` | brand mark, window caption buttons, transport (play/pause/step), badges, pin, radial centre, remove cross | not drawn |
| `navigation.rs` | navigation cube faces and labels, arrows, axis tips | not drawn (flat `orbit`, `pan`, `spin`, `fit`, `lookat`, `perspective` exist for the toolbar) |
| `markers.rs` | constraint letters, snap markers, dimension arrowheads | not drawn |
| `pointer.rs` | agent pointer and tag | not drawn |

## Workspaces

`ws_design` and `ws_animate` are drawn; `ws_render` and `ws_drawing` are drawn for planned workspaces. Surface, Assemble, Insert, Inspect, Manage and View tab icons are not drawn (see the tab lists in the UX package's `needed-icons.md`).

## Still open before implementation

- Format decision (Rust drawing code, SVG assets, or SVG authored then generated) and workspace-colour decision, per the proposal.
- Check any further icons that blur at 16 px in `tools/preview.py` (`combine`, `fastener`, `accessibility`, `environment_map` are the candidates left) and add a `small/` variant where it helps.
- Check every master in the running app at 16, 20 and 26 px in both themes; the dark values in `palette.toml` are a first proposal.
- Replace `gap` knock-outs (`as_built`, `center_of_mass`, `revolve`, `sweep`, `style`) with generated geometry so they stay correct on hover and selected backgrounds.
- Every command now has an icon id and a master (or an alias); `design/status.toml` is empty. Surface-tab icons are aliases in the `surface` accent (`surface_patch`, `surface_stitch`, …), and `error`/`warning` are aliases of `status_error`/`status_warning` in the `surface`/`mesh` accents.
