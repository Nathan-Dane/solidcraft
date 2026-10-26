//! Standalone Environment inspector body. Drawing and source browsing never save a setup.
use super::{ImageMethod, Stage, cpu, setups, studio};
use crate::{SolveApp, theme::Tokens, ui_kit};
use serde_json::{Value, json};
use solidcraft_engine::doc::render_settings::{RenderJob, RenderSetup};

pub(super) fn editor(app: &mut SolveApp, ui: &mut egui::Ui) {
    ui_kit::scope(ui, |ui| {
        let name = source_name(app, ui.ctx());
        ui.horizontal(|ui| {
            let t = Tokens::get();
            ui_kit::sphere_thumbnail(
                ui,
                "render:environment:preview",
                ui_kit::Sphere { color: t.body, kind: ui_kit::FinishKind::Metal, opacity: None },
                t.control_height * 2.0,
            )
            .on_hover_text(app.tr("render.env_preview_help"));
            ui.vertical(|ui| {
                ui_kit::text(ui, app.tr("render.page_environment"), ui_kit::Text::Inline);
                ui_kit::text(ui, &name, ui_kit::Text::NameCell(true));
            });
        });
        let source_label = app.tr("render.env_source").to_string();
        ui_kit::row(ui, &source_label, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let change = ui_kit::button(ui, app.tr("render.env_change"), ui_kit::Button::Normal);
                crate::scenario::publish_target("render:environment:change", change.rect);
                if change.clicked() {
                    open_sources(app);
                }
                ui_kit::text(ui, &name, ui_kit::Text::Cell).on_hover_text(&name);
            });
        });
        let snapshot = setups::snapshot(app);
        let controls = environment_values(app);
        let valid = snapshot.is_ok() && controls.is_some();
        let (rotation, intensity, maximum, rotates) = controls.unwrap_or((0.0, 1.0, 100.0, false));
        ui.add_enabled_ui(valid, |ui| {
            let mut request = None;
            ui_kit::row(ui, app.tr("render.env_rotate"), |ui| {
                ui.add_enabled_ui(rotates, |ui| {
                    if let Some(value) = edit(app, ui, "rotation", rotation, |ui, value| {
                        let field = ui_kit::number_field(ui, value, -1e6..=1e6, 1.0, "°");
                        crate::scenario::publish_target("render:environment:rotation", field.rect);
                        field
                    }) {
                        request = Some(json!({"rotation_deg":value}));
                    }
                });
            });
            ui.add_enabled_ui(rotates, |ui| {
                if let Some(value) = edit(app, ui, "rotation-slider", rotation, |ui, value| {
                    let response = ui_kit::slider(ui, value, -180.0..=180.0, ui_kit::Slider { show_value: false, ..Default::default() });
                    crate::scenario::publish_target("render:environment:rotation-slider", response.rect);
                    response
                }) {
                    request = Some(json!({"rotation_deg":value}));
                }
            });
            ui_kit::row(ui, app.tr("render.env_intensity"), |ui| {
                if let Some(value) = edit(app, ui, "intensity", intensity, |ui, value| {
                    let response = ui_kit::number_field(ui, value, 0.0..=maximum, 0.05, "×");
                    crate::scenario::publish_target("render:environment:intensity", response.rect);
                    response
                }) {
                    request = Some(json!({"intensity":value}));
                }
            });
            if let Some(value) = edit(app, ui, "intensity-ev", intensity.max(1.0 / 1024.0).log2(), |ui, value| {
                let response =
                    ui_kit::slider(ui, value, -10.0..=maximum.log2(), ui_kit::Slider { format: |v| format!("{v:+.1} EV"), ..Default::default() });
                crate::scenario::publish_target("render:environment:intensity-ev", response.rect);
                response
            }) {
                request = Some(json!({"intensity":value.exp2().min(maximum)}));
            }
            if let Some(request) = request
                && let Err(error) = commit_environment(app, request)
            {
                app.set_status(error, true);
            }
        });
        if !valid {
            let reason = snapshot.err().unwrap_or_else(|| app.tr("render.env_choose_source").into());
            ui_kit::text(ui, &reason, ui_kit::Text::Note("render:environment:unavailable"));
        }
        backdrop(app, ui);
        ground(app, ui);
    });
}

