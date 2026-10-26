//! Standalone Shot editor and transient frame composition. Commands own every edit.
use super::{RenderSelection, Stage, inspector};
use crate::{SolveApp, theme::Tokens, ui_kit};
use egui::{Rect, pos2, vec2};
use serde_json::{Value, json};
use solidcraft_engine::doc::render_settings::{RenderSettings, RenderSetup};

#[derive(Clone, Debug, Default)]
pub(crate) struct Frame {
    aspect: Option<String>,
    pub(super) guides: ui_kit::FrameGuides,
    name: Option<(String, String)>,
    exposure: Option<(String, f64, f64)>,
    size: Option<(String, [u32; 2], [u32; 2])>,
}

pub(crate) fn apply_frame(app: &mut SolveApp, request: &Value) {
    let frame = &mut app.render_workspace.view.frame;
    if let Some(aspect) = request.get("aspect").and_then(Value::as_str) {
        frame.aspect = Some(aspect.into());
    }
    for (key, value) in
        [("thirds", &mut frame.guides.thirds), ("safe_area", &mut frame.guides.safe_area), ("centre_mark", &mut frame.guides.centre_mark)]
    {
        if let Some(request) = request.get(key).and_then(Value::as_bool) {
            *value = request;
        }
    }
}

pub(super) fn selected(app: &SolveApp) -> Option<&str> {
    match &app.render_workspace.view.selection {
        RenderSelection::Shot(name) if app.session.doc.render.setups.contains_key(name) => Some(name),
        _ => None,
    }
}

const ASPECTS: [(&str, u32, u32, &str); 6] = [
    ("16:9", 1920, 1080, "render.frame_wide"),
    ("3:2", 1620, 1080, "render.frame_photo"),
    ("1:1", 1080, 1080, "render.frame_square"),
    ("4:5", 864, 1080, "render.frame_portrait"),
    ("9:16", 1080, 1920, "render.frame_tall"),
    ("custom", 0, 0, "render.frame_custom"),
];

fn target(app: &SolveApp) -> String {
    selected(app).unwrap_or(&app.render_workspace.view.name).to_owned()
}

fn setup(app: &SolveApp, name: &str) -> RenderSetup {
    app.session.doc.render.setups.get(name).cloned().unwrap_or_else(|| app.render_workspace.view.draft.clone())
}

fn aspect_index(app: &SolveApp, setup: &RenderSetup) -> usize {
    if let Some(aspect) = &app.render_workspace.view.frame.aspect
        && let Some(index) = ASPECTS.iter().position(|(key, ..)| *key == aspect)
    {
        return index;
    }
    ASPECTS.iter().position(|(_, w, h, _)| *w > 0 && u64::from(setup.width) * u64::from(*h) == u64::from(setup.height) * u64::from(*w)).unwrap_or(5)
}

/// Patch only the requested saved fields; unrelated live drafts and the live camera survive.
pub(super) fn edit_settings(app: &mut SolveApp, name: &str, patch: Value) -> Result<(), String> {
    app.run("render.settings.set", json!({"name":name,"set":patch}))?;
    if name == app.render_workspace.view.name {
        let saved = setup(app, name);
        let view = &mut app.render_workspace.view;
        for draft in std::iter::once(&mut view.draft).chain(view.clean_setup.iter_mut()) {
            if patch.get("width").is_some() {
                draft.width = saved.width;
            }
            if patch.get("height").is_some() {
                draft.height = saved.height;
            }
            if patch.get("exposure_ev").is_some() {
                draft.exposure_ev = saved.exposure_ev;
            }
            if patch.get("camera").is_some() {
                draft.camera = saved.camera;
            }
        }
        app.render_workspace.revision = app.session.revision;
    }
    Ok(())
}

pub(super) fn choose_aspect(app: &mut SolveApp, name: &str, index: usize) -> Result<(), String> {
    let Some((aspect, width, height, _)) = ASPECTS.get(index) else { return Err(app.tr("render.frame_invalid_aspect").into()) };
    if *width > 0 {
        edit_settings(app, name, json!({"width":width,"height":height}))?;
    }
    app.run("view.render_frame", json!({"aspect":aspect}))?;
    Ok(())
}

