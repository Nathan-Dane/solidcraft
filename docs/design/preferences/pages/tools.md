# Tools

Board: `Prefs2_Tools` (1920x1033). Heading: Work. Subtitle "How modelling tools behave." No "In
profile" tag. Sources today: Design (Hole, Extrude, Placement, Assembly Move), Navigation (Direct
interaction, tool at cursor), Graphics (axis distances in Measure).

## Create

One bordered card holds three rows (canvas version 212). Each row has the tool's real icon in a 36 pt rounded tile (`pref_well`) with the icon at 28 pt, the tool name (13 px / 600) beside it, and the tool's checkboxes in the right column (the left column is 290 pt wide, rows are separated by a 1 px `dialog_rule`, padding 10 14). The icons are the app's own: `hole`, `extrude`, and `rest`, `boss`, `snap_fit` (three tiles) from `assets/icons/ui/design/svg/model`, drawn with the icon palette for the theme. The earlier 120x64 figures (`tool-hole`, `tool-extrude`, `tool-placement`) are no longer used on this page.

| Label | Control | Edits | Default | Hover | Figure | Notes |
|---|---|---|---|---|---|---|
| Hole: Snap to centres | Checkbox | `Prefs.hole_snaps` | `false` | "The middle of a face and the centres of its round edges." | icon `hole` | Today's label "Snap to face and edge centres". |
| Hole: Move by click or drag | Checkbox | `Prefs.hole_moves` | `false` | "Picking the face again moves the hole instead of removing it." | icon `hole` | Today's label "Move holes by clicking or dragging". |
| Extrude: Extrude to a picked target | Checkbox | `Prefs.extrude_auto_to_object` | `false` | "A face, vertex or parallel plane switches the extent to To Object." | icon `extrude` | |
| Extrude: Choose the operation automatically | Checkbox | `Prefs.auto_operation` | `true` | "Join, New Body or Cut, from what the material touches." | icon `extrude` | |
| Rest, Boss, Snap Fit: Placement handles | Checkbox | `Prefs.placement_handles` | `false` | "A marker to drag the feature across the model and a ring to turn it." | icons `rest`, `boss`, `snap_fit` | |

## Assembly Move

| Label | Control | Edits | Default | Hover | Figure | Notes |
|---|---|---|---|---|---|---|
| Use joint motion handles for Move | Checkbox | `Prefs.assembly_move_handles` | `true` | "Off: the previous Move panel." | `assembly-move` | |
| Show moved parts' old place as a ghost | Checkbox | `Prefs.drive_joints_ghost` | `true` | none | `assembly-move` | |
| Near-axis step | Number field (64 wide, aria "Degrees") with "deg" | `Prefs.joint_rotation_coarse` (0.001 to 180) or `joint_rotation_fine`: see note | `10` (coarse) / `0.1` (fine) | "Pull outside the dashed guide for the fine step." | `assembly-move` | The board's hover says pulling outside the dashed guide uses the fine step, so the "Near-axis" field edits the step used near the axis and the "Far-axis step" the other. In `joint_handles.rs` the coarse step applies unless `d.fine` is set. Assumed mapping: Near-axis = `joint_rotation_coarse`, Far-axis = `joint_rotation_fine`; confirm with the design reviewer before building. |
| Far-axis step | Number field with "deg" | `Prefs.joint_rotation_fine` (assumed) | `0.1` | none | `assembly-move` | |

## Measure and pointer

Three toggle tiles (208.8 x 140).

| Label | Control | Edits | Default | Hover | Figure | Notes |
|---|---|---|---|---|---|---|
| Axis distances in Measure | Toggle tile (the figure shows "X 46", "Y 28") | `Prefs.measure_axis_distances` | `true` | none | `measure-axis-distances` | Moves from Graphics. |
| Tool at the cursor | Toggle tile | `app.ui.tool_cursor` (session; in the Snapshot) | per session | none | `tool-cursor` | Moves from Navigation. |
| Direct interaction | Toggle tile (the figure shows "+18 mm") | `Prefs.direct_interaction` | `false` | none (today: "Work on the model directly in the viewport. Press a flat face or a profile and pull: up extrudes, down into a body cuts, and sideways movement sets the step size") | `direct-interaction` | Moves from Navigation. The board has no hover on the tile; keep the existing help text as the tooltip. |

## Mismatches

* The Near-axis and Far-axis mapping above is an assumption (see the row).
* Create uses app icons, not figures; Assembly Move and Measure and pointer keep their figures.
