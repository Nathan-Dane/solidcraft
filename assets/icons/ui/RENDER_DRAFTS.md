# Render workspace icon drafts

All ids below are **Open** for owner review. No commands, toolbar entries, browser
selectors or runtime request manifests are changed by this draft set. The four
new aliases reuse identical finished meanings; accepted source masters stay unchanged.
Other operations have distinct original SVG masters in `design/svg/model/`.

Review with `python3 -B assets/icons/ui/design/tools/icon_status.py --serve`,
search `render_`, and select Open. See [STYLE_GUIDE.md](STYLE_GUIDE.md) for
acceptance; drafts do not imply implemented commands.

The two finished references in each row were read and previewed in both themes
before authoring. The initial reference previews and draft contact sheets (96, 16 and 24 px)
are local evidence under `target/render-icon-review/`, not shipped assets.

The six **Lights** drafts have been remade again after owner feedback, using
original analytic geometry. The supplied external images informed only the idea
of simple source silhouettes and dotted direction stems; they were not traced or
copied and are not repository assets. The finished SolidCraft references below
remain the drawing and palette authority. No light draft is final.

The family uses light passive fills, slate outlines at **2.55** units, and a
single accent cue. Each stem has **three dots**, diameter **4.25**, spaced **7**
units centre-to-centre. Point, spot, area and solo use identical stem centres:
`(32,44)`, `(32,51)`, `(32,58)`. Highlight placement keeps the same vertical axis
and spacing, with dots at y = 9, 16, 23 arriving from above its light-side target.
Sun replaces the stem with eight short radial rays at the same 4.25-unit weight.
Point, sun, highlight and solo share 15-unit source radii. The cone and flat panel
have comparable optical size; the panel follows the canonical 30° axes without
thickness or extra edges. Solo adds only focus brackets to the exact point glyph.

The revised lights were inspected individually and side by side at 96, 24 and
16 px in both themes. Current local evidence is `/tmp/icon-groups/lights.png`
and `/tmp/icon-groups/lights-dark.png`; the original set and finished references
are `/tmp/icon-groups/before-all{,-dark}.png` and
`/tmp/icon-groups/references{,-dark}.png`. These PNGs are review evidence only.

| Meaning | New id | Finished references | Drawing / reuse |
|---|---|---|---|
| Appearance | `render_appearance` | `appearance`, `material` | Alias of appearance: assign a visual look. |
| Paint | `render_paint` | `appearance`, `paste` | Broad brush with blue bristles. |
| Pick up | `render_pick_up` | `appearance`, `material` | Eyedropper with a narrow sampling tip. |
| Replace all | `render_replace_all` | `copy`, `paste` | One swatch flows to a stack of replacement swatches. |
| Copy look | `render_copy_look` | `copy`, `material` | Overlapping look cards, foreground sphere swatch. |
| Remove override | `render_remove_override` | `appearance`, `delete` | Look swatch with a minus cue. |
| Same look | `render_same_look` | `select`, `material` | Matching swatches and a selection pointer. |
| From material | `render_from_material` | `material`, `paste` | Physical sphere feeds a look swatch through an inheritance arrow. |
| Library | `render_library` | `library`, `copy` | Four substantial swatches in a grid; distinct from book libraries. |
| Lighting presets | `render_lighting_presets` | `new`, `render_scene` | Preset card with a light and a choice row. |
| Environment | `render_environment` | `environment_map`, `render_scene` | Alias of environment_map: environment globe with a light opening. |
| Light area | `render_light_area` | `box`, `camera` | Remade: light-filled parallelogram panel on the canonical axes, with the shared dotted stem. |
| Light spot | `render_light_spot` | `camera`, `material` | Remade: one light-filled cone with a curved rim and the shared dotted stem. |
| Light point | `render_light_point` | `camera`, `material` | Remade: light-filled slate ring and small slate centre dot, with the shared dotted stem. |
| Sun | `render_sun` | `sphere`, `camera` | Remade: the same source-disc size with eight short, equally spaced radial rays. |
| Light by highlight | `render_light_by_highlight` | `sphere`, `material` | Remade: quietly shaded sphere and small slate highlight target, with three blue dots arriving from above. |
| Solo light | `render_solo_light` | `fit`, `camera` | Remade: exact point-light disc, inner dot and stem inside quiet corner focus brackets. |
| Ground | `render_ground` | `box`, `render_scene` | Canonical grey slab with a blue contact patch; distinct from assembly ground. |
| Backdrop | `render_backdrop` | `render_scene`, `render_image` | Continuous curved studio sweep. |
| Shot | `render_shot` | `camera`, `new` | Camera with a saved-view bookmark. |
| Update shot | `render_update_shot` | `camera`, `undo` | Camera with a refresh arrow. |
| Shots strip | `render_shots_strip` | `camera`, `copy` | Three adjacent view thumbnails, middle view active. |
| Fit product | `render_fit_product` | `fit`, `camera` | Alias of fit: fit the subject to the view. |
| Lens / field of view | `render_lens` | `camera`, `fit` | Lens and a widening angular field. |
| Exposure | `render_exposure` | `material`, `environment_map` | Split bright/dark disk, plus and minus controls. |
| Aspect frame | `render_aspect_frame` | `render_image`, `fit` | Square reference behind a wide active frame. |
| Frame guides | `render_frame_guides` | `render_image`, `fit` | Four coarse rule-of-thirds lines inside a frame. |
| Render output size | `render_output_size` | `render_image`, `fit` | Output rectangle with horizontal and vertical dimensions. |
| Transparent background | `render_transparent_background` | `material`, `render_image` | Large checker tiles behind a product sphere. |
| Quality | `render_quality` | `list`, `library` | Three substantial increasing quality bars. |
| Render image | `render_execute` | `render_image`, `camera` | Alias of render_image: create a rendered image. |
| Check render | `render_check` | `render_image`, `new` | Image with a large preflight check. |
| Cancel render | `render_cancel` | `render_image`, `delete` | Image with a square stop control. |
| Export image | `render_export_image` | `render_image`, `copy` | Image with an outward export arrow. |
| Compare | `render_compare` | `render_image`, `material` | One subject split into two look previews. |
| Results | `render_results` | `render_image`, `copy` | Completed image stack with a check. |
| Render all shots | `render_all_shots` | `copy`, `render_image` | Queued shot cards with a play control. |