fn result(app: &mut SolveApp, action: Result<(), String>) {
    if let Err(error) = action {
        app.set_status(error, true);
    }
}

fn guides(app: &mut SolveApp, ui: &mut egui::Ui, editor: bool) {
    let current = app.render_workspace.view.frame.guides;
    for (key, label, mut checked) in [
        ("thirds", if editor { "render.frame_rule_thirds" } else { "render.frame_thirds" }, current.thirds),
        ("safe_area", "render.frame_safe_area", current.safe_area),
        ("centre_mark", "render.frame_centre_mark", current.centre_mark),
    ] {
        let response = ui_kit::checkbox(ui, app.tr(label), &mut checked);
        crate::scenario::publish_target(&format!("render:{}:{key}", if editor { "shot" } else { "frame" }), response.rect);
        if response.changed() {
            let _ = app.run("view.render_frame", json!({key:checked}));
        }
    }
}

pub(super) fn presets(app: &mut SolveApp, ui: &mut egui::Ui) {
    let name = target(app);
    let current = setup(app, &name);
    let selected = aspect_index(app, &current);
    let title = app.tr("render.frame_aspect").to_string();
    ui_kit::section(ui, "render_frame_aspects", &title, true, |ui| {
        let ids: Vec<_> = ASPECTS.iter().map(|(key, ..)| format!("render:frame:aspect:{key}")).collect();
        let tiles: Vec<_> = ASPECTS
            .iter()
            .enumerate()
            .map(|(index, (key, w, h, _))| ui_kit::AspectTile {
                id: ids.get(index).map(String::as_str).unwrap_or_default(),
                label: if *key == "custom" { app.tr("render.frame_custom") } else { key },
                aspect: if *h == 0 { 1.5 } else { f64::from(*w) / f64::from(*h) },
                selected: index == selected,
            })
            .collect();
        if let Some(index) = ui_kit::aspect_grid(ui, &tiles) {
            let action = choose_aspect(app, &name, index);
            result(app, action);
        }
    });
    let title = app.tr("render.frame_guides").to_string();
    ui_kit::section(ui, "render_frame_guides", &title, true, |ui| guides(app, ui, false));
}

pub(super) fn rename(app: &mut SolveApp, name: &str, new_name: &str) -> Result<(), String> {
    app.run("render.shot.rename", json!({"name":name,"new_name":new_name,"expected_revision":app.session.revision}))?;
    if app.render_workspace.view.name == name {
        app.render_workspace.view.name = new_name.into();
    }
    app.run("view.render_select", json!({"kind":"shot","id":new_name}))?;
    Ok(())
}

pub(super) fn duplicate(app: &mut SolveApp, name: &str) -> Result<(), String> {
    for number in 1..=32 {
        let mut prefix = name.to_string();
        let mut destination = crate::i18n::format("render.frame_copy_name", &[("name", &prefix), ("number", &number.to_string())]);
        while destination.len() > solidcraft_engine::doc::render_settings::MAX_SETUP_NAME_BYTES && prefix.pop().is_some() {
            destination = crate::i18n::format("render.frame_copy_name", &[("name", &prefix), ("number", &number.to_string())]);
        }
        if !app.session.doc.render.setups.contains_key(&destination) {
            app.run("render.shot.duplicate", json!({"name":name,"new_name":destination,"expected_revision":app.session.revision}))?;
            return Ok(());
        }
    }
    Err(app.tr("render.frame_copy_limit").into())
}

pub(super) fn revert(app: &mut SolveApp, name: &str) -> Result<(), String> {
    app.run("render.settings.apply", json!({"name":name}))?;
    app.follow_view_request(false);
    Ok(())
}

pub(super) fn render(app: &mut SolveApp, name: &str) -> Result<(), String> {
    let saved = app.session.doc.render.setups.get(name).ok_or_else(|| app.tr("render.ribbon_no_saved_camera").to_string())?;
    let settings = RenderSetup { camera: app.cam, ..saved.clone() };
    let reply = app.run("render.start", json!({"settings":settings}))?;
    let job = serde_json::from_value(reply["job"].clone()).map_err(|error| error.to_string())?;
    app.render_workspace.view.job = Some(job);
    app.render_workspace.view.job_shot = Some(name.into());
    Ok(())
}

