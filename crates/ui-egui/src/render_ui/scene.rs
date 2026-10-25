//! Render scene content in the shared Browser host and row painter.
use super::{ImageMethod, cpu, hdr, inspector, setups, studio};
use crate::{
    SolveApp,
    browser::{self, Row},
    context_menu::{self, Item},
    theme::Tokens,
    ui_kit,
};
use egui::Color32;
use serde_json::json;
use solidcraft_engine::{
    Sel,
    doc::{ModelBody, appearance::Look},
};

#[derive(Default)]
pub(crate) struct Decoration {
    pub swatch: Option<Color32>,
    pub name: String,
    pub count: String,
}

pub(crate) fn decoration(app: &SolveApp, names: &[String]) -> Decoration {
    if !super::active(app) {
        return Decoration::default();
    }
    let state = app.session.world_state();
    let bodies: Vec<_> = names.iter().filter_map(|name| state.body(name)).collect();
    let Some(body) = bodies.first() else { return Decoration::default() };
    let look = object_look(app, body);
    let mixed = bodies.iter().any(|b| object_look(app, b) != look);
    let count = bodies.iter().map(|b| app.session.doc.face_colors(b).len()).fold(0usize, usize::saturating_add);
    Decoration {
        swatch: Some(if mixed { Tokens::get().text_dim } else { look_color(app, body, look.as_ref()) }),
        name: if mixed { app.tr("render.tree_mixed").into() } else { look_name(app, look.as_ref()) },
        count: if count == 0 {
            String::new()
        } else {
            crate::i18n::format(if count == 1 { "render.tree_face_one" } else { "render.tree_face_count" }, &[("count", &count.to_string())])
        },
    }
}

fn object_look(app: &SolveApp, body: &ModelBody) -> Option<Look> {
    let face = app.session.selection.iter().rev().find_map(|s| match s {
        Sel::Face { body: name, index, .. } if name == &body.name => Some(*index),
        _ => None,
    });
    face.map_or_else(|| app.session.doc.model_display_look(body), |face| app.session.doc.model_face_look(body, face))
}

pub(crate) fn same_look(app: &SolveApp, names: &[String]) -> bool {
    let state = app.session.world_state();
    let mut looks = names.iter().filter_map(|name| state.body(name)).map(|body| app.session.doc.model_display_look(body));
    let Some(first) = looks.next() else { return false };
    looks.all(|look| look == first)
}

fn look_name(app: &SolveApp, look: Option<&Look>) -> String {
    look.map_or_else(|| app.tr("render.tree_default").into(), |look| look.name.clone())
}
fn look_color(app: &SolveApp, body: &ModelBody, look: Option<&Look>) -> Color32 {
    let color = look.map(|l| l.color).or_else(|| app.session.doc.body_color(body).map(|c| c.map(|v| (v.clamp(0.0, 1.0) * 255.0).round() as u8)));
    color.map_or(Tokens::get().body, |[r, g, b]| Color32::from_rgb(r, g, b))
}

fn section(app: &mut SolveApp, ui: &mut egui::Ui, key: &str) -> bool {
    let id = format!("render:tree:{key}");
    let open = browser::is_open(app, &id, true);
    let label = app.tr(&format!("render.tree_{key}")).to_string();
    let row = browser::draw_row(ui, ui.id().with(&id), &Row { label: &label, name: true, fold: Some(open), ..Default::default() });
    if let Some(rect) = row.rect {
        crate::scenario::publish_target(&id, rect);
    }
    if row.fold || row.clicked {
        browser::toggle(app, &id, true);
    }
    open
}

pub(crate) fn light_types(app: &SolveApp) -> Vec<&'static str> {
    let view = &app.render_workspace.view;
    let mut lights = Vec::new();
    match view.method {
        ImageMethod::Raster => {}
        ImageMethod::CpuRaytrace => {
            lights.extend(view.raytrace.lights.iter().flatten().map(|_| "render.tree_directional"));
            lights.extend(
                view.raytrace.punctual_lights.iter().flatten().map(|l| if l.cone.is_some() { "render.tree_spot" } else { "render.tree_point" }),
            );
            lights.extend(view.raytrace.area_lights.iter().flatten().map(|_| "render.tree_rectangle"));
        }
        ImageMethod::GpuRaytrace if view.studio.lighting_enabled => {
            if view.studio.sun_enabled {
                lights.push("render.tree_sun");
            }
            lights.extend(view.studio.area_lights.iter().map(|_| "render.tree_rectangle"));
        }
        ImageMethod::GpuRaytrace => {}
    }
    lights
}

