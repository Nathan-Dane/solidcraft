//! Selected-light editor and transient visibility. Saved settings are edited only by commands.
use super::{ImageMethod, RenderSelection, cpu, setups, studio};
use crate::{SolveApp, theme::Tokens, ui_kit};
use serde_json::{Value, json};
use solidcraft_engine::doc::render_settings::{CpuJob, GpuJob, RenderJob, RenderSetup, SkyEnvironment};
use solidcraft_engine::render::job::{AreaLights, StudioLighting};
use solidcraft_engine::schema::render_lights::{Geometry, Light, Visibility};

#[derive(Clone, Debug, Default)]
pub(super) struct View {
    pub visibility: Visibility,
    scope: String,
    aim: Option<[f64; 3]>,
    draft: Option<Draft>,
}

#[derive(Clone, Debug)]
struct Draft {
    id: String,
    revision: u64,
    ev: f64,
    temperature: f64,
    softness: f64,
    azimuth: f64,
    elevation: f64,
    distance: f64,
    size: f64,
}

/// Lighting-only read snapshot also works with unsaved HDR paths and temporary bindings.
fn settings(app: &SolveApp) -> Result<RenderSetup, String> {
    let view = &app.render_workspace.view;
    let job = match view.method {
        ImageMethod::Raster => RenderJob::Raster { transparent_background: false },
        ImageMethod::CpuRaytrace => RenderJob::CpuRaytrace {
            options: CpuJob {
                lights: view.raytrace.lights.clone(),
                preset: view.raytrace.preset,
                punctual_lights: view.raytrace.punctual_lights.clone(),
                area_lights: view.raytrace.area_lights.clone(),
                ..Default::default()
            },
        },
        ImageMethod::GpuRaytrace => RenderJob::GpuPath {
            transparent_background: false,
            options: Box::new(GpuJob {
                lighting: if view.studio.lighting_enabled {
                    Some(StudioLighting {
                        sun: view.studio.sun_enabled.then_some(view.studio.sun),
                        environment: view.studio.ambient,
                        area_lights: AreaLights::new(&view.studio.area_lights).map_err(|e| e.to_string())?,
                    })
                } else {
                    None
                },
                // Environment illumination suppresses the legacy analytic studio; its bytes are not needed here.
                environment: (view.studio.environment != studio::Environment::None).then_some(SkyEnvironment::Sky {
                    gradient: Default::default(),
                    rotation_rad: 0.0,
                    intensity: 1.0,
                }),
                ..Default::default()
            }),
        },
    };
    Ok(RenderSetup { camera: app.cam, job: Some(job), ..Default::default() })
}

pub(super) fn rows(app: &SolveApp) -> Result<Vec<Light>, String> {
    solidcraft_engine::render_lights::inspect(&settings(app)?)
}

pub(super) fn name(app: &SolveApp, light: &Light) -> String {
    if light.number <= 3 { app.tr(&light.name_key).into() } else { crate::i18n::format(&light.name_key, &[("number", &light.number.to_string())]) }
}

pub(crate) fn validate_visibility(app: &SolveApp, params: &Value) -> Result<(), String> {
    if let Some(id) = params.get("id").and_then(Value::as_str)
        && !rows(app)?.iter().any(|light| light.id == id)
    {
        return Err(app.tr("render.light_missing").into());
    }
    Ok(())
}

pub(crate) fn apply_visibility(app: &mut SolveApp, command: &str, reply: &Value) {
    let visibility = &mut app.render_workspace.view.lights.visibility;
    let id = reply.get("id").and_then(Value::as_str);
    if command == "render.lights.solo" {
        visibility.solo = if reply.get("solo").and_then(Value::as_bool) == Some(true) { id.map(str::to_owned) } else { None };
    } else if let Some(id) = id {
        if reply.get("mute").and_then(Value::as_bool) == Some(true) {
            visibility.muted.insert(id.into());
        } else {
            visibility.muted.remove(id);
        }
    }
}

