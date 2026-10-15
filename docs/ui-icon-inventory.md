# UI icon inventory

Where the icon redesign stands (PR #853), generated from the command registry, the icon ids the app
used before the redesign, and the artwork in `assets/icons/ui/design/`. The per-icon data is in
`assets/icons/ui/design/inventory.csv`.

**The application now draws the SVG masters** (see below). Every icon id with no master draws the
standard placeholder, so "stand-in" and "not drawn" below describe what still needs refining or drawing.
The step that remains is refining the existing masters and adding the missing ones.

## Summary

The app has 187 distinct icon ids (bound to the 368 commands, or in the pre-redesign icon set).

| Status | Icon ids | Commands using them | Meaning |
|---|---|---|---|
| Done | 50 | 66 | Final-style master exists (Technical Accent, flat navigation, v3 application bar) |
| Stand-in | 69 | 97 | Quick placeholder drawn in the handoff palette; to be redrawn as a master |
| Not drawn | 68 | 70 | No new artwork at all |

135 commands have no icon today and no toolbar place (browser, view and timeline housekeeping); they do not need one yet.

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
| `svg/model/` (96) | Two-tone modelling, analysis, sketch and object masters |
| `svg/small/` (1) | Small-slot variants, only where the artwork differs (Section Analysis) |
| `svg/flat/` (18) | Single-colour application-bar and navigation icons, painted with `currentColor` |
| `svg/workspace/` (4) | Design, Animate, Render (planned), Drawing (planned) |
| `palette.toml` | Colour roles, each with a light and a dark hex, plus stroke weights |
| `aliases.toml` | App icon ids that reuse another master |
| `inventory.csv` | Every app icon id with its status and artwork |
| `tools/lint_icons.py` | Rules every master must pass |
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

## Done (50 icon ids)

| App icon id | Commands | Artwork | Example command |
|---|---|---|---|
| `accessibility` | 1 | `svg/model/accessibility.svg` | `inspect.accessibility` |
| `as_built` | 1 | `svg/model/as_built.svg` | `joint.as_built` |
| `body` | 0 | `svg/model/body.svg` | – |
| `bug_report` | 0 | `svg/flat/bug_report.svg` | – |
| `center_of_mass` | 1 | `svg/model/center_of_mass.svg` | `inspect.center_of_mass` |
| `chamfer` | 1 | `svg/model/chamfer.svg` | `solid.chamfer` |
| `combine` | 1 | `svg/model/combine.svg` | `solid.combine` |
| `component` | 3 | `svg/model/component.svg` | `component.create` |
| `configuration_table` | 0 | `svg/model/configuration_table.svg` | – |
| `curvature_comb` | 1 | `svg/model/curvature_comb.svg` | `inspect.curvature_comb` |
| `curvature_map` | 1 | `svg/model/curvature_map.svg` | `inspect.curvature_map` |
| `draft_analysis` | 1 | `svg/model/draft_analysis.svg` | `inspect.draft` |
| `environment_map` | 1 | `svg/model/environment_map.svg` | `inspect.environment_map` |
| `extrude` | 1 | `svg/model/extrude.svg` | `solid.extrude` |
| `fastener` | 2 | `svg/model/fastener.svg` | `parts.fastener` |
| `fillet` | 1 | `svg/model/fillet.svg` | `solid.fillet` |
| `fit` | 0 | `svg/flat/fit.svg` | – |
| `folder` | 2 | `svg/model/folder.svg` | `browser.group` |
| `help` | 0 | `svg/flat/help.svg` | – |
| `home` | 0 | `svg/flat/home.svg` | – |
| `import` | 3 | `svg/model/import.svg` | `file.insert_step` |
| `interference` | 1 | `svg/model/interference.svg` | `inspect.interference` |
| `iso_analysis` | 1 | `svg/model/iso_analysis.svg` | `inspect.isocurve` |
| `joint` | 7 | `svg/model/joint.svg` | `joint.create` |
| `lookat` | 0 | `svg/flat/lookat.svg` | – |
| `measure` | 3 | `svg/model/measure.svg` | `component.properties` |
| `min_radius` | 1 | `svg/model/min_radius.svg` | `inspect.minimum_radius` |
| `moon` | 0 | `svg/flat/moon.svg` | – |
| `orbit` | 1 | `svg/flat/orbit.svg` | `view.navigate` |
| `pan` | 0 | `svg/flat/pan.svg` | – |
| `params` | 3 | `svg/model/params.svg` | `config.configure` |
| `perspective` | 0 | `svg/flat/perspective.svg` | – |
| `plane` | 9 | `svg/model/plane.svg` | `sketch.redefine` |
| `redo` | 1 | `svg/flat/redo.svg` | `edit.redo` |
| `revolve` | 1 | `svg/model/revolve.svg` | `solid.revolve` |
| `save` | 2 | `svg/flat/save.svg` | `file.save` |
| `scale` | 2 | `svg/model/scale.svg` | `solid.scale` |
| `section` | 1 | `svg/model/section.svg` | `inspect.section` |
| `settings` | 3 | `svg/flat/settings.svg` | `document.units` |
| `sketch` | 3 | `svg/model/sketch.svg` | `sketch.create` |
| `spin` | 0 | `svg/flat/spin.svg` | – |
| `style` | 0 | `svg/flat/style.svg` | – |
| `surface_extrude` | 1 | `svg/model/extrude.svg (alias of extrude, surface accent)` | `surface.extrude` |
| `surface_fillet` | 0 | `svg/model/fillet.svg (alias of fillet, surface accent)` | – |
| `surface_revolve` | 1 | `svg/model/revolve.svg (alias of revolve, surface accent)` | `surface.revolve` |
| `surface_sweep` | 1 | `svg/model/sweep.svg (alias of sweep, surface accent)` | `surface.sweep` |
| `sweep` | 1 | `svg/model/sweep.svg` | `solid.sweep` |
| `undo` | 1 | `svg/flat/undo.svg` | `edit.undo` |
| `zebra` | 1 | `svg/model/zebra.svg` | `inspect.zebra` |
| `zoom` | 0 | `svg/flat/zoom.svg` | – |

Notes:

- `plane` has two masters (`plane`, offset; `plane_angle`, at angle). All 9 plane commands share the offset plane today; the angled one (`construct.plane.at_angle`) should switch to `plane_angle`, and the command map asks for 9 plane variants in total.
- `redo` is the Undo master mirrored horizontally, stored as its own file.
- `save`, `help`, `home`, `folder` and `bug_report` use the rounder redesigns in preference to the handoff versions.
- Dark mode has `moon`; the matching `sun` is **not drawn**.

## Stand-ins, to redraw as masters (69 icon ids)

| App icon id | Commands | Artwork | Example command |
|---|---|---|---|
| `appearance` | 1 | `svg/model/appearance.svg` | `appearance.assign` |
| `arc3` | 1 | `svg/model/arc3.svg` | `sketch.arc.three_point` |
| `arc_center` | 1 | `svg/model/arc3.svg (alias of arc3)` | `sketch.arc.center_point` |
| `arc_tangent` | 1 | `svg/model/arc3.svg (alias of arc3)` | `sketch.arc.tangent` |
| `auto_constrain` | 2 | `svg/model/auto_constrain.svg` | `sketch.auto_constrain` |
| `axis` | 5 | `svg/model/axis.svg` | `construct.axis.through_cylinder` |
| `boss` | 1 | `svg/model/boss.svg` | `plastic.boss` |
| `box` | 4 | `svg/model/box.svg` | `solid.box` |
| `break` | 1 | `svg/model/break.svg` | `sketch.break` |
| `canvas` | 2 | `svg/model/canvas.svg` | `canvas.insert` |
| `circle` | 1 | `svg/model/circle.svg` | `sketch.circle.center` |
| `circle2` | 1 | `svg/model/circle.svg (alias of circle)` | `sketch.circle.two_point` |
| `circle3` | 1 | `svg/model/circle.svg (alias of circle)` | `sketch.circle.three_point` |
| `circle_tt` | 1 | `svg/model/circle.svg (alias of circle)` | `sketch.circle.two_tangent` |
| `circle_ttt` | 1 | `svg/model/circle.svg (alias of circle)` | `sketch.circle.three_tangent` |
| `coil` | 1 | `svg/model/coil.svg` | `solid.coil` |
| `construction` | 1 | `svg/model/construction.svg` | `sketch.construction` |
| `cylinder` | 1 | `svg/model/cylinder.svg` | `solid.cylinder` |
| `dimension` | 1 | `svg/model/dimension.svg` | `sketch.dimension` |
| `draft` | 1 | `svg/model/draft.svg` | `solid.draft` |
| `dxf` | 1 | `svg/model/dxf.svg` | `sketch.insert_dxf` |
| `ellipse` | 1 | `svg/model/ellipse.svg` | `sketch.ellipse` |
| `extend` | 2 | `svg/model/extend.svg` | `sketch.extend` |
| `eye` | 1 | `svg/model/eye.svg` | `browser.visibility` |
| `flange` | 1 | `svg/model/flange.svg` | `sheet.flange` |
| `flat` | 1 | `svg/model/flat.svg` | `sheet.flat_pattern` |
| `fold` | 1 | `svg/model/fold.svg` | `sheet.fold` |
| `hem` | 2 | `svg/model/hem.svg` | `sheet.perforate` |
| `hole` | 1 | `svg/model/hole.svg` | `solid.hole` |
| `include` | 1 | `svg/model/include.svg` | `sketch.include_3d` |
| `intersect` | 1 | `svg/model/intersect.svg` | `sketch.intersect` |
| `line` | 1 | `svg/model/line.svg` | `sketch.line` |
| `loft` | 1 | `svg/model/loft.svg` | `solid.loft` |
| `mirror` | 1 | `svg/model/mirror.svg` | `solid.mirror` |
| `move` | 3 | `svg/model/move.svg` | `solid.move` |
| `offset` | 1 | `svg/model/offset.svg` | `sketch.offset` |
| `offset_face` | 2 | `svg/model/offset_face.svg` | `solid.offset_face` |
| `origin` | 0 | `svg/model/origin.svg` | – |
| `pattern_circ` | 1 | `svg/model/pattern_circ.svg` | `solid.pattern.circular` |
| `pattern_rect` | 2 | `svg/model/pattern_rect.svg` | `solid.pattern.rectangular` |
| `plastic_assign` | 1 | `svg/model/plastic_assign.svg` | `plastic.assign_rule` |
| `plastic_rule` | 1 | `svg/model/plastic_assign.svg (alias of plastic_assign)` | `plastic.manage_rules` |
| `point` | 7 | `svg/model/point.svg` | `construct.point.vertex` |
| `polygon` | 3 | `svg/model/polygon.svg` | `sketch.polygon.inscribed` |
| `presspull` | 1 | `svg/model/presspull.svg` | `solid.press_pull` |
| `project` | 1 | `svg/model/project.svg` | `sketch.project` |
| `rect` | 1 | `svg/model/rect.svg` | `sketch.rectangle.two_point` |
| `rect3` | 1 | `svg/model/rect.svg (alias of rect)` | `sketch.rectangle.three_point` |
| `rect_center` | 1 | `svg/model/rect.svg (alias of rect)` | `sketch.rectangle.center` |
| `rect_rounded` | 1 | `svg/model/rect.svg (alias of rect)` | `sketch.rectangle.rounded` |
| `refold` | 1 | `svg/model/refold.svg` | `sheet.refold` |
| `rib` | 2 | `svg/model/rib.svg` | `solid.rib` |
| `sheet_rules` | 1 | `svg/model/sheet_rules.svg` | `sheet.manage_rules` |
| `shell` | 1 | `svg/model/shell.svg` | `solid.shell` |
| `sketch_circ_pattern` | 1 | `svg/model/pattern_circ.svg (alias of pattern_circ)` | `sketch.pattern.circular` |
| `sketch_fillet` | 1 | `svg/model/sketch_fillet.svg` | `sketch.fillet` |
| `sketch_mirror` | 1 | `svg/model/sketch_mirror.svg` | `sketch.mirror` |
| `sketch_rect_pattern` | 1 | `svg/model/pattern_rect.svg (alias of pattern_rect)` | `sketch.pattern.rectangular` |
| `slot` | 3 | `svg/model/slot.svg` | `sketch.slot.center_to_center` |
| `snap` | 1 | `svg/model/snap.svg` | `plastic.snap_fit` |
| `sphere` | 1 | `svg/model/sphere.svg` | `solid.sphere` |
| `spline` | 1 | `svg/model/spline.svg` | `sketch.spline.fit_point` |
| `spline_cv` | 2 | `svg/model/spline.svg (alias of spline)` | `sketch.spline.control_point` |
| `split` | 2 | `svg/model/split.svg` | `solid.split_face` |
| `surface_loft` | 1 | `svg/model/loft.svg (alias of loft, surface accent)` | `surface.loft` |
| `text` | 1 | `svg/model/text.svg` | `sketch.text` |
| `thread` | 1 | `svg/model/thread.svg` | `solid.thread` |
| `trim` | 2 | `svg/model/trim.svg` | `sketch.trim` |
| `unfold` | 1 | `svg/model/unfold.svg` | `sheet.unfold` |

New ids with artwork but no app icon yet: `align`, `bendtable`, `chain`, `chevron`, `dots`, `filter`, `nav_settings`, `pattern`, `placeholder`, `plane_angle`, `select`, `sheet`, `surface`, `ws_animate`, `ws_design`, `ws_drawing`, `ws_render`
(`dots` is the overflow menu, `chain` a link, `select` the pointer, `sheet` a sheet-metal part, `surface` a generic surface, `plane_angle` the angled plane, `ws_*` the workspaces, `chevron` and `nav_settings` navigation).

## Not drawn (68 icon ids)

| App icon id | Commands | Example command |
|---|---|---|
| `arc_slot` | 2 | `sketch.slot.arc_three_point` |
| `blend` | 1 | `sketch.blend_curve` |
| `boundary` | 0 | – |
| `boundary_fill` | 1 | `solid.boundary_fill` |
| `c_coincident` | 1 | `sketch.constraint.coincident` |
| `c_collinear` | 1 | `sketch.constraint.collinear` |
| `c_concentric` | 1 | `sketch.constraint.concentric` |
| `c_equal` | 1 | `sketch.constraint.equal` |
| `c_fix` | 1 | `sketch.constraint.fix` |
| `c_hv` | 1 | `sketch.constraint.horizontal_vertical` |
| `c_midpoint` | 1 | `sketch.constraint.midpoint` |
| `c_parallel` | 1 | `sketch.constraint.parallel` |
| `c_perpendicular` | 1 | `sketch.constraint.perpendicular` |
| `c_polygon` | 1 | `sketch.constraint.polygon` |
| `c_smooth` | 1 | `sketch.constraint.curvature` |
| `c_symmetry` | 1 | `sketch.constraint.symmetry` |
| `c_tangent` | 1 | `sketch.constraint.tangent` |
| `camera` | 2 | `view.save` |
| `capture` | 1 | `component.capture_position` |
| `centerline` | 1 | `sketch.centerline` |
| `compute` | 1 | `timeline.compute_all` |
| `conic` | 1 | `sketch.conic` |
| `constrainer` | 1 | `sketch.constrainer` |
| `continuity` | 0 | – |
| `convert_sheet` | 1 | `sheet.convert` |
| `decal` | 1 | `canvas.decal` |
| `delete` | 4 | `selection.delete` |
| `drive` | 1 | `joint.drive` |
| `emboss` | 1 | `solid.emboss` |
| `export` | 2 | `file.export` |
| `finish` | 2 | `sketch.finish` |
| `fit_section` | 1 | `sketch.fit_curves_to_section` |
| `intersection_curve` | 1 | `sketch.intersection_curve` |
| `iso_curve` | 1 | `sketch.isoparametric_curve` |
| `joint_origin` | 1 | `joint.origin` |
| `line_mid` | 1 | `sketch.line.midpoint` |
| `lip` | 1 | `plastic.lip` |
| `material` | 5 | `material.assign` |
| `motion_link` | 1 | `joint.motion_link` |
| `new` | 1 | `file.new` |
| `normal` | 0 | – |
| `offset_one_side` | 0 | – |
| `offset_symmetric` | 0 | – |
| `offset_two_sides` | 0 | – |
| `open` | 1 | `doc.open` |
| `patch` | 1 | `surface.patch` |
| `pipe` | 1 | `solid.pipe` |
| `project_surface` | 1 | `sketch.project_to_surface` |
| `rest` | 1 | `plastic.rest` |
| `reverse_normal` | 1 | `surface.reverse_normal` |
| `revert` | 1 | `component.revert_position` |
| `rigid_group` | 2 | `component.unlock` |
| `ruled` | 1 | `surface.ruled` |
| `sketch_chamfer` | 3 | `sketch.chamfer.equal_distance` |
| `sketch_chamfer_angle` | 0 | – |
| `sketch_chamfer_equal` | 0 | – |
| `sketch_chamfer_two` | 0 | – |
| `spun` | 1 | `sketch.spun_profile` |
| `stitch` | 1 | `surface.stitch` |
| `sun` | 0 | – |
| `surface_blend` | 0 | – |
| `surface_offset` | 1 | `surface.offset` |
| `svg` | 1 | `sketch.insert_svg` |
| `thicken` | 1 | `surface.thicken` |
| `torus` | 1 | `solid.torus` |
| `unstitch` | 1 | `surface.unstitch` |
| `untrim` | 1 | `surface.untrim` |
| `warning` | 0 | – |

Groups: sketch constraints (`c_*`, 12), surface operations (`surface_*`, `patch`, `ruled`, `stitch`, `thicken`, `untrim`, ...),
assembly (`rigid_group`, `joint_origin`, `motion_link`, `drive`, `capture`, `revert`), sketch curves and chamfers
(`conic`, `centerline`, `arc_slot`, `sketch_chamfer*`, `line_mid`), offsets (`offset_*`), files (`new`, `open`, `export`, `svg`, `delete`),
materials (`material`, `decal`, `camera`), plastic (`lip`, `rest`), and status (`error`, `warning`, `update`, `sun`).

### Icon ids commands use that the app does not draw at all

These fall back to the generic icon today: `boundary_fill`, `camera`, `capture`, `coil`, `emboss`, `material`, `offset_face`, `pipe`, `revert`, `rib`, `thread`.
Stand-ins exist for `coil`, `offset_face`, `rib` and `thread`; `boundary_fill`, `camera`, `capture`, `emboss`, `material`, `pipe` and `revert` have nothing.

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
- Simplified `small/` variants for icons that blur at 16 px in `tools/preview.py`: `pattern`, `interference`, `iso_analysis`, `joint`, `as_built`, `combine`, `component`, `fastener`, `accessibility`, `environment_map`, `curvature_comb`.
- Check every master in the running app at 16, 20 and 26 px in both themes; the dark values in `palette.toml` are a first proposal.
- Replace `gap` knock-outs (`as_built`, `center_of_mass`, `revolve`, `sweep`, `style`) with generated geometry so they stay correct on hover and selected backgrounds.
- Redraw the stand-ins; draw the not-drawn ids; port to the chosen pipeline and update `ATTRIBUTION.md`/`cargo xtask assets`.