pub(crate) fn environment(app: &SolveApp) -> (String, String) {
    let view = &app.render_workspace.view;
    let fixed = || (app.tr("render.tree_fixed_studio").to_string(), view.draft.environment_rotation, view.draft.environment_intensity);
    let source = |input: &hdr::Input| match input {
        hdr::Input::EmbeddedResource { resource } => format!("{} {resource}", app.tr("render.tree_hdr")),
        hdr::Input::FileCaptured { path, .. } | hdr::Input::FilePending { path } => {
            std::path::Path::new(path).file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| app.tr("render.tree_hdr").into())
        }
        hdr::Input::Empty => app.tr("render.tree_no_environment").into(),
    };
    let (name, rotation, intensity) = match view.method {
        ImageMethod::CpuRaytrace => match view.cpu.environment {
            cpu::Environment::Hdr => (source(&view.cpu.hdr), view.cpu.rotation_rad, view.cpu.intensity),
            cpu::Environment::Constant => (app.tr("render.tree_constant").into(), 0.0, view.cpu.intensity),
            cpu::Environment::None => fixed(),
        },
        ImageMethod::GpuRaytrace => match view.studio.environment {
            studio::Environment::Hdr => (source(&view.studio.hdr), view.studio.rotation_rad, view.studio.intensity),
            studio::Environment::Sky => (app.tr("render.tree_sky").into(), view.studio.rotation_rad, view.studio.intensity),
            studio::Environment::None => fixed(),
        },
        ImageMethod::Raster => fixed(),
    };
    (
        name,
        crate::i18n::format(
            "render.tree_environment_values",
            &[("rotation", &format!("{:.0}", rotation.to_degrees())), ("intensity", &format!("{intensity:.2}"))],
        ),
    )
}