pub(super) fn loaded(app: &mut SolveApp) {
    if app.render_workspace.view.lights.scope != app.render_workspace.view.name {
        app.render_workspace.view.lights = View { scope: app.render_workspace.view.name.clone(), ..Default::default() };
    }
    if let Ok(lights) = rows(app) {
        let view = &mut app.render_workspace.view.lights;
        view.visibility.muted.retain(|id| lights.iter().any(|light| &light.id == id));
        if view.visibility.solo.as_ref().is_some_and(|id| !lights.iter().any(|light| &light.id == id)) {
            view.visibility.solo = None;
        }
    }
}

/// Runs after successful command dispatch, including programmatic control-channel edits.
pub(crate) fn edited(app: &mut SolveApp, params: &Value, reply: &Value) {
    let Some(name) = reply.get("name").and_then(Value::as_str) else { return };
    if name != app.render_workspace.view.name {
        return;
    }
    let visibility = app.render_workspace.view.lights.visibility.clone();
    setups::load(app);
    app.render_workspace.view.lights.visibility = visibility;
    loaded(app);
    if let Some(aim) = params.get("set").and_then(|set| set.get("aim_at")) {
        app.render_workspace.view.lights.aim = serde_json::from_value(aim.clone()).ok();
    }
    app.render_workspace.view.selection =
        reply.get("id").and_then(Value::as_str).map_or(RenderSelection::Nothing, |id| RenderSelection::Light(id.into()));
}

fn product_centre(app: &SolveApp) -> [f64; 3] {
    let bounds = solidcraft_engine::view::bounds(&app.session);
    let point = if bounds.is_empty() { app.cam.target } else { bounds.center() };
    [point.x, point.y, point.z]
}

pub(super) fn edit(app: &mut SolveApp, command: &str, id: Option<&str>, mut set: Option<Value>) -> Result<(), String> {
    let snapshot = setups::snapshot(app)?;
    let mut request = json!({"name":app.render_workspace.view.name,"settings":snapshot,"expected_revision":app.session.revision});
    if let Some(id) = id {
        request["id"] = json!(id);
    }
    if let Some(patch) = set.as_mut().and_then(Value::as_object_mut) {
        let kind = patch.get("kind").and_then(Value::as_str).or_else(|| id.and_then(|id| id.split_once(':').map(|(kind, _)| kind))).unwrap_or("area");
        if kind != "directional"
            && ["kind", "azimuth_deg", "elevation_deg", "distance_mm"].iter().any(|key| patch.contains_key(*key))
            && !patch.contains_key("aim_at")
        {
            patch.insert("aim_at".into(), json!(app.render_workspace.view.lights.aim.unwrap_or_else(|| product_centre(app))));
        }
    }
    if let Some(set) = set {
        request["set"] = set;
    }
    app.run(command, request).map(|_| ())
}