pub(super) fn open_sources(app: &mut SolveApp) {
    let page = app.render_workspace.view.inspector.page;
    super::library::open_category(app, "hdr");
    if let Err(error) = app.run("view.render_stage", json!({"stage":Stage::Light.key()})) {
        app.set_status(error, true);
    }
    app.render_workspace.view.inspector.page = page;
}

fn source_name(app: &mut SolveApp, ctx: &egui::Context) -> String {
    let source = match app.render_workspace.view.method {
        ImageMethod::CpuRaytrace => {
            app.render_workspace.view.cpu.hdr.resource().filter(|_| app.render_workspace.view.cpu.environment == cpu::Environment::Hdr)
        }
        ImageMethod::GpuRaytrace => {
            app.render_workspace.view.studio.hdr.resource().filter(|_| app.render_workspace.view.studio.environment == studio::Environment::Hdr)
        }
        ImageMethod::Raster => None,
    };
    if let Some(source) = source {
        type Names = std::sync::Arc<std::collections::BTreeMap<String, String>>;
        let cache = egui::Id::new("render_environment_catalog_names");
        let names = ctx.data_mut(|data| data.get_temp::<Names>(cache)).unwrap_or_else(|| {
            let mut names = std::collections::BTreeMap::new();
            if let Ok(reply) = app.run("render.library.list", json!({"kind":"hdr","limit":128})) {
                for entry in reply["items"].as_array().into_iter().flatten().take(128) {
                    if let Some(id) = entry["id"].as_str()
                        && let Ok(detail) = app.run("render.library.describe", json!({"kind":"hdr","id":id}))
                        && let (Some(digest), Some(name)) = (detail["resource"]["sha256"].as_str(), detail["resource"]["name"].as_str())
                    {
                        names.insert(digest.to_string(), name.to_string());
                    }
                }
            }
            let names = std::sync::Arc::new(names);
            ctx.data_mut(|data| data.insert_temp(cache, names.clone()));
            names
        });
        if let Some(name) = names.get(&source.to_string()) {
            return name.clone();
        }
    }
    super::scene::environment(app).0
}

fn environment_values(app: &SolveApp) -> Option<(f64, f64, f64, bool)> {
    let v = &app.render_workspace.view;
    match v.method {
        ImageMethod::GpuRaytrace if v.studio.environment != studio::Environment::None => {
            Some((wrapped_degrees(v.studio.rotation_rad), v.studio.intensity, 100.0, true))
        }
        ImageMethod::CpuRaytrace if v.cpu.environment != cpu::Environment::None => {
            Some((wrapped_degrees(v.cpu.rotation_rad), v.cpu.intensity, 64.0, v.cpu.environment == cpu::Environment::Hdr))
        }
        _ => None,
    }
}

fn wrapped_degrees(radians: f64) -> f64 {
    let degrees = radians.to_degrees();
    if degrees.is_finite() { (degrees + 180.0).rem_euclid(360.0) - 180.0 } else { 0.0 }
}

pub(super) fn commit_environment(app: &mut SolveApp, mut request: Value) -> Result<(), String> {
    let fields = request.as_object_mut().ok_or_else(|| app.tr("render.env_invalid_edit").to_string())?;
    fields.insert("name".into(), json!(app.render_workspace.view.name));
    fields.insert("set".into(), json!(setups::snapshot(app)?));
    app.run("render.environment.set", request)?;
    setups::load(app);
    Ok(())
}

fn save(app: &mut SolveApp, setup: RenderSetup) -> Result<(), String> {
    app.run("render.settings.set", json!({"name":app.render_workspace.view.name,"set":setup}))?;
    setups::load(app);
    Ok(())
}

pub(super) fn commit_backdrop(app: &mut SolveApp, color: [u8; 3], transparent: bool) -> Result<(), String> {
    let mut setup = setups::snapshot(app)?;
    setup.background = color;
    match &mut setup.job {
        Some(RenderJob::Raster { transparent_background }) => *transparent_background = transparent,
        Some(RenderJob::GpuPath { transparent_background, options }) => {
            *transparent_background = transparent;
            options.backdrop = None;
        }
        _ if transparent => return Err(app.tr("render.cpu_opaque_required").into()),
        _ => {}
    }
    save(app, setup)
}