/// Mounted only on the old Camera page until the separate Inspector dispatch step.
pub(super) fn editor(app: &mut SolveApp, ui: &mut egui::Ui, shot: &str) {
    let Some(saved) = app.session.doc.render.setups.get(shot).cloned() else { return };
    let title = crate::i18n::format("render.frame_shot_title", &[("name", shot)]);
    ui_kit::tree_row(ui, ui.id().with("shot_title"), &ui_kit::Row { label: &title, icon: "camera", ..Default::default() });
    if app.render_workspace.view.frame.name.as_ref().is_none_or(|(name, _)| name != shot) {
        app.render_workspace.view.frame.name = Some((shot.into(), shot.into()));
    }
    let mut name = app.render_workspace.view.frame.name.as_ref().map(|(_, text)| text.clone()).unwrap_or_else(|| shot.into());
    let invalid = RenderSettings::validate_name(&name).is_err();
    let response = ui_kit::row(ui, app.tr("render.frame_name"), |ui| {
        ui_kit::bounded_cell_editor(ui, ui.id().with(("shot_name", shot)), &mut name, "", invalid, 128)
    });
    crate::scenario::publish_target("render:shot:name", response.rect);
    let commit = response.lost_focus() || (response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)));
    app.render_workspace.view.frame.name = Some((shot.into(), name.clone()));
    if commit && name != shot {
        let action = rename(app, shot, name.trim());
        result(app, action);
        return;
    }
    if super::shots::camera_modified(app, shot) {
        ui_kit::banner(ui, ui_kit::AssistantTone::Warning, |ui| {
            ui_kit::text(ui, app.tr("render.shots_camera_changed"), ui_kit::Text::Note("render.shots_camera_changed"));
            ui.horizontal_wrapped(|ui| {
                let update = ui_kit::primary_button(ui, app.tr("render.shots_update"));
                crate::scenario::publish_target("render:shot:update", update.rect);
                if update.clicked() {
                    let action = edit_settings(app, shot, json!({"camera":app.cam}));
                    result(app, action);
                }
                let response = ui_kit::button(ui, app.tr("render.shots_revert"), ui_kit::Button::Normal);
                crate::scenario::publish_target("render:shot:revert", response.rect);
                if response.clicked() {
                    let action = revert(app, shot);
                    result(app, action);
                }
            });
        });
    }
    let title = app.tr("render.page_camera").to_string();
    ui_kit::section(ui, "render_shot_camera", &title, true, |ui| camera(app, ui, shot, &saved));
    let title = app.tr("render.stage_frame").to_string();
    ui_kit::section(ui, "render_shot_frame", &title, true, |ui| {
        let index = aspect_index(app, &saved);
        let mut chosen = None;
        let labels: Vec<_> = ASPECTS.iter().map(|(_, _, _, key)| app.tr(key).to_string()).collect();
        ui_kit::row(ui, app.tr("render.frame_aspect"), |ui| {
            let response = ui_kit::dropdown_menu(ui, "render_shot_aspect", labels.get(index).map(String::as_str).unwrap_or_default(), |ui| {
                for (choice, label) in labels.iter().enumerate() {
                    if ui_kit::list_picker(ui, &format!("render:shot:aspect:{choice}"), label, choice == index).clicked() {
                        chosen = Some(choice);
                        ui.close();
                    }
                }
            });
            crate::scenario::publish_target("render:shot:aspect", response.rect);
        });
        if let Some(index) = chosen {
            let action = choose_aspect(app, shot, index);
            result(app, action);
        }
        let observed = [saved.width, saved.height];
        if app.render_workspace.view.frame.size.as_ref().is_none_or(|(name, size, _)| name != shot || *size != observed) {
            app.render_workspace.view.frame.size = Some((shot.into(), observed, observed));
        }
        let [mut width, mut height] = app.render_workspace.view.frame.size.as_ref().map_or(observed, |(_, _, size)| *size);
        ui_kit::row(ui, app.tr("render.frame_output_size"), |ui| {
            ui.horizontal(|ui| {
                let response = ui_kit::number_field(ui, &mut width, 1..=16384, 1.0, "");
                crate::scenario::publish_target("render:shot:width", response.rect);
                ui_kit::text(ui, "×", ui_kit::Text::Inline);
                let response = ui_kit::number_field(ui, &mut height, 1..=16384, 1.0, "");
                crate::scenario::publish_target("render:shot:height", response.rect);
            });
        });
        app.render_workspace.view.frame.size = Some((shot.into(), observed, [width, height]));
        if [width, height] != observed && !ui.input(|i| i.pointer.primary_down()) {
            let action = edit_settings(app, shot, json!({"width":width,"height":height}));
            if action.is_ok() {
                let _ = app.run("view.render_frame", json!({"aspect":"custom"}));
            } else {
                app.render_workspace.view.frame.size = None;
            }
            result(app, action);
        }
        guides(app, ui, true);
    });
    ui_kit::divider(ui, "render:shot:footer-rule");
    ui.horizontal_wrapped(|ui| {
        let response =
            ui_kit::icon_label_action(ui, app.tr("render.frame_render_shot"), Some("play"), super::job_ready(app, "render.cancel").is_err(), true);
        crate::scenario::publish_target("render:shot:render", response.rect);
        if response.clicked() {
            let action = render(app, shot);
            result(app, action);
        }
        let response = ui_kit::button(ui, app.tr("render.frame_duplicate"), ui_kit::Button::Normal);
        crate::scenario::publish_target("render:shot:duplicate", response.rect);
        if response.clicked() {
            let action = duplicate(app, shot);
            result(app, action);
        }
    });
}