/// Applied to copies at the request boundary, including engine.execute through SolveApp::run.
/// The draft and saved RenderSetup remain unfiltered, including while saving or undoing.
pub(crate) fn filter_request(app: &SolveApp, mut request: Value) -> Result<Value, String> {
    let visibility = &app.render_workspace.view.lights.visibility;
    if !visibility.active() {
        return Ok(request);
    }
    if request.is_null() {
        request = json!({});
    }
    if !request.is_object() {
        return Err("render request must be an object".into());
    }
    let mut setup = if let Some(inline) = request.get("settings").filter(|value| !value.is_null()) {
        serde_json::from_value::<RenderSetup>(inline.clone()).map_err(|e| e.to_string())?
    } else if let Some(name) = request.get("setup").and_then(Value::as_str) {
        app.session.doc.render.setups.get(name).cloned().ok_or_else(|| app.tr("render.light_missing").to_string())?
    } else {
        RenderSetup::default()
    };
    if setup.job.is_some() {
        if let Some(weights) = solidcraft_engine::render_lights::filter(&mut setup, visibility)? {
            request["studio_weights"] = json!(weights);
        }
        if let Some(object) = request.as_object_mut() {
            object.remove("setup");
        }
        request["settings"] = json!(setup);
    } else if request.get("progressive").and_then(Value::as_bool) == Some(true) {
        let gpu_lighting = request
            .get("lighting")
            .filter(|value| !value.is_null())
            .map(|value| serde_json::from_value(value.clone()).map_err(|e| e.to_string()))
            .transpose()?;
        let environment = request.get("environment").filter(|value| !value.is_null()).map(|_| SkyEnvironment::Sky {
            gradient: Default::default(),
            rotation_rad: 0.0,
            intensity: 1.0,
        });
        let mut lighting = RenderSetup {
            job: Some(RenderJob::GpuPath {
                transparent_background: false,
                options: Box::new(GpuJob { lighting: gpu_lighting, environment, ..Default::default() }),
            }),
            ..Default::default()
        };
        if let Some(weights) = solidcraft_engine::render_lights::filter(&mut lighting, visibility)? {
            request["studio_weights"] = json!(weights);
        }
        if let Some(RenderJob::GpuPath { options, .. }) = lighting.job {
            request["lighting"] = json!(options.lighting);
        }
    } else if request.get("integrator").and_then(Value::as_str) == Some("raytrace") {
        let options: solidcraft_engine::render::job::RaytraceOptions =
            serde_json::from_value(request.get("raytrace").filter(|value| !value.is_null()).cloned().unwrap_or_else(|| json!({})))
                .map_err(|e| e.to_string())?;
        let mut lighting = RenderSetup {
            job: Some(RenderJob::CpuRaytrace {
                options: CpuJob {
                    lights: options.lights.clone(),
                    preset: options.preset,
                    punctual_lights: options.punctual_lights.clone(),
                    area_lights: options.area_lights.clone(),
                    ..Default::default()
                },
            }),
            ..Default::default()
        };
        solidcraft_engine::render_lights::filter(&mut lighting, visibility)?;
        if let Some(RenderJob::CpuRaytrace { options: filtered }) = lighting.job {
            let mut options = options;
            options.lights = filtered.lights;
            options.preset = filtered.preset;
            options.punctual_lights = filtered.punctual_lights;
            options.area_lights = filtered.area_lights;
            request["raytrace"] = json!(options);
        }
    } else if let Some(weights) = solidcraft_engine::render_lights::filter(&mut setup, visibility)? {
        request["studio_weights"] = json!(weights);
    }
    Ok(request)
}