pub(crate) fn sections(app: &mut SolveApp, ui: &mut egui::Ui) {
    setups::refresh_dirty(app);
    if section(app, ui, "environment") {
        let (name, detail) = environment(app);
        let row = browser::draw_row(
            ui,
            ui.id().with("render_environment"),
            &Row { depth: 1, label: &name, name: true, secondary: &detail, ..Default::default() },
        );
        if let Some(rect) = row.rect {
            crate::scenario::publish_target("render:tree:environment:current", rect);
        }
        if row.clicked {
            inspector::open_panel(app, inspector::PanelId::Environment);
        }
        if let Some(response) = row.resp {
            response.on_hover_text(format!("{name}\n{detail}"));
        }
    }
    if section(app, ui, "lights") {
        for (index, key) in light_types(app).iter().enumerate() {
            let label = app.tr(key).to_string();
            let row =
                browser::draw_row(ui, ui.id().with(("render_light", index)), &Row { depth: 1, label: &label, name: true, ..Default::default() });
            if row.clicked {
                inspector::open_panel(app, inspector::PanelId::Lights);
            }
        }
    }
    if section(app, ui, "shots") {
        let names: Vec<_> = app.session.doc.render.setups.keys().cloned().collect();
        for name in names {
            let active = name == app.render_workspace.view.name;
            let row = browser::draw_row(
                ui,
                ui.id().with(("render_shot", &name)),
                &Row {
                    depth: 1,
                    label: &name,
                    name: true,
                    radio: Some(active),
                    radio_hint: app.tr("render.tree_switch"),
                    modified: active && app.render_workspace.view.dirty,
                    ..Default::default()
                },
            );
            if let Some(rect) = row.rect {
                crate::scenario::publish_target(&format!("render:tree:shot:{name}"), rect);
            }
            if row.clicked || row.radio {
                super::switch_shot(app, name.clone());
            }
            if let Some(response) = row.resp {
                ui_kit::Menu::context(&response, |ui| {
                    for (command, key) in [
                        ("render.settings.apply", "render.tree_switch"),
                        ("render.shot.rename", "render.shot_rename"),
                        ("render.shot.duplicate", "render.shot_duplicate"),
                        ("render.shot.remove", "render.shot_remove"),
                    ] {
                        let item = Item::action(command, app.tr(key), "");
                        if context_menu::list_row(ui, &item).is_some() {
                            if command == "render.settings.apply" {
                                super::switch_shot(app, name.clone());
                            } else {
                                inspector::shot_row_action(app, name.clone(), command);
                            }
                            ui.close();
                        }
                    }
                });
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct LookGroup {
    pub look: Option<Look>,
    pub objects: Vec<String>,
}

pub(crate) fn groups(app: &SolveApp) -> Vec<LookGroup> {
    let mut groups: Vec<LookGroup> = Vec::new();
    for body in &app.session.world_state().bodies {
        for look in app.session.doc.model_used_looks(body) {
            if let Some(group) = groups.iter_mut().find(|g| g.look == look) {
                group.objects.push(body.name.clone());
            } else {
                groups.push(LookGroup { look, objects: vec![body.name.clone()] });
            }
        }
    }
    groups.sort_by_key(|g| look_name(app, g.look.as_ref()));
    groups
}

/// Returns true when the object section is closed or the By look lens drew its objects.
pub(crate) fn objects(app: &mut SolveApp, ui: &mut egui::Ui, hover_bodies: &mut Vec<String>) -> bool {
    if app.tree.reveal.is_some() {
        app.tree.collapsed.remove("render:tree:objects");
    }
    if !section(app, ui, "objects") {
        return true;
    }
    let labels = [app.tr("render.tree_by_object").to_string(), app.tr("render.tree_by_look").to_string()];
    let mut lens = usize::from(app.render_workspace.view.by_look);
    let top = ui.cursor().min;
    let width = ui.available_width();
    ui.push_id("render_browser_lens", |ui| {
        ui_kit::segmented(
            ui,
            &mut lens,
            &labels.iter().map(String::as_str).collect::<Vec<_>>(),
            ui_kit::Segments { compact: true, ..Default::default() },
        );
    });
    for (index, name) in ["render:tree:by-object", "render:tree:by-look"].iter().enumerate() {
        crate::scenario::publish_target(
            name,
            egui::Rect::from_min_size(top + egui::vec2(width * index as f32 / 2.0, 0.0), egui::vec2(width / 2.0, ui_kit::row_height())),
        );
    }
    app.render_workspace.view.by_look = lens == 1;
    if lens == 0 {
        return false;
    }
    for (index, group) in groups(app).into_iter().enumerate() {
        let key = format!("render:look:{:?}", group.look);
        if app.tree.reveal.as_ref().is_some_and(|b| group.objects.contains(b)) {
            app.tree.collapsed.remove(&key);
        }
        let open = browser::is_open(app, &key, true);
        let label = look_name(app, group.look.as_ref());
        let count = crate::i18n::format(
            if group.objects.len() == 1 { "render.tree_user_one" } else { "render.tree_users" },
            &[("count", &group.objects.len().to_string())],
        );
        let row = browser::draw_row(
            ui,
            ui.id().with(("render_look", &key)),
            &Row {
                depth: 1,
                label: &label,
                name: true,
                fold: Some(open),
                count: &count,
                swatch: Some(group.look.as_ref().map_or(Tokens::get().body, |l| {
                    let [r, g, b] = l.color;
                    Color32::from_rgb(r, g, b)
                })),
                ..Default::default()
            },
        );
        if let Some(rect) = row.rect {
            crate::scenario::publish_target(&format!("render:tree:look:{index}"), rect);
        }
        if row.fold {
            browser::toggle(app, &key, true);
        }
        let source = group.objects.first().and_then(|name| {
            let state = app.session.world_state();
            let body = state.body(name)?;
            let face = if app.session.doc.model_display_look(body) == group.look {
                None
            } else {
                app.session.doc.face_colors(body).into_iter().find(|(_, look)| Some(look) == group.look.as_ref()).map(|(index, _)| index)
            };
            Some(json!({"body":name,"face":face}))
        });
        if row.clicked
            && let Some(params) = &source
        {
            let _ = app.run("select.same_look", params.clone());
        }
        if let Some(response) = row.resp {
            ui_kit::Menu::context(&response, |ui| {
                let item = context_menu::cmd(app, "select.same_look", "Select Same Look");
                if context_menu::list_row(ui, &item).is_some()
                    && let Some(params) = &source
                {
                    let _ = app.run("select.same_look", params.clone());
                    ui.close();
                }
            });
        }
        if open {
            for name in group.objects {
                let decoration = decoration(app, std::slice::from_ref(&name));
                let visible = !app.session.visibility.hidden_bodies.contains(&name);
                let selected = app.session.selection.iter().any(|s| match s {
                    Sel::Occurrence { path } => app.session.occurrence_bodies(path, true).is_ok_and(|bodies| bodies.contains(&name)),
                    _ => browser::picked_body(s) == Some(&name),
                });
                let row = browser::draw_row(
                    ui,
                    ui.id().with((&key, &name)),
                    &Row {
                        depth: 2,
                        label: &name,
                        name: true,
                        eye: Some(visible),
                        selected,
                        dim: !app.session.body_visible(&name),
                        swatch: decoration.swatch,
                        secondary: &decoration.name,
                        count: &decoration.count,
                        ..Default::default()
                    },
                );
                if row.clicked {
                    let _ = app.run(
                        "select.set",
                        json!({"items":[Sel::Body {name:name.clone()}],"add":ui.input(|i|i.modifiers.command || i.modifiers.ctrl)}),
                    );
                }
                if row.eye {
                    let _ = app.run("browser.visibility", json!({"items":[name],"visible":!visible}));
                }
                if row.hovered {
                    *hover_bodies = vec![name.clone()];
                }
                if app.tree.reveal.as_ref() == Some(&name)
                    && let Some(rect) = row.rect
                {
                    ui.scroll_to_rect(rect, Some(egui::Align::Center));
                }
                if let Some(at) = row.secondary {
                    context_menu::open_for(app, at, context_menu::Target::Body { name });
                }
            }
        }
    }
    true
}