fn camera(app: &mut SolveApp, ui: &mut egui::Ui, shot: &str, saved: &RenderSetup) {
    let mut camera = app.cam;
    let mut projection = usize::from(camera.fov == 0.0);
    if ui_kit::row(ui, app.tr("render.frame_projection"), |ui| {
        ui_kit::segmented(
            ui,
            &mut projection,
            &[app.tr("render.perspective"), app.tr("render.stage_orthographic")],
            ui_kit::Segments { compact: true, proportional: true, workspace_accent: true, ..Default::default() },
        )
    }) {
        camera.fov = if projection == 0 { 35f64.to_radians() } else { 0.0 };
    }
    let mut fov = if projection == 0 { camera.fov.to_degrees() } else { 35.0 };
    ui.add_enabled_ui(projection == 0, |ui| {
        let response = ui_kit::row(ui, app.tr("render.frame_fov"), |ui| {
            ui_kit::slider(ui, &mut fov, 1.0..=150.0, ui_kit::Slider { format: |v| format!("{v:.0}°"), ..Default::default() })
        });
        if projection == 0 && response.changed() && (response.is_pointer_button_down_on() || response.has_focus()) {
            camera.fov = fov.to_radians();
        }
    });
    let bounds = crate::viewport::model_bounds(app);
    let size = if bounds.is_empty() { 1.0 } else { bounds.diagonal().max(0.000001) };
    let mut distance = camera.distance / size;
    let suffix = app.tr("render.frame_distance_unit").to_owned();
    let response = ui_kit::row(ui, app.tr("render.camera_distance"), |ui| {
        ui_kit::number_field(ui, &mut distance, (0.000001 / size)..=(1e9 / size), 0.01, &suffix)
    });
    crate::scenario::publish_target("render:shot:distance", response.rect);
    if response.changed() {
        camera.distance = distance * size;
    }
    if camera != app.cam {
        let action = inspector::apply_camera_draft(app, camera);
        result(app, action);
    }
    if app.render_workspace.view.frame.exposure.as_ref().is_none_or(|(name, observed, _)| name != shot || *observed != saved.exposure_ev) {
        app.render_workspace.view.frame.exposure = Some((shot.into(), saved.exposure_ev, saved.exposure_ev));
    }
    let mut exposure = app.render_workspace.view.frame.exposure.as_ref().map_or(saved.exposure_ev, |(_, _, value)| *value);
    let response = ui_kit::row(ui, app.tr("render.frame_exposure"), |ui| {
        ui_kit::slider(ui, &mut exposure, -32.0..=32.0, ui_kit::Slider { format: |v| format!("{v:+.1} EV"), ..Default::default() })
    });
    crate::scenario::publish_target("render:shot:exposure", response.rect);
    app.render_workspace.view.frame.exposure = Some((shot.into(), saved.exposure_ev, exposure));
    if exposure != saved.exposure_ev && !ui.input(|i| i.pointer.primary_down()) {
        let action = edit_settings(app, shot, json!({"exposure_ev":exposure}));
        if action.is_err() {
            app.render_workspace.view.frame.exposure = None;
        }
        result(app, action);
    }
}