// Keep an in-progress gesture outside the design. Release/Enter commits once;
// external commands, undo and document switches invalidate its temporary identity.
fn edit<T: Clone + PartialEq + Send + Sync + 'static>(
    app: &SolveApp,
    ui: &mut egui::Ui,
    key: &str,
    current: T,
    draw: impl FnOnce(&mut egui::Ui, &mut T) -> egui::Response,
) -> Option<T> {
    let id =
        ui.id().with(("render-environment", key, &app.render_workspace.view.name, std::sync::Arc::as_ptr(&app.session.doc), app.session.revision));
    let (_, mut value, mut pending) = ui
        .data_mut(|data| data.get_temp::<(T, T, bool)>(id))
        .filter(|(original, _, _)| original == &current)
        .unwrap_or((current.clone(), current.clone(), false));
    let response = draw(ui, &mut value);
    pending |= response.changed();
    let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
    let finished =
        response.drag_stopped() || response.lost_focus() || (!ui.input(|input| input.pointer.any_down()) && !ui.ctx().egui_wants_keyboard_input());
    if escape || (pending && finished) {
        ui.data_mut(|data| data.remove::<(T, T, bool)>(id));
        return (!escape).then_some(value);
    }
    ui.data_mut(|data| data.insert_temp(id, (current, value, pending)));
    None
}

fn backdrop(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.env_backdrop").to_string();
    ui_kit::section(ui, "render_environment_backdrop", &title, true, |ui| {
        ui.add_enabled_ui(false, |ui| {
            let mut visible = false;
            ui_kit::checkbox(ui, app.tr("render.env_show_environment"), &mut visible);
        })
        .response
        .on_disabled_hover_text(app.tr("render.env_backdrop_pending"));
        let snapshot = setups::snapshot(app);
        ui.add_enabled_ui(snapshot.is_ok(), |ui| {
            let Ok(setup) = snapshot else { return };
            let mut mode = usize::from(app.render_workspace.view.transparent);
            let mode_changed = ui_kit::segmented(
                ui,
                &mut mode,
                &[app.tr("render.env_colour"), app.tr("render.env_clear")],
                ui_kit::Segments { targets: &["render:environment:colour", "render:environment:clear"], ..Default::default() },
            );
            let color = edit(app, ui, "backdrop-colour", setup.background, |ui, value| {
                ui_kit::row(ui, app.tr("render.env_colour"), |ui| {
                    let response = ui_kit::color_field(ui, value);
                    crate::scenario::publish_target("render:environment:backdrop-colour", response.rect);
                    response
                })
            });
            if (mode_changed || color.is_some())
                && let Err(error) = commit_backdrop(app, color.unwrap_or(setup.background), mode == 1)
            {
                app.set_status(error, true);
            }
        });
        ui.add_enabled_ui(false, |ui| {
            ui_kit::row(ui, app.tr("render.env_blur"), |ui| {
                ui_kit::slider(ui, &mut 0.0, 0.0..=100.0, ui_kit::Slider { format: |v| format!("{v:.0} %"), ..Default::default() });
            });
        })
        .response
        .on_disabled_hover_text(app.tr("render.env_backdrop_pending"));
    });
}

#[derive(Clone)]
pub(super) struct Ground {
    pub margin: f64,
    pub gap: f64,
    pub color: [f64; 3],
}

fn ground_values(app: &SolveApp, setup: &RenderSetup) -> Ground {
    let mut value = Ground { margin: 0.25, gap: 0.1, color: [0.18; 3] };
    if let Some(panel) = setup.stage.iter().find(|panel| matches!(panel.id.as_str(), "ground" | "studio_ground")) {
        value.color = panel.reflectance_linear;
        if let Ok(scene) = solidcraft_engine::product_scene::extract(&app.session)
            && !scene.bounds.is_empty()
        {
            let bounds = scene.bounds;
            let size = bounds.max - bounds.min;
            let extent = size.x.max(size.y).max(size.z).max(0.001);
            value.margin = ((panel.half_u_mm.first().copied().unwrap_or(0.0).abs() - (size.x * 0.5).max(0.001)) / extent).clamp(0.0, 10.0);
            value.gap = (bounds.min.z - panel.center_mm.get(2).copied().unwrap_or(bounds.min.z)).clamp(0.0001, 1000.0);
        }
    }
    value
}