/// Unshadowed analytic preview: area sources use their centre and projected area.
/// Final rendering retains the full emitter and transport contracts.
pub(super) fn preview(app: &SolveApp) -> Result<Vec<[f32; 16]>, String> {
    let mut output = Vec::new();
    for light in rows(app)? {
        if !app.render_workspace.view.lights.visibility.visible(&light.id) {
            continue;
        }
        let mut value = [0.0_f32; 16];
        let (position, kind) = match light.geometry {
            Geometry::Directional { direction, .. } => (direction, 0.0),
            Geometry::Point { position_mm, .. } => (position_mm, 1.0),
            Geometry::Spot { position_mm, .. } => (position_mm, 2.0),
            Geometry::Area { position_mm, .. } => (position_mm, 3.0),
        };
        for (slot, v) in value.iter_mut().take(3).zip(position) {
            *slot = v as f32;
        }
        if let Some(slot) = value.get_mut(3) {
            *slot = kind;
        }
        for (slot, v) in value.iter_mut().skip(4).take(3).zip(light.colour) {
            *slot = (v * light.intensity) as f32;
        }
        let (axis, inner, outer, scale, minimum) = match light.geometry {
            Geometry::Point { reference_distance_mm, minimum_distance_mm, .. } => {
                ([0.0; 3], 0.0, 0.0, reference_distance_mm.powi(2), minimum_distance_mm)
            }
            Geometry::Spot {
                emitted_axis, inner_half_angle_radians, outer_half_angle_radians, reference_distance_mm, minimum_distance_mm, ..
            } => (emitted_axis, inner_half_angle_radians.cos(), outer_half_angle_radians.cos(), reference_distance_mm.powi(2), minimum_distance_mm),
            Geometry::Area { edge_u_mm: [ux, uy, uz], edge_v_mm: [vx, vy, vz], .. } => {
                let normal = solidcraft_engine::geom::Vec3::new(ux, uy, uz).cross(solidcraft_engine::geom::Vec3::new(vx, vy, vz));
                let area = normal.len();
                let n = solidcraft_engine::render::surface::unit_direction(normal).unwrap_or(-solidcraft_engine::geom::Vec3::Z);
                ([n.x, n.y, n.z], 0.0, 0.0, area, 0.001)
            }
            _ => ([0.0; 3], 0.0, 0.0, 1.0, 0.001),
        };
        for (slot, v) in value.iter_mut().skip(8).take(3).zip(axis) {
            *slot = v as f32;
        }
        if let Some(slot) = value.get_mut(7) {
            *slot = outer as f32;
        }
        if let Some(slot) = value.get_mut(11) {
            *slot = inner as f32;
        }
        if let Some(slot) = value.get_mut(12) {
            *slot = scale as f32;
        }
        if let Some(slot) = value.get_mut(13) {
            *slot = minimum as f32;
        }
        output.push(value);
    }
    let view = &app.render_workspace.view;
    let ambient = match view.method {
        ImageMethod::GpuRaytrace if view.studio.lighting_enabled => view.studio.ambient,
        ImageMethod::CpuRaytrace if view.cpu.environment == cpu::Environment::Constant => view.cpu.color.map(|c| c * view.cpu.intensity),
        _ => [0.0; 3],
    };
    if ambient.iter().any(|c| *c > 0.0) {
        let mut value = [0.0; 16];
        if let Some(slot) = value.get_mut(3) {
            *slot = 4.0;
        }
        for (slot, c) in value.iter_mut().skip(4).take(3).zip(ambient) {
            *slot = c as f32;
        }
        output.push(value);
    }
    Ok(output)
}