/// Largest centred frame at an aspect. Reject invalid or unrepresentable geometry before arithmetic.
pub(super) fn frame_rect(aspect: f64, viewport: Rect) -> Option<Rect> {
    if !aspect.is_finite() || !(1e-6..=1e6).contains(&aspect) || !viewport.is_finite() || !viewport.is_positive() || viewport.size().max_elem() > 1e7
    {
        return None;
    }
    let width = f64::from(viewport.width()).min(f64::from(viewport.height()) * aspect);
    let height = width / aspect;
    let frame = Rect::from_center_size(viewport.center(), vec2(width as f32, height as f32));
    (frame.is_finite() && frame.is_positive()).then_some(frame)
}

pub(crate) fn overlay_visible(app: &SolveApp) -> bool {
    !app.render_workspace.view.result
        && (app.render_workspace.view.stage == Stage::Frame
            || (app.render_workspace.view.inspector.page == inspector::Page::Camera && selected(app).is_some()))
}

pub(crate) fn preview_rect(app: &SolveApp, viewport: Rect) -> Option<Rect> {
    if !overlay_visible(app) {
        return None;
    }
    let saved = setup(app, &target(app));
    let index = aspect_index(app, &saved);
    let aspect = ASPECTS
        .get(index)
        .filter(|(_, _, height, _)| *height > 0)
        .map_or(f64::from(saved.width) / f64::from(saved.height), |(_, width, height, _)| f64::from(*width) / f64::from(*height));
    let free = crate::shell::workspace_content(app, crate::render_ui::canvas(app, viewport));
    frame_rect(aspect, free.shrink(Tokens::get().space_8))
}

pub(crate) fn overlay(app: &mut SolveApp, ui: &mut egui::Ui, viewport: Rect) {
    if !overlay_visible(app) {
        return;
    }
    let t = Tokens::get();
    let saved = setup(app, &target(app));
    let Some(frame) = preview_rect(app, viewport) else { return };
    ui_kit::frame_guides(ui, viewport, frame, app.render_workspace.view.frame.guides);
    crate::scenario::publish_rect("render:frame:overlay", frame);
    let index = aspect_index(app, &saved);
    let aspect = ASPECTS.get(index).map(|(key, ..)| *key).unwrap_or("custom");
    let aspect = if aspect == "custom" { app.tr("render.frame_custom") } else { aspect };
    let label = format!("{aspect} · {} × {}", saved.width, saved.height);
    ui_kit::Container::navigation_card(
        ui.ctx(),
        egui::Id::new("render_frame_size_chip"),
        frame.left_top() + vec2(t.space_8, t.space_8),
        egui::Align2::LEFT_TOP,
        |ui| {
            ui_kit::text(ui, &label, ui_kit::Text::Inline);
        },
    );
    let lens = if app.cam.fov > 0.0 {
        // Full-frame equivalent using the vertical 24 mm sensor dimension.
        let mm = 12.0 / (app.cam.fov / 2.0).tan();
        crate::i18n::format("render.frame_lens", &[("fov", &format!("{:.0}", app.cam.fov.to_degrees())), ("mm", &format!("{mm:.0}"))])
    } else {
        app.tr("render.stage_orthographic").to_string()
    };
    ui_kit::Container::navigation_card(
        ui.ctx(),
        egui::Id::new("render_frame_lens_chip"),
        pos2(frame.left() + t.space_8, frame.bottom() - t.space_8),
        egui::Align2::LEFT_BOTTOM,
        |ui| {
            ui_kit::text(ui, &lens, ui_kit::Text::Inline);
        },
    );
}
