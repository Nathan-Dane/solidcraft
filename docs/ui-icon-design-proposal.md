# UI icon redesign considerations

**Status: draft for discussion. No format, style or colour palette has been selected.**

SolidCraft's UI icons currently provide working command symbols, but they need a
consistent, professional visual language. This proposal establishes the decisions
and review criteria for replacing them. It contains no replacement artwork or
implementation changes.

Progress on the artwork is tracked in [ui-icon-inventory.md](ui-icon-inventory.md); the collected
masters are in `assets/icons/ui/design/`.

The resource organisation in [PR #833](https://github.com/bherbruck/solidcraft/pull/833)
provides the intended home for the eventual icon masters and exports. The redesign
should use that structure once it lands.

## Aims

- Make command icons recognisable at toolbar, menu, browser and timeline sizes.
- Establish consistent geometry, visual weight, spacing and treatment of detail.
- Make related operations look related while keeping their meanings distinct.
- Coordinate colours across modelling workspaces, themes and interaction states.
- Keep the artwork original, maintainable and suitable for desktop and wasm.

## Decision 1: Rust or SVG

Compare the current Rust vector drawing approach with an SVG asset workflow using
the same small set of independently drawn candidate icons. A format does not
itself determine visual quality.

| Consideration | Rust vector drawing | SVG assets |
| --- | --- | --- |
| Authoring | Geometry is edited in code; shared drawing helpers can enforce consistency | Vector editors can make visual iteration easier; preserve editable masters and controlled exports |
| Theme and state | Existing `icons::paint` receives ink, fill and accent from its callers | Define how the loader or generation step applies semantic colours and disabled/selected states |
| Rendering | Uses the existing egui painter pipeline | Compare direct vector rendering, cached rasterisation and build-time conversion; record dependencies and desktop/wasm support |
| Maintenance | Review geometry and shared helper changes alongside Rust | Review artwork, exports, icon-ID bindings, validation and regeneration together |
| Cost | Measure build time and drawing cost for the redesigned set | Measure load time, drawing cost, cache/memory use and scaling behaviour for the proposed pipeline |

An SVG authoring workflow that generates Rust geometry is also worth evaluating,
if it offers useful editing benefits without creating two competing masters.
Decide which source is authoritative, how changes reach the application, and how
missing or malformed artwork is reported. Record the trade-offs before selecting
a pipeline; retain the existing command IDs and UI rendering boundary.

## Decision 2: General style

Evaluate a small, coherent sample in each candidate style:

- Outlined icons with restrained fills or two-colour accents.
- Filled geometric icons with simplified three-dimensional faces.
- Monochrome icons, with workspace identity expressed elsewhere in the UI.

Compare them in the actual application. Include Create Sketch, Extrude, Revolve,
Fillet, Chamfer, a surface operation, a mesh operation, and shared commands such
as Move, Measure and Settings. Form/T-Spline examples can remain design samples;
this proposal does not add modelling features or a new workspace.

The chosen style should specify the drawing grid, stroke weights, corner and
line-end treatment, perspective, padding, optical alignment, arrows, control
points and the maximum useful amount of detail. Test common small sizes and UI
scaling in both light and dark themes. Include enabled, disabled, hovered and
selected states, and check symbols in monochrome so meaning survives colour loss.

## Decision 3: Workspace colour coordination

Decide whether modelling families should have their own icon colours. The
requested mapping is a candidate for discussion:

| Modelling family | Proposed colour family | Exact colours |
| --- | --- | --- |
| Solid | Blue | Undecided |
| Surface | Orange | Undecided |
| T-Spline / Form | Purple | Undecided |
| Mesh | Yellow | Undecided |

Questions to resolve:

- Adopt this mapping, choose another mapping, or keep icons largely neutral?
- Apply family colours to faces/fills, outlines, action accents, or a combination?
- Does colour follow the geometry type or the active workspace for shared tools?
- How should Sketch, Sheet Metal, Plastic and shared utilities fit into the system?
- Which exact light/dark colour values, neutral inks and fill intensities work on
  toolbar, menu and timeline backgrounds? Evaluate yellow especially on light
  backgrounds and keep orange distinguishable from construction geometry.
- How do workspace colours coexist with selection, warnings, errors, success,
  previews and disabled states? Colour must not be the only indicator of meaning.

If family colours are adopted, record the exact colour values and state variants
as semantic theme tokens, with representative swatches and application examples.
The blue/orange/purple/yellow mapping is not an approved palette, and the reference
image is not a source of colour values to extract.

## Reference image

The user-supplied image is attached to the draft PR discussion for reference. It
illustrates the proposed family grouping and provides context for discussing
filled versus outlined geometry. It does not select SolidCraft's final style,
icon shapes, palette or workspace layout.

Keep the screenshot outside the repository. SolidCraft's eventual icon artwork
must be original or appropriately openly licensed and attributed; do not copy
commercial icon artwork or extract assets from the reference.

## Outputs required before implementation

- [ ] Record the chosen format/pipeline and its rationale.
- [ ] Approve a general style using original comparative samples.
- [ ] Decide whether to adopt workspace colour coordination.
- [ ] If adopted, approve exact colours, theme variants and interaction-state rules.
- [ ] Define source ownership, attribution, exports and the editing workflow.
- [ ] Review samples in the actual app at small sizes in both themes.
- [ ] Agree the replacement scope and rollout, including currently missing icons.

Replacement artwork and renderer changes should follow these decisions in a
separate implementation PR, with before/after application screenshots and the
repository's normal quality gates.