pub(super) fn editor(app: &mut SolveApp, ui: &mut egui::Ui, id: &str) {
    let lights = match rows(app) {
        Ok(lights) => lights,
        Err(error) => {
            ui_kit::text(ui, &error, ui_kit::Text::Error("render.light_invalid"));
            return;
        }
    };
    let Some(light) = lights.iter().find(|light| light.id == id) else {
        ui_kit::text(ui, app.tr("render.light_missing"), ui_kit::Text::Note("render.light_missing"));
        return;
    };
    crate::scenario::publish_target("render:light:editor", ui.available_rect_before_wrap());
    let kind = light.geometry.kind();
    let gpu = app.render_workspace.view.method == ImageMethod::GpuRaytrace;
    let blocked = app.render_workspace.view.method == ImageMethod::Raster
        || (gpu && lights.iter().filter(|light| light.geometry.kind() == "directional").count() > 1);
    let type_key = match kind {
        "area" => "render.light_type_area",
        "spot" => "render.light_type_spot",
        "point" => "render.light_type_point",
        _ => "render.light_type_sun",
    };
    ui_kit::scope(ui, |ui| {
        ui.horizontal(|ui| {
            let [r, g, b] = light.colour.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8);
            ui_kit::sphere_thumbnail(
                ui,
                "render:light:sphere",
                ui_kit::Sphere { color: egui::Color32::from_rgb(r, g, b), kind: ui_kit::FinishKind::Gloss, opacity: None },
                Tokens::get().control_height * 2.0,
            );
            ui.vertical(|ui| {
                ui_kit::text(ui, &name(app, light), ui_kit::Text::Heading("render.light_heading"));
                ui.add_enabled_ui(false, |ui| ui_kit::choice_chip(ui, "render:light:type-chip", app.tr(type_key), true));
                let solo = app.render_workspace.view.lights.visibility.solo.as_deref() == Some(id);
                if ui_kit::choice_chip(ui, "render:light:solo", app.tr("render.tree_solo"), solo).clicked() {
                    let _ = app.run("render.lights.solo", json!({"id":id,"solo":!solo}));
                }
            });
        });
        if blocked {
            ui_kit::text(ui, app.tr("render.light_backend_limit"), ui_kit::Text::Note("render.light_backend_limit"));
        }
        let aim = app.render_workspace.view.lights.aim.unwrap_or_else(|| product_centre(app));
        let (azimuth, elevation, distance) = solidcraft_engine::render_lights::position(light, aim, app.cam.distance);
        let size = match light.geometry {
            Geometry::Area { edge_u_mm: [x, y, z], .. } => (x * x + y * y + z * z).sqrt(),
            _ => 10.0,
        };
        let softness = match light.geometry {
            Geometry::Directional { angular_radius_rad: Some(radius), .. } => (radius - 0.001) / (std::f64::consts::FRAC_PI_2 - 0.001),
            Geometry::Area { .. } => (size - 0.0002) / 1000.0,
            _ => 0.0,
        };
        let mut draft =
            app.render_workspace.view.lights.draft.take().filter(|draft| draft.id == id && draft.revision == app.session.revision).unwrap_or(Draft {
                id: id.into(),
                revision: app.session.revision,
                ev: light.intensity.max(2.0_f64.powi(-20)).log2(),
                temperature: solidcraft_engine::render_lights::colour_temperature(light.colour),
                softness,
                azimuth,
                elevation,
                distance,
                size,
            });
        ui.add_enabled_ui(!blocked, |ui| {
            let labels = [
                app.tr("render.light_type_area").to_string(),
                app.tr("render.light_type_spot").to_string(),
                app.tr("render.light_type_point").to_string(),
                app.tr("render.light_type_sun").to_string(),
            ];
            let kinds = ["area", "spot", "point", "directional"];
            let mut selected = kinds.iter().position(|value| *value == kind).unwrap_or(0);
            let reason = app.tr("render.light_gpu_punctual").to_string();
            let disabled = [None, gpu.then_some(reason.as_str()), gpu.then_some(reason.as_str()), None];
            ui_kit::text(ui, app.tr("render.light_type"), ui_kit::Text::Inline);
            if ui_kit::segmented(
                ui,
                &mut selected,
                &labels.iter().map(String::as_str).collect::<Vec<_>>(),
                ui_kit::Segments {
                    compact: true,
                    disabled: &disabled,
                    targets: &["render:light:type:area", "render:light:type:spot", "render:light:type:point", "render:light:type:directional"],
                    ..Default::default()
                },
            ) && let Some(chosen) = kinds.get(selected)
                && *chosen != kind
            {
                dispatch(app, "render.lights.set", Some(id), Some(json!({"kind":chosen})));
            }
            field(app, ui, id, "render.light_intensity", "intensity_ev", &mut draft.ev, -20.0..=if gpu { 27.0 } else { 6.0 }, 0.1, " EV", true);
            field(app, ui, id, "render.light_temperature", "temperature_k", &mut draft.temperature, 1000.0..=40000.0, 50.0, " K", true);
            ui_kit::text(ui, app.tr("render.light_temperature_help"), ui_kit::Text::Note("render.light_temperature_help"));
            let soft = kind == "area" || (kind == "directional" && gpu);
            field(app, ui, id, "render.light_softness", "softness", &mut draft.softness, 0.0..=1.0, 0.01, "", soft);
            if !soft {
                ui_kit::text(ui, app.tr("render.light_hard_shadow"), ui_kit::Text::Note("render.light_hard_shadow"));
            }
            let title = app.tr("render.light_position").to_string();
            ui_kit::section(ui, "render:light:position", &title, true, |ui| {
                field(app, ui, id, "render.light_azimuth", "azimuth_deg", &mut draft.azimuth, -360.0..=360.0, 1.0, "°", true);
                field(app, ui, id, "render.light_elevation", "elevation_deg", &mut draft.elevation, -90.0..=90.0, 1.0, "°", true);
                field(app, ui, id, "render.light_distance", "distance_mm", &mut draft.distance, 0.001..=1e6, 1.0, " mm", kind != "directional");
                field(app, ui, id, "render.light_size", "size_mm", &mut draft.size, 0.0002..=if gpu { 2e6 } else { 1e6 }, 1.0, " mm", kind == "area");
                ui.add_enabled_ui(matches!(kind, "area" | "spot"), |ui| {
                    let label = app.tr("render.light_aim").to_string();
                    ui_kit::row(ui, &label, |ui| {
                        let current = if app.render_workspace.view.lights.aim == Some([0.0; 3]) {
                            "render.light_world_origin"
                        } else {
                            "render.light_product_centre"
                        };
                        let current = app.tr(current).to_string();
                        ui_kit::dropdown_menu(ui, "render:light:aim", &current, |ui| {
                            for (key, target) in [("render.light_product_centre", product_centre(app)), ("render.light_world_origin", [0.0; 3])] {
                                if ui_kit::list_picker(ui, key, app.tr(key), aim == target).clicked() {
                                    dispatch(app, "render.lights.set", Some(id), Some(json!({"aim_at":target})));
                                    ui.close();
                                }
                            }
                        });
                    });
                });
            });
        });
        app.render_workspace.view.lights.draft = Some(draft);
        ui_kit::button(ui, app.tr("render.light_place_highlight"), ui_kit::Button::Enabled(false))
            .on_hover_text(app.tr("render.light_place_pending"));
        let title = app.tr("render.light_behaviour").to_string();
        ui_kit::section(ui, "render:light:behaviour", &title, false, |ui| {
            let mut muted = app.render_workspace.view.lights.visibility.muted.contains(id);
            if ui_kit::checkbox(ui, app.tr("render.tree_mute"), &mut muted).changed() {
                let _ = app.run("render.lights.mute", json!({"id":id,"mute":muted}));
            }
            ui_kit::text(ui, app.tr("render.light_view_only"), ui_kit::Text::Note("render.light_view_only"));
        });
        ui.horizontal(|ui| {
            if ui_kit::button(ui, app.tr("render.light_duplicate"), ui_kit::Button::Enabled(!blocked)).clicked() {
                dispatch(app, "render.lights.duplicate", Some(id), None);
            }
            if ui_kit::button(ui, app.tr("render.light_delete"), ui_kit::Button::Enabled(!blocked)).clicked() {
                dispatch(app, "render.lights.remove", Some(id), None);
            }
        });
        ui_kit::text(ui, app.tr("render.light_preview_help"), ui_kit::Text::Note("render.light_preview_help"));
    });
}

fn dispatch(app: &mut SolveApp, command: &str, id: Option<&str>, set: Option<Value>) {
    if let Err(error) = edit(app, command, id, set) {
        app.set_status(error, true);
    }
}

#[allow(clippy::too_many_arguments)]
fn field(
    app: &mut SolveApp,
    ui: &mut egui::Ui,
    id: &str,
    label: &str,
    key: &str,
    value: &mut f64,
    range: std::ops::RangeInclusive<f64>,
    speed: f64,
    suffix: &str,
    enabled: bool,
) {
    let label = app.tr(label).to_string();
    ui.add_enabled_ui(enabled, |ui| {
        ui_kit::row(ui, &label, |ui| {
            let response = ui_kit::number_field(ui, value, range, speed, suffix);
            crate::scenario::publish_target(&format!("render:light:{key}"), response.rect);
            if response.drag_stopped() || (response.changed() && (response.has_focus() || response.lost_focus())) {
                dispatch(app, "render.lights.set", Some(id), Some(json!({(key):*value})));
            }
        })
    });
}