pub(super) fn commit_ground(app: &mut SolveApp, enabled: bool, value: Ground) -> Result<(), String> {
    let set = setups::snapshot(app)?;
    let request = if enabled {
        json!({"name":app.render_workspace.view.name,"set":set,"margin_fraction":value.margin,"gap_mm":value.gap,"reflectance_linear":value.color,"replace_stage":true})
    } else {
        json!({"name":app.render_workspace.view.name,"set":set})
    };
    app.run(if enabled { "render.stage.ground" } else { "render.stage.clear" }, request)?;
    setups::load(app);
    Ok(())
}

fn ground(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.env_ground").to_string();
    ui_kit::section(ui, "render_environment_ground", &title, true, |ui| {
        ui.add_enabled_ui(false, |ui| {
            ui_kit::checkbox(ui, app.tr("render.env_shadow_catcher"), &mut false);
            ui_kit::checkbox(ui, app.tr("render.env_reflection"), &mut false);
        })
        .response
        .on_disabled_hover_text(app.tr("render.env_ground_pending"));
        let snapshot = setups::snapshot(app);
        let supported = app.render_workspace.view.method != ImageMethod::Raster
            && snapshot.as_ref().is_ok_and(|setup| {
                setup.stage.iter().count() <= 1 && setup.stage.iter().all(|panel| matches!(panel.id.as_str(), "ground" | "studio_ground"))
            });
        ui.add_enabled_ui(snapshot.is_ok() && supported, |ui| {
            let Ok(setup) = snapshot else { return };
            let mut enabled = !setup.stage.is_empty();
            let mut value = ground_values(app, &setup);
            let toggle = ui_kit::checkbox(ui, app.tr("render.env_physical_ground"), &mut enabled);
            crate::scenario::publish_target("render:environment:ground", toggle.rect);
            let mut changed = toggle.changed();
            let title = app.tr("render.env_advanced").to_string();
            ui_kit::section(ui, "render_environment_ground_advanced", &title, false, |ui| {
                ui.add_enabled_ui(enabled, |ui| {
                    for (key, label, field, range, speed, suffix) in [
                        ("margin", "render.ground_margin", &mut value.margin, 0.0..=10.0, 0.05, "×"),
                        ("gap", "render.ground_gap", &mut value.gap, 0.0001..=1000.0, 0.01, " mm"),
                    ] {
                        ui_kit::row(ui, app.tr(label), |ui| {
                            if let Some(edited) = edit(app, ui, key, *field, |ui, value| {
                                let response = ui_kit::number_field(ui, value, range, speed, suffix);
                                crate::scenario::publish_target(&format!("render:environment:ground-{key}"), response.rect);
                                response
                            }) {
                                *field = edited;
                                changed = true;
                            }
                        });
                    }
                    for (index, field) in value.color.iter_mut().enumerate() {
                        ui_kit::row(
                            ui,
                            &format!("{} {}", app.tr("render.ground_linear_rgb"), ["R", "G", "B"].get(index).copied().unwrap_or("")),
                            |ui| {
                                if let Some(edited) = edit(app, ui, &format!("ground-colour-{index}"), *field, |ui, value| {
                                    ui_kit::number_field(ui, value, 0.0..=1.0, 0.01, "")
                                }) {
                                    *field = edited;
                                    changed = true;
                                }
                            },
                        );
                    }
                });
            });
            if changed && let Err(error) = commit_ground(app, enabled, value) {
                app.set_status(error, true);
            }
        })
        .response
        .on_disabled_hover_text(app.tr("render.env_stage_details"));
        ui_kit::text(ui, app.tr("render.env_ground_pending"), ui_kit::Text::Note("render:environment:ground-help"));
    });
}
