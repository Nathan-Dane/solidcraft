# UI artwork

The editable masters are Rust drawing definitions in `source/`, compiled only by `ui-egui`.
Use the existing `icons::paint` facade with semantic ink/fill/accent tokens on the 24 × 24 grid.
Unknown IDs retain the original fallback. Widget state, projection, picking and commands stay with their widgets.

`cargo xtask assets --generate icon.ui.sheet` writes labelled light/dark previews to `target/resources/ui-icons.png`.
Native Illustrator/VectorCraft work files were not found for this UI set. Add actual work files and linked inputs under `source/work/` when available, and declare/attribute each file.

| Source | Artwork | Consumers |
| --- | --- | --- |
| `icons.rs` | Toolbar, browser, menu and constraint icons | `icons::paint` |
| `glyphs.rs` | Brand mark, captions, transport, badges, pin, caret, disclosure, radial centre and removal cross | toolbar, titlebar, timeline, browser, menus |
| `navigation.rs` | Signed axes, cube face painting/labels and arrows | viewport/navigation cube |
| `markers.rs` | Constraint mappings/letters, snap markers, dimension arrowheads | sketch, assembly and dimensions |
| `pointer.rs` | Agent pointer and tag | agent cursor |
| `ids.rs` | Icon ID index for tools and reference checks | facade/review sheet |

Original artwork is MIT OR Apache-2.0; caption geometry retains VectorCraft attribution.

## Icon IDs

`accessibility`, `appearance`, `arc3`, `arc_center`, `arc_slot`, `arc_tangent`, `as_built`, `auto_constrain`, `axis`, `blend`, `body`, `boss`, `box`, `break`, `bug_report`, `c_coincident`, `c_collinear`, `c_concentric`, `c_equal`, `c_fix`, `c_hv`, `c_midpoint`, `c_parallel`, `c_perpendicular`, `c_polygon`, `c_smooth`, `c_symmetry`, `c_tangent`, `canvas`, `center_of_mass`, `centerline`, `chamfer`, `circle`, `circle2`, `circle3`, `circle_tt`, `circle_ttt`, `combine`, `component`, `compute`, `configuration_table`, `conic`, `constrainer`, `construction`, `convert_sheet`, `curvature_comb`, `curvature_map`, `cylinder`, `decal`, `delete`, `dimension`, `draft`, `draft_analysis`, `drive`, `dxf`, `ellipse`, `environment_map`, `export`, `extend`, `extrude`, `eye`, `fastener`, `fillet`, `finish`, `fit`, `fit_section`, `flange`, `flat`, `fold`, `folder`, `help`, `hem`, `hole`, `home`, `import`, `include`, `interference`, `intersect`, `intersection_curve`, `iso_analysis`, `iso_curve`, `joint`, `joint_origin`, `line`, `line_mid`, `lip`, `loft`, `lookat`, `measure`, `min_radius`, `mirror`, `moon`, `motion_link`, `move`, `new`, `normal`, `offset`, `open`, `orbit`, `origin`, `pan`, `params`, `patch`, `pattern_circ`, `pattern_rect`, `perspective`, `plane`, `plastic_assign`, `plastic_rule`, `point`, `polygon`, `presspull`, `project`, `project_surface`, `rect`, `rect3`, `rect_center`, `redo`, `refold`, `rest`, `revolve`, `rigid_group`, `save`, `scale`, `section`, `settings`, `sheet_rules`, `shell`, `sketch`, `sketch_chamfer`, `sketch_circ_pattern`, `sketch_fillet`, `sketch_mirror`, `sketch_rect_pattern`, `slot`, `snap`, `sphere`, `spin`, `spline`, `spline_cv`, `split`, `spun`, `stitch`, `style`, `sun`, `svg`, `sweep`, `text`, `thicken`, `torus`, `trim`, `undo`, `unfold`, `warning`, `zebra`, `zoom`

Existing fallback IDs are explicit in `source/ids.rs` and the manifest. They continue to draw the original unknown-icon symbol; the review sheet labels them as fallback. Add new IDs to the index when adding artwork.
