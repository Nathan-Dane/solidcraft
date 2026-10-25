//! Existing Render-panel library browser. Engine commands own catalog resolution,
//! assignments and saved resources; this view owns only search and request drafts.
use super::{ImageMethod, materials, setups};
use crate::{SolveApp, theme::Tokens, ui_kit};
use serde_json::{Value, json};
use solidcraft_engine::{doc::Document, geom::Vec3};
use std::{
    collections::BTreeMap,
    sync::{Arc, Weak},
};

#[derive(Clone, Debug)]
pub(super) struct View {
    kind: String,
    query: String,
    entries: Vec<Value>,
    selected: Option<String>,
    details: Option<Value>,
    loaded: bool,
    offset: usize,
    total: usize,
    next: Option<usize>,
    target_query: String,
    targets: Vec<materials::Target>,
    target: Option<materials::Target>,
    target_revision: Option<u64>,
    target_document: Option<Weak<Document>>,
    acknowledge_projection: bool,
    replace_environment: bool,
    replace_studio: bool,
    replace_surface: bool,
    include_cards: bool,
    physical_name: String,
    physical_ack: bool,
    physical_plan: Option<Value>,
    center: [f64; 3],
    extent: f64,
    availability: String,
    collection: usize,
    collection_query: String,
    navigation: BTreeMap<String, Navigation>,
    category: String,
    layout: usize,
    revision: Option<u64>,
    document: Option<Weak<Document>>,
    labels: BTreeMap<(String, String), String>,
    statuses: BTreeMap<(String, String), String>,
    preview: Option<SourcePreview>,
    preview_error: Option<String>,
    mapping_plane: usize,
    mapping_scale: f64,
}
#[derive(Clone, Debug)]
struct Navigation {
    query: String,
    offset: usize,
    selected: Option<String>,
    details: Option<Value>,
    availability: String,
}
#[derive(Clone)]
struct SourcePreview {
    id: String,
    sha256: String,
    texture: egui::TextureHandle,
}
impl std::fmt::Debug for SourcePreview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourcePreview").field("id", &self.id).field("sha256", &self.sha256).finish()
    }
}
impl Default for View {
    fn default() -> Self {
        Self {
            kind: "appearances".into(),
            query: String::new(),
            entries: Vec::new(),
            selected: None,
            details: None,
            loaded: false,
            offset: 0,
            total: 0,
            next: None,
            target_query: String::new(),
            targets: Vec::new(),
            target: None,
            target_revision: None,
            target_document: None,
            acknowledge_projection: false,
            replace_environment: false,
            replace_studio: false,
            replace_surface: false,
            include_cards: true,
            physical_name: "Reference material".into(),
            physical_ack: false,
            physical_plan: None,
            center: [0.0; 3],
            extent: 1000.0,
            availability: "all".into(),
            collection: 0,
            collection_query: String::new(),
            navigation: Default::default(),
            category: String::new(),
            layout: 0,
            revision: None,
            document: None,
            labels: Default::default(),
            statuses: Default::default(),
            preview: None,
            preview_error: None,
            mapping_plane: 0,
            mapping_scale: 1.0,
        }
    }
}

fn refresh(app: &mut SolveApp, view: &mut View) -> Result<(), String> {
    if matches!(view.collection, 1 | 2) {
        let keys: Vec<_> = if view.collection == 1 {
            app.preferences.render_library_favorites.iter().take(256).cloned().collect()
        } else {
            app.preferences.render_library_recent.iter().take(32).cloned().collect()
        };
        view.entries = keys
            .into_iter()
            .filter_map(|(kind, id)| {
                let detail = describe(app, &kind, &id).ok()?;
                let mut entry = detail["resource"].clone();
                entry["resource"] = detail["resource"].clone();
                entry["projection"] = detail["projection"].clone();
                entry["library_kind"] = json!(kind);
                entry["catalog_identity"] = json!(id);
                Some(entry)
            })
            .collect();
        view.total = view.entries.len();
        view.next = None;
    } else if view.collection == 3 || view.kind == "appearances" {
        let reply = app.run(if view.collection == 3 { "appearance.list" } else { "appearance.library" }, json!({}))?;
        view.entries = reply[if view.collection == 3 { "in_design" } else { "appearances" }].as_array().cloned().unwrap_or_default();
        view.total = view.entries.len();
        view.next = None;
    } else {
        let reply = app.run("render.library.list", json!({"kind":view.kind,"query":view.query,"offset":view.offset,"limit":48,"availability":if view.kind=="physical_profiles" {"all"} else {&view.availability}}))?;
        view.entries = reply["items"].as_array().cloned().ok_or_else(|| "invalid render-library response".to_string())?;
        // Capture authored optical parameters once per page; selection remains read-only.
        for entry in &mut view.entries {
            if let Some(id) = entry["id"].as_str() {
                let detail = app.run("render.library.describe", json!({"kind":view.kind,"id":id}))?;
                entry["resource"] = detail["resource"].clone();
            }
        }
        view.total = reply["total"].as_u64().and_then(|n| usize::try_from(n).ok()).unwrap_or(0);
        view.next = reply["next_offset"].as_u64().and_then(|n| usize::try_from(n).ok());
    }
    view.revision = Some(app.session.revision);
    view.document = Some(Arc::downgrade(&app.session.doc));
    view.loaded = true;
    // Search/page changes preserve the independently inspected resource.
    view.acknowledge_projection = false;
    view.physical_ack = false;
    view.physical_plan = None;
    Ok(())
}
fn current_target(app: &SolveApp, view: &View) -> bool {
    view.target.is_some()
        && view.target_revision == Some(app.session.revision)
        && view.target_document.as_ref().is_some_and(|d| Weak::ptr_eq(d, &Arc::downgrade(&app.session.doc)))
        && !app.session.model.is_stale()
}
fn refresh_targets(app: &mut SolveApp, view: &mut View) -> Result<(), String> {
    let reply = app.run("render.library.targets", json!({"query":view.target_query,"limit":128}))?;
    view.targets = serde_json::from_value(reply["items"].clone()).map_err(|e| e.to_string())?;
    view.target = None;
    view.target_revision = Some(app.session.revision);
    view.target_document = Some(Arc::downgrade(&app.session.doc));
    if let (Ok(min), Ok(max)) =
        (serde_json::from_value::<Vec3>(reply["bounds"]["min"].clone()), serde_json::from_value::<Vec3>(reply["bounds"]["max"].clone()))
    {
        let c = (min + max) * 0.5;
        let size = max - min;
        let extent = size.x.max(size.y).max(size.z);
        if extent.is_finite() && (0.001..=1e6).contains(&extent) {
            view.center = [c.x, c.y, c.z];
            view.extent = extent;
        }
    }
    Ok(())
}
fn target_controls(app: &mut SolveApp, ui: &mut egui::Ui, view: &mut View) {
    ui.label(app.tr("render.library_target"));
    ui_kit::bounded_cell_editor(
        ui,
        ui.id().with("render_library_target_search"),
        &mut view.target_query,
        app.tr("render.material_filter"),
        false,
        128,
    );
    if ui_kit::button(ui, app.tr("render.material_refresh"), ui_kit::Button::Normal).clicked()
        && let Err(e) = refresh_targets(app, view)
    {
        app.set_status(e, true);
    }
    let label = view.target.as_ref().map_or_else(|| app.tr("render.material_choose_body").to_string(), |t| t.name.clone());
    ui_kit::dropdown_menu(ui, "render_library_target", &label, |ui| {
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for target in &view.targets {
                if ui_kit::list_picker(ui, &format!("library-target:{}", target.ordinal), &target.name, view.target.as_ref() == Some(target))
                    .clicked()
                {
                    view.target = Some(target.clone());
                    ui.close();
                }
            }
        });
    });
    if view.target.is_some() && !current_target(app, view) {
        ui_kit::text(ui, app.tr("render.material_revalidate_needed"), ui_kit::Text::Note("render.material_revalidate_needed"));
    }
}
fn patch(app: &SolveApp) -> Result<Value, String> {
    serde_json::to_value(setups::snapshot(app)?).map_err(|e| e.to_string())
}

const KINDS: &[(&str, &str)] = &[
    ("appearances", "render.appearances"),
    ("native_scenes", "render.library_native_scenes"),
    ("surfaces", "render.library_surfaces"),
    ("hdr", "render.library_hdr"),
    ("rigs", "render.library_rigs"),
    ("selene_appearances", "render.library_selene"),
    ("physical_profiles", "render.library_physical_profiles"),
    ("scenes", "render.library_scenes"),
    ("selene_shots", "render.library_selene_shots"),
    ("materials", "render.library_material_concepts"),
    ("lighting", "render.library_light_concepts"),
    ("environments", "render.library_environment_concepts"),
    ("quality", "render.library_quality_concepts"),
    ("capabilities", "render.library_capabilities"),
];

fn switch_kind(view: &mut View, kind: &str) {
    if view.kind == kind || !KINDS.iter().any(|(value, _)| *value == kind) {
        return;
    }
    view.navigation.insert(
        view.kind.clone(),
        Navigation {
            query: view.query.clone(),
            offset: view.offset,
            selected: view.selected.clone(),
            details: view.details.clone(),
            availability: view.availability.clone(),
        },
    );
    view.kind = kind.into();
    if let Some(saved) = view.navigation.get(kind) {
        view.query = saved.query.clone();
        view.offset = saved.offset;
        view.selected = saved.selected.clone();
        view.details = saved.details.clone();
        view.availability = saved.availability.clone();
    } else {
        view.query.clear();
        view.offset = 0;
        view.selected = None;
        view.details = None;
        view.availability = "all".into();
    }
    view.loaded = false;
    view.category.clear();
    view.preview = None;
    view.preview_error = None;
    view.physical_plan = None;
    view.acknowledge_projection = false;
    view.physical_ack = false;
}
pub(super) fn open_category(app: &mut SolveApp, kind: &str) {
    switch_kind(&mut app.render_workspace.view.library, kind);
    app.render_workspace.view.library.collection = 0;
    app.ui.show_browser = true;
    app.tree.panel_collapsed = false;
}
fn choose(app: &mut SolveApp, view: &mut View, kind: &str, id: &str) -> Result<(), String> {
    switch_kind(view, kind);
    let detail = describe(app, kind, id)?;
    let display = detail["resource"]["name"].as_str().or_else(|| detail["resource"]["designation"].as_str()).unwrap_or(id).to_string();
    let status = detail["projection"]["status"].as_str().unwrap_or("reference_only").to_string();
    view.details = Some(detail);
    view.selected = Some(id.into());
    view.preview = None;
    view.preview_error = None;
    view.acknowledge_projection = false;
    view.physical_ack = false;
    view.physical_plan = None;
    app.render_workspace.view.inspector.clear_resource_reviews();
    let key = (kind.to_string(), id.to_string());
    app.preferences.render_library_recent.retain(|old| old != &key);
    app.preferences.render_library_recent.push_front(key.clone());
    app.preferences.render_library_recent.truncate(32);
    if matches!(view.collection, 1 | 2) {
        view.loaded = false;
    }
    view.statuses.insert(key.clone(), status);
    view.labels.insert(key, display);
    view.labels.retain(|key, _| app.preferences.render_library_favorites.contains(key) || app.preferences.render_library_recent.contains(key));
    view.statuses.retain(|key, _| app.preferences.render_library_favorites.contains(key) || app.preferences.render_library_recent.contains(key));
    let panel = match kind {
        "native_scenes" | "scenes" | "selene_shots" => super::inspector::PanelId::Studio,
        "hdr" | "environments" => super::inspector::PanelId::Environment,
        "rigs" | "lighting" => super::inspector::PanelId::Lights,
        "quality" => super::inspector::PanelId::Image,
        "capabilities" => super::inspector::PanelId::Check,
        _ => super::inspector::PanelId::Surface,
    };
    super::inspector::open_panel(app, panel);
    Ok(())
}

/// Resolve previews and Inspector metadata through the same public catalog commands.
fn look_signature(entry: &Value) -> Value {
    json!([entry["name"], entry["color"], entry["opacity"], entry["pbr"]])
}
fn appearance_id(entry: &Value) -> String {
    entry["id"].as_str().map(str::to_owned).unwrap_or_else(|| look_signature(entry).to_string())
}
fn describe(app: &mut SolveApp, kind: &str, id: &str) -> Result<Value, String> {
    if kind != "appearances" {
        return app.run("render.library.describe", json!({"kind":kind,"id":id}));
    }
    for (command, field) in [("appearance.library", "appearances"), ("appearance.list", "in_design")] {
        let reply = app.run(command, json!({}))?;
        if let Some(resource) = reply[field].as_array().and_then(|items| items.iter().find(|item| appearance_id(item) == id)) {
            return Ok(
                json!({"resource":resource,"projection":{"status":if resource["available"] == false { "blocked" } else { "native_appearance" }}}),
            );
        }
    }
    // Custom looks carry their complete scalar definition in their preference key,
    // so a favourite survives closing its originating design.
    if let Ok(Value::Array(parts)) = serde_json::from_str::<Value>(id)
        && parts.len() == 4
    {
        return Ok(
            json!({"resource":{"name":parts.first(),"color":parts.get(1),"opacity":parts.get(2),"pbr":parts.get(3)},"projection":{"status":"native_appearance"}}),
        );
    }
    Err("appearance is no longer available".into())
}

fn preview_parameters(entry: &Value) -> Value {
    let source = entry.get("resource").unwrap_or(entry);
    if let Some(parameters) = source.pointer("/baseline_projection/runtime_parameters") {
        return parameters.clone();
    }
    if let (Some(low), Some(high)) = (source["base_linear_low"].as_array(), source["base_linear_high"].as_array()) {
        let color: Vec<_> = low
            .iter()
            .zip(high)
            .take(3)
            .map(|(a, b)| {
                let linear = ((a.as_f64().unwrap_or(0.0) + b.as_f64().unwrap_or(0.0)) * 0.5).clamp(0.0, 1.0);
                let encoded = if linear <= 0.0031308 { linear * 12.92 } else { 1.055 * linear.powf(1.0 / 2.4) - 0.055 };
                (encoded * 255.0).round() as u8
            })
            .collect();
        return json!({"color":color,"opacity":1.0,"pbr":{"metallic":source["metallic"],"roughness":source["roughness"],"dielectric_ior":1.5}});
    }
    source.clone()
}
fn tile_sphere(entry: &Value) -> ui_kit::Sphere {
    let p = preview_parameters(entry);
    let color = crate::dialogs_appearance::parse_color(&p["color"]).map(|[r, g, b]| egui::Color32::from_rgb(r, g, b)).unwrap_or(Tokens::get().body);
    let scalar = |key: &str, default: f64| p["pbr"][key].as_f64().filter(|v| v.is_finite()).unwrap_or(default).clamp(0.0, 1.0);
    let opacity = p["opacity"].as_f64().filter(|v| v.is_finite()).unwrap_or(1.0).clamp(0.0, 1.0);
    let kind = if opacity < 1.0 {
        ui_kit::FinishKind::Glass
    } else if scalar("metallic", 0.0) >= 0.5 {
        ui_kit::FinishKind::Metal
    } else if scalar("roughness", 0.5) <= 0.2 {
        ui_kit::FinishKind::Gloss
    } else if scalar("roughness", 0.5) >= 0.7 {
        ui_kit::FinishKind::Matte
    } else {
        ui_kit::FinishKind::Satin
    };
    ui_kit::Sphere { color, kind, opacity: Some(opacity as f32) }
}
fn user_count(entry: &Value) -> usize {
    ["bodies", "faces", "components"].iter().filter_map(|key| entry[*key].as_array()).map(Vec::len).fold(0, usize::saturating_add)
}
fn usage_for<'a>(entry: &Value, used: &'a [Value]) -> Option<&'a Value> {
    let source = entry.get("resource").unwrap_or(entry);
    let parameters = preview_parameters(entry);
    let name = source["name"].as_str()?;
    let name = if source.get("base_linear_low").is_some() || source.pointer("/baseline_projection/runtime_parameters").is_some() {
        format!("{} {name}", entry["id"].as_str()?)
    } else {
        name.to_string()
    };
    let color = crate::dialogs_appearance::parse_color(&parameters["color"])?;
    used.iter().find(|look| {
        look["name"].as_str() == Some(&name)
            && crate::dialogs_appearance::parse_color(&look["color"]) == Some(color)
            && look["opacity"].as_f64() == parameters["opacity"].as_f64()
            && ["metallic", "roughness", "dielectric_ior"].iter().all(|key| look["pbr"][*key].as_f64() == parameters["pbr"][*key].as_f64())
    })
}
fn matches_entry(entry: &Value, query: &str, category: &str, availability: &str) -> bool {
    let state = entry.pointer("/projection/status").and_then(Value::as_str).unwrap_or(if entry["available"] == false {
        "blocked"
    } else {
        "native_appearance"
    });
    let admitted = match availability {
        "usable" => matches!(state, "native_appearance" | "native_composition" | "implemented_pending_batch_validation"),
        "blocked" => state == "blocked",
        "specification" => state == "specification_only",
        _ => true,
    };
    let source = entry.get("resource").unwrap_or(entry);
    let query = query.to_lowercase();
    admitted
        && (category.is_empty() || entry["category"].as_str() == Some(category))
        && ["id", "name", "category", "family", "description", "domains", "process", "intended_use", "tags"]
            .iter()
            .filter_map(|field| source.get(*field))
            .any(|field| field.to_string().to_lowercase().contains(&query))
}

pub(super) fn card(app: &mut SolveApp, ui: &mut egui::Ui, bounds: egui::Rect) {
    let mut side = app.preferences.render_browser_side.clone();
    let mut width = app.preferences.render_browser_width;
    let title = app.tr("render.appearances").to_string();
    let mut layout = app.render_workspace.view.library.layout;
    let mut actions = |ui: &mut egui::Ui| {
        ui_kit::segmented_icons(
            ui,
            &mut layout,
            &[
                ui_kit::IconSegment { key: "render:library-grid", icon: "grid", tip: crate::i18n::tr("render.library_grid").into() },
                ui_kit::IconSegment { key: "render:library-list", icon: "list", tip: crate::i18n::tr("render.library_list").into() },
            ],
            &[],
        );
    };
    let response = ui_kit::Panel::workspace(
        ui,
        ui_kit::WorkspaceOptions {
            id: "render:appearances",
            title: &title,
            icon: "appearance",
            bounds,
            fill_height: true,
            header_actions: Some(&mut actions),
        },
        &mut side,
        &mut width,
        |ui| browse(app, ui),
    );
    app.render_workspace.view.library.layout = layout;
    app.preferences.render_browser_width = width;
    if app.preferences.render_browser_side != side {
        app.preferences.render_inspector_side = if side == "left" { "right" } else { "left" }.into();
    }
    app.preferences.render_browser_side = side.clone();
    crate::shell::register_panel(
        app,
        "render:appearances",
        response.map(|r| r.response.rect),
        Some(if side == "left" { ui_kit::Edge::Left } else { ui_kit::Edge::Right }),
    );
}

pub(super) fn browse(app: &mut SolveApp, ui: &mut egui::Ui) {
    let mut view = std::mem::take(&mut app.render_workspace.view.library);
    let t = Tokens::get();
    let query = if view.collection == 0 { &mut view.query } else { &mut view.collection_query };
    let search = ui_kit::search_field(ui, ui.id().with("render_library_search"), query, app.tr("render.library_search"));
    query.truncate(query.char_indices().nth(128).map_or(query.len(), |(at, _)| at));
    crate::scenario::publish_target("render:library-search", search.rect);
    if search.changed() {
        view.offset = 0;
        view.loaded = false;
    }
    let labels = [
        app.tr("render.library_in_design").to_string(),
        app.tr("render.browser_library").to_string(),
        app.tr("render.library_recent").to_string(),
        "★".to_string(),
    ];
    let mut scope = match view.collection {
        3 => 0,
        0 => 1,
        2 => 2,
        _ => 3,
    };
    let top = ui.cursor().min;
    let scope_width = ui.available_width();
    if ui_kit::segmented(
        ui,
        &mut scope,
        &labels.iter().map(String::as_str).collect::<Vec<_>>(),
        ui_kit::Segments { compact: true, proportional: true, ..Default::default() },
    ) {
        view.collection = [3, 0, 2, 1].get(scope).copied().unwrap_or(0);
        view.loaded = false;
        view.offset = 0;
        view.category.clear();
    }
    // Publish stable semantic scope targets, including the narrow star segment.
    let natural: Vec<_> = labels
        .iter()
        .map(|label| ui.painter().layout_no_wrap(label.clone(), crate::fonts::semibold(t.font_label), t.form_ink).size().x.ceil() + t.space_16)
        .collect();
    let total: f32 = natural.iter().sum();
    let mut x = top.x;
    for (key, natural) in ["in-design", "library", "recent", "favorites"].iter().zip(natural) {
        let width = scope_width * natural / total.max(1.0);
        crate::scenario::publish_target(
            &format!("render:library-scope:{key}"),
            egui::Rect::from_min_size(egui::pos2(x, top.y), egui::vec2(width, t.control_height)),
        );
        x += width;
    }
    let document_changed =
        view.revision != Some(app.session.revision) || view.document.as_ref().is_none_or(|doc| !Weak::ptr_eq(doc, &Arc::downgrade(&app.session.doc)));
    if (!view.loaded || document_changed)
        && let Err(error) = refresh(app, &mut view)
    {
        view.loaded = true;
        app.set_status(error, true);
    }
    let mut entries: Vec<(String, String, Value)> = view
        .entries
        .iter()
        .map(|entry| {
            let appearance = view.collection == 3 || view.kind == "appearances";
            (
                entry["library_kind"]
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| if appearance { "appearances".into() } else { view.kind.clone() }),
                entry["catalog_identity"]
                    .as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| if appearance { appearance_id(entry) } else { entry["id"].as_str().unwrap_or("").to_string() }),
                entry.clone(),
            )
        })
        .collect();
    let categories: std::collections::BTreeSet<_> =
        entries.iter().filter_map(|(_, _, entry)| entry["category"].as_str().map(str::to_owned)).collect();
    if !categories.is_empty() {
        egui::ScrollArea::horizontal().id_salt("render_library_chips").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui_kit::choice_chip(ui, "render-library-all", app.tr("ui.all"), view.category.is_empty()).clicked() {
                    view.category.clear();
                }
                for category in &categories {
                    if ui_kit::choice_chip(ui, category, category, view.category == *category).clicked() {
                        view.category.clone_from(category);
                    }
                }
            });
        });
    }
    let query = if view.collection == 0 { &view.query } else { &view.collection_query };
    entries.retain(|(_, _, entry)| matches_entry(entry, query, &view.category, &view.availability));
    let used = app.run("appearance.list", json!({})).ok().and_then(|reply| reply["in_design"].as_array().cloned()).unwrap_or_default();
    let tips: Vec<_> = entries
        .iter()
        .map(|(_, id, entry)| {
            let p = preview_parameters(entry);
            let usage = usage_for(entry, &used).unwrap_or(entry);
            let users = user_count(usage);
            let names: Vec<_> = ["bodies", "faces", "components"]
                .iter()
                .filter_map(|key| usage[*key].as_array())
                .flatten()
                .take(128)
                .filter_map(|user| user.as_str().or_else(|| user["body"].as_str()))
                .collect();
            let scalar = |value: &Value| value.as_f64().map(|n| format!("{n:.2}")).unwrap_or_else(|| "—".into());
            format!(
                "{}\n{}\n{}: {} · {}: {}\n{}: {} · {}: {}\n{}: {} {}\n{}",
                entry["name"].as_str().unwrap_or(id),
                entry["category"].as_str().or_else(|| entry["family"].as_str()).unwrap_or(app.tr("render.library_in_design")),
                app.tr("render.library_metallic"),
                scalar(&p["pbr"]["metallic"]),
                app.tr("render.library_roughness"),
                scalar(&p["pbr"]["roughness"]),
                app.tr("render.library_ior"),
                scalar(&p["pbr"]["dielectric_ior"]),
                app.tr("render.library_opacity"),
                scalar(&p["opacity"]),
                app.tr("render.library_users"),
                users,
                names.join(", "),
                availability_label(
                    app,
                    entry.pointer("/projection/status").and_then(Value::as_str).or(Some(if entry["available"] == false {
                        "blocked"
                    } else {
                        "native_appearance"
                    }))
                )
            )
        })
        .collect();
    let identities: Vec<_> = entries.iter().map(|(_, id, _)| format!("render:library:{id}")).collect();
    let footer = t.control_height * if view.collection == 0 && view.kind != "appearances" { 3.0 } else { 2.0 } + t.space_16;
    let height = (ui.available_height() - footer).max(t.control_height);
    let mut selected = None;
    egui::ScrollArea::vertical().id_salt(("render_library_items", &view.kind, view.collection)).max_height(height).auto_shrink([false, false]).show(
        ui,
        |ui| {
            if entries.is_empty() {
                ui_kit::text(ui, app.tr("render.library_empty"), ui_kit::Text::Note("render.library_empty"));
            }
            if view.layout == 0 {
                let tiles: Vec<_> = entries
                    .iter()
                    .zip(&identities)
                    .zip(&tips)
                    .map(|(((kind, id, entry), identity), tip)| ui_kit::SphereTile {
                        id: identity,
                        name: entry["name"].as_str().unwrap_or(id),
                        sphere: tile_sphere(entry),
                        selected: view.kind == *kind && view.selected.as_ref() == Some(id),
                        users: usage_for(entry, &used).map_or_else(|| user_count(entry), user_count),
                        tip,
                    })
                    .collect();
                selected = ui_kit::sphere_grid(ui, &tiles);
            } else {
                for (index, ((kind, id, entry), tip)) in entries.iter().zip(&tips).enumerate() {
                    let mut item = ui_kit::MenuItem::new(id, entry["name"].as_str().unwrap_or(id), "appearance");
                    item.highlighted = view.kind == *kind && view.selected.as_ref() == Some(id);
                    let r = item.search_row(
                        ui,
                        query,
                        &format!(
                            "{} / {}",
                            if kind == "appearances" { entry["category"].as_str().unwrap_or(app.tr("render.library_in_design")) } else { id },
                            short_availability(app, entry.pointer("/projection/status").and_then(Value::as_str))
                        ),
                        None,
                    );
                    crate::scenario::publish_target(identities.get(index).map(String::as_str).unwrap_or("render:library:item"), r.rect);
                    if r.on_hover_text(tip).clicked() {
                        selected = Some(index);
                    }
                }
            }
        },
    );
    let menu = ui_kit::dropdown_menu(ui, "render_library_filters", app.tr("render.library_filters"), |ui| {
        if view.collection == 0 {
            for &(kind, key) in KINDS {
                let r = ui_kit::list_picker(ui, kind, app.tr(key), view.kind == kind);
                crate::scenario::publish_target(&format!("render:library-kind:{kind}"), r.rect);
                if r.clicked() {
                    switch_kind(&mut view, kind);
                    ui.close();
                }
            }
            ui_kit::divider(ui, "render-library-availability");
        }
        for (id, key) in [
            ("all", "render.library_all_states"),
            ("usable", "render.library_usable"),
            ("specification", "render.library_specs"),
            ("blocked", "render.library_blocked"),
        ] {
            if ui_kit::list_picker(ui, id, app.tr(key), view.availability == id).clicked() {
                view.availability = id.into();
                view.offset = 0;
                view.loaded = false;
                ui.close();
            }
        }
    });
    crate::scenario::publish_target("render:library-category", menu.rect);
    if view.collection == 0 && view.kind != "appearances" {
        ui.horizontal(|ui| {
            if ui_kit::button(ui, app.tr("render.library_previous"), ui_kit::Button::Enabled(view.offset > 0)).clicked() {
                view.offset = view.offset.saturating_sub(48);
                view.loaded = false;
            }
            if ui_kit::button(ui, app.tr("render.library_next"), ui_kit::Button::Enabled(view.next.is_some())).clicked() {
                view.offset = view.next.unwrap_or(view.offset);
                view.loaded = false;
            }
        });
    }
    if let Some((kind, id, _)) = selected.and_then(|index| entries.get(index))
        && let Err(error) = choose(app, &mut view, kind, id)
    {
        app.set_status(error, true);
    }
    ui_kit::text(ui, app.tr("render.library_inspect_only"), ui_kit::Text::Note("render.library_inspect_only"));
    app.render_workspace.view.library = view;
}

pub(super) fn selected_kind(app: &SolveApp, kind: &str) -> bool {
    let view = &app.render_workspace.view.library;
    view.kind == kind && view.selected.is_some()
}

pub(super) fn inspect_kinds(app: &mut SolveApp, ui: &mut egui::Ui, kinds: &[&str]) {
    if kinds.contains(&app.render_workspace.view.library.kind.as_str())
        || (kinds.contains(&"surfaces") && app.render_workspace.view.library.kind == "appearances")
    {
        inspect(app, ui);
    } else {
        ui_kit::text(ui, app.tr("render.library_select_hint"), ui_kit::Text::Note("render.library_select_hint"));
    }
}
pub(super) fn inspect(app: &mut SolveApp, ui: &mut egui::Ui) {
    let mut view = std::mem::take(&mut app.render_workspace.view.library);
    if let Some(id) = view.selected.clone() {
        ui.strong(view.details.as_ref().and_then(|d| d["resource"]["name"].as_str()).unwrap_or(&id));
        if view.kind != "appearances" {
            ui.small(&id);
        }
        // Native composition is an actionable review, not a long specification
        // reader: keep Resolve/consent/Apply ahead of ancillary resource metadata.
        let native_composition = view.kind == "native_scenes";
        if native_composition {
            super::inspector::scene_resource(app, ui, &id);
        }
        let key = (view.kind.clone(), id.clone());
        let favorite = app.preferences.render_library_favorites.contains(&key);
        if ui_kit::button(
            ui,
            app.tr(if favorite { "render.library_unfavorite" } else { "render.library_favorite" }),
            ui_kit::Button::Enabled(favorite || app.preferences.render_library_favorites.len() < 256),
        )
        .clicked()
        {
            if favorite {
                app.preferences.render_library_favorites.remove(&key);
            } else {
                app.preferences.render_library_favorites.insert(key);
            }
            view.loaded = false;
        }
        if let Some(detail) = &view.details {
            ui.small(format!("{}: {}", app.tr("render.library_status"), availability_label(app, detail["projection"]["status"].as_str())));
            if let Some(description) = detail["resource"]["description"].as_str() {
                ui.label(description);
            }
            if let Some(family) = detail["resource"]["family"].as_str() {
                ui.small(family);
            }
            if let Some(omitted) = detail["projection"]["omitted"].as_array() {
                for item in omitted {
                    ui.small(format!("{}: {}", app.tr("render.library_omitted"), item.as_str().unwrap_or("unknown")));
                }
            }
        }
        if view.kind == "surfaces" {
            source_preview(app, ui, &mut view, &id);
        }
        if !native_composition {
            resource_controls(app, ui, &mut view, &id);
        }
        if let Some(detail) = &view.details {
            let title = app.tr("render.library_details").to_string();
            ui_kit::section(ui, ("render_library_detail", &id), &title, false, |ui| {
                let text = serde_json::to_string_pretty(detail).unwrap_or_default();
                if ui_kit::button(ui, app.tr("render.copy_request"), ui_kit::Button::Normal).clicked() {
                    ui.ctx().copy_text(text.clone());
                }
                egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                    ui.monospace(text);
                });
            });
        }
    } else {
        ui_kit::text(ui, app.tr("render.library_select_hint"), ui_kit::Text::Note("render.library_select_hint"));
    }
    app.render_workspace.view.library = view;
}
fn source_preview(app: &mut SolveApp, ui: &mut egui::Ui, view: &mut View, id: &str) {
    if view.preview.as_ref().is_none_or(|preview| preview.id != id)
        && view.preview_error.is_none()
        && ui_kit::button(ui, app.tr("render.library_source_preview"), ui_kit::Button::Normal).clicked()
    {
        let result = (|| -> Result<SourcePreview, String> {
            let reply = app.run("render.library.preview", json!({"id":id}))?;
            let bytes: Vec<u8> = serde_json::from_value(reply["bytes"].clone()).map_err(|e| e.to_string())?;
            if bytes.len() > 2 * 1024 * 1024 {
                return Err("source preview exceeds its byte envelope".into());
            }
            let mut reader = image::ImageReader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::Png);
            let mut limits = image::Limits::default();
            limits.max_image_width = Some(1024);
            limits.max_image_height = Some(1024);
            limits.max_alloc = Some(16 * 1024 * 1024);
            reader.limits(limits);
            let pixels = reader.decode().map_err(|e| e.to_string())?.to_rgba8();
            let size = [pixels.width() as usize, pixels.height() as usize];
            let image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_raw());
            Ok(SourcePreview {
                id: id.into(),
                sha256: reply["sha256"].as_str().unwrap_or("").into(),
                texture: ui.ctx().load_texture(format!("source-map-{id}"), image, egui::TextureOptions::LINEAR),
            })
        })();
        match result {
            Ok(preview) => view.preview = Some(preview),
            Err(e) => view.preview_error = Some(e),
        }
    }
    if let Some(preview) = &view.preview {
        ui.add(egui::Image::new(&preview.texture).max_size(egui::vec2(ui.available_width(), 180.0)).shrink_to_fit());
        ui_kit::text(ui, app.tr("render.library_source_preview_help"), ui_kit::Text::Note("render.library_source_preview_help"));
    }
    if let Some(error) = &view.preview_error {
        ui.small(error);
    }
}
#[cfg(test)]
pub(super) fn controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    stage_controls(app, ui);
    saved_surface_controls(app, ui);
    let title = app.tr("render.library_title").to_string();
    ui_kit::section(ui, "render_resource_library", &title, false, |ui| {
        browse(app, ui);
        inspect(app, ui);
    });
}

fn resource_controls(app: &mut SolveApp, ui: &mut egui::Ui, view: &mut View, id: &str) {
    let Some(detail) = view.details.clone() else { return };
    let source = &detail["resource"];
    if let Some(description) = source["description"].as_str().or_else(|| source["purpose"].as_str()) {
        ui.small(description);
    }
    let mut linked_profile = None;
    if let Some(specifications) = detail["physical_references"]["specifications"].as_array() {
        let title = app.tr("render.physical_associations").to_string();
        ui_kit::section(ui, ("physical_associations", id), &title, false, |ui| {
            ui_kit::text(ui, app.tr("render.physical_separate"), ui_kit::Text::Note("render.physical_separate"));
            for spec in specifications {
                ui.label(format!("{}: {}", spec["catalog_id"].as_str().unwrap_or(""), spec["physical_role"].as_str().unwrap_or("")));
                ui.small(spec["reason"].as_str().unwrap_or(""));
                if let Some(references) = spec["references"].as_array() {
                    for reference in references {
                        ui.label(format!("{} - {}", reference["profile_id"].as_str().unwrap_or(""), reference["relation"].as_str().unwrap_or("")));
                    }
                }
            }
            if let Some(profiles) = detail["physical_references"]["candidate_profiles"].as_array() {
                if profiles.is_empty() {
                    ui_kit::text(ui, app.tr("render.physical_unknown"), ui_kit::Text::Note("render.physical_unknown"));
                }
                for profile in profiles {
                    let profile_id = profile["id"].as_str().unwrap_or("");
                    if ui_kit::button(ui, profile["name"].as_str().unwrap_or(profile_id), ui_kit::Button::Normal).clicked() {
                        linked_profile = Some(profile_id.to_string());
                    }
                }
            }
        });
    }
    if let Some(profile_id) = linked_profile {
        if let Err(error) = choose(app, view, "physical_profiles", &profile_id) {
            app.set_status(error, true);
        }
        // The next frame uses the newly captured resource. Do not combine the
        // previous surface's detail with the newly selected physical profile.
        return;
    }
    if view.kind != "appearances" {
        ui_kit::text(ui, app.tr("render.library_authored_help"), ui_kit::Text::Note("render.library_authored_help"));
    }
    match view.kind.as_str() {
        "appearances" => {
            target_controls(app, ui, view);
            if ui_kit::button(
                ui,
                app.tr("render.library_apply_appearance"),
                ui_kit::Button::Enabled(current_target(app, view) && source["available"] != false),
            )
            .clicked()
                && let Some(target) = &view.target
            {
                let params = if let Some(preset) = source["id"].as_str() {
                    json!({"bodies":[target.name],"appearance":preset})
                } else {
                    json!({"bodies":[target.name],"name":source["name"],"color":source["color"],"opacity":source["opacity"],"pbr":source["pbr"]})
                };
                match app.run("appearance.assign", params) {
                    Ok(_) => {
                        view.target_revision = None;
                        view.loaded = false;
                        app.set_status(app.tr("render.library_applied").to_string(), false);
                    }
                    Err(error) => app.set_status(error, true),
                }
            }
        }
        "native_scenes" => super::inspector::scene_resource(app, ui, id),
        "quality" => super::inspector::quality_resource(app, ui, id),
        "physical_profiles" => physical_controls(app, ui, view, id, &detail),
        "surfaces" | "selene_appearances" => {
            target_controls(app, ui, view);
            ui_kit::checkbox(ui, app.tr("render.library_uniform_ack"), &mut view.acknowledge_projection);
            if ui_kit::button(
                ui,
                app.tr("render.library_apply_uniform"),
                ui_kit::Button::Enabled(current_target(app, view) && view.acknowledge_projection),
            )
            .clicked()
                && let Some(target) = view.target.clone()
            {
                match app.run("render.library.appearance.apply", json!({"id":id,"variant":"uniform_opaque","bodies":[target.name]})) {
                    Ok(_) => {
                        view.target_revision = None;
                        app.set_status(app.tr("render.library_applied").to_string(), false);
                    }
                    Err(e) => app.set_status(e, true),
                }
            }
            if view.kind == "surfaces" {
                ui_kit::text(ui, app.tr("render.library_maps_help"), ui_kit::Text::Note("render.library_maps_help"));
                ui_kit::checkbox(ui, app.tr("render.replace_saved_surface"), &mut view.replace_surface);
                ui.label(app.tr("render.mapping_scale"));
                ui_kit::number_field(ui, &mut view.mapping_scale, 0.001..=1000.0, 0.01, " x");
                ui_kit::segmented(ui, &mut view.mapping_plane, &["XY", "YZ", "ZX"], Default::default());
                let candidate = patch(app);
                let enabled = current_target(app, view) && app.render_workspace.view.method == ImageMethod::GpuRaytrace && candidate.is_ok();
                if ui_kit::button(ui, app.tr("render.library_save_maps"), ui_kit::Button::Enabled(enabled)).clicked()
                    && let (Some(target), Ok(set)) = (view.target.clone(), candidate)
                {
                    let name = app.render_workspace.view.name.clone();
                    match app.run("render.library.surface.bind",json!({"name":name,"set":set,"surface":{"id":id,"body_index":target.ordinal,"body_name":target.name,"scale":view.mapping_scale,"plane":(["xy","yz","zx"].get(view.mapping_plane).copied().unwrap_or("xy"))},"replace":view.replace_surface})) {
                        Ok(_)=>{setups::load(app);view.target_revision=None;app.set_status(app.tr("render.library_applied").to_string(),false);},
                        Err(e)=>app.set_status(e,true),
                    }
                }

                let enabled = current_target(app, view) && app.render_workspace.view.method == ImageMethod::GpuRaytrace;
                if ui_kit::button(ui, app.tr("render.library_apply_maps"), ui_kit::Button::Enabled(enabled)).clicked()
                    && let Some(target) = view.target.clone()
                {
                    match materials::install_library_surface(
                        app,
                        id,
                        target,
                        ["xy", "yz", "zx"].get(view.mapping_plane).copied().unwrap_or("xy"),
                        view.mapping_scale,
                    ) {
                        Ok(()) => app.set_status(app.tr("render.library_applied").to_string(), false),
                        Err(e) => app.set_status(e, true),
                    }
                }
            }
        }
        "hdr" => {
            ui_kit::text(ui, app.tr("render.library_hdr_help"), ui_kit::Text::Note("render.library_hdr_help"));
            ui_kit::checkbox(ui, app.tr("render.library_replace_environment"), &mut view.replace_environment);
            let candidate = patch(app);
            if let Err(e) = &candidate {
                ui.small(e);
            }
            let enabled = candidate.is_ok() && app.render_workspace.view.method != ImageMethod::Raster;
            if ui_kit::button(ui, app.tr("render.library_apply_hdr"), ui_kit::Button::Enabled(enabled)).clicked()
                && let Ok(set) = candidate
            {
                let name = app.render_workspace.view.name.clone();
                match app
                    .run("render.library.environment.apply", json!({"id":id,"name":name,"set":set,"replace_environment":view.replace_environment}))
                {
                    Ok(_) => {
                        setups::load(app);
                        app.set_status(app.tr("render.library_applied").to_string(), false);
                    }
                    Err(e) => app.set_status(e, true),
                }
            }
        }
        "rigs" => {
            let blocked = detail["projection"]["status"] == "blocked";
            if blocked {
                ui.colored_label(Tokens::get().warning, app.tr("render.library_blocked"));
                return;
            }
            ui_kit::text(ui, app.tr("render.library_rig_help"), ui_kit::Text::Note("render.library_rig_help"));
            if ui_kit::button(ui, app.tr("render.library_scene_bounds"), ui_kit::Button::Normal).clicked()
                && let Err(e) = refresh_targets(app, view)
            {
                app.set_status(e, true);
            }
            ui.label(app.tr("render.library_center"));
            for c in &mut view.center {
                ui_kit::number_field(ui, c, -1e12..=1e12, 1.0, " mm");
            }
            ui.label(app.tr("render.library_extent"));
            ui_kit::number_field(ui, &mut view.extent, 0.001..=1e6, 1.0, " mm");
            let cards = detail["projection"]["passive_cards"].as_u64().unwrap_or(0);
            if cards > 0 {
                ui_kit::checkbox(ui, app.tr("render.include_rig_cards"), &mut view.include_cards);
                if !view.include_cards {
                    ui_kit::checkbox(ui, app.tr("render.library_emitters_only_ack"), &mut view.acknowledge_projection);
                }
            }
            ui_kit::checkbox(ui, app.tr("render.library_replace_studio"), &mut view.replace_studio);
            let candidate = patch(app);
            let enabled = !blocked
                && (cards == 0 || view.include_cards || view.acknowledge_projection)
                && view.replace_studio
                && candidate.is_ok()
                && app.render_workspace.view.method == ImageMethod::GpuRaytrace;
            if ui_kit::button(ui, app.tr("render.library_apply_studio"), ui_kit::Button::Enabled(enabled)).clicked()
                && let Ok(set) = candidate
            {
                let name = app.render_workspace.view.name.clone();
                let rig = json!({"id":id,"variant":if cards==0 || view.include_cards {"complete"} else {"emitters_only"},"center_mm":view.center,"subject_extent_mm":view.extent});
                match app.run(
                    "render.library.studio.apply",
                    json!({"rig":rig,"name":name,"set":set,"replace_lighting_and_environment":true,"replace_stage":true}),
                ) {
                    Ok(_) => {
                        setups::load(app);
                        app.set_status(app.tr("render.library_applied").to_string(), false);
                    }
                    Err(e) => app.set_status(e, true),
                }
            }
        }
        _ => {
            ui.colored_label(Tokens::get().warning, app.tr("render.library_specification_only"));
        }
    }
}

pub(super) fn saved_surface_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.saved_surfaces").to_string();
    let bindings = app.render_workspace.view.draft.surfaces.clone();
    if bindings.is_empty() {
        return;
    }
    ui_kit::section(ui, "render_saved_surfaces", &title, false, |ui| {
        ui_kit::text(ui, app.tr("render.saved_surfaces_help"), ui_kit::Text::Note("render.saved_surfaces_help"));
        for (index, binding) in bindings.iter().enumerate() {
            ui.label(format!(
                "{} - {} ({}): {:?}",
                binding.source.id, binding.target.definition, binding.target.feature, binding.target.occurrence_path
            ));
            let response = ui_kit::button(ui, app.tr("render.saved_surface_remove"), ui_kit::Button::Normal);
            crate::scenario::publish_handle(&format!("render:saved-surface-remove:{index}"), response.rect.center());
            if response.clicked() {
                let name = app.render_workspace.view.name.clone();
                // Remove from the saved authority without attempting to save an
                // incompatible draft (for example a deliberate CPU method change).
                match app.run("render.library.surface.unbind", json!({"name":name,"target":binding.target})) {
                    Ok(_) => setups::load(app),
                    Err(e) => app.set_status(e, true),
                }
            }
        }
    });
}

#[cfg(test)]
pub(super) fn stage_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.passive_stage").to_string();
    ui_kit::section(ui, "render_passive_stage", &title, false, |ui| stage_content(app, ui));
}
pub(super) fn stage_content(app: &mut SolveApp, ui: &mut egui::Ui) {
    ui_kit::text(ui, app.tr("render.passive_stage_help"), ui_kit::Text::Note("render.passive_stage_help"));
    let stage = &app.render_workspace.view.draft.stage;
    ui.label(format!("{}: {}", app.tr("render.stage_panel_count"), stage.len()));
    let empty = stage.is_empty();
    let candidate = patch(app);
    if ui_kit::button(
        ui,
        app.tr("render.add_ground"),
        ui_kit::Button::Enabled(empty && candidate.is_ok() && app.render_workspace.view.method != ImageMethod::Raster),
    )
    .clicked()
        && let Ok(set) = candidate
    {
        let name = app.render_workspace.view.name.clone();
        match app.run("render.stage.ground", json!({"name":name,"set":set})) {
            Ok(_) => setups::load(app),
            Err(e) => app.set_status(e, true),
        }
    }
    let clear = ui_kit::button(ui, app.tr("render.clear_stage"), ui_kit::Button::Enabled(!empty && patch(app).is_ok()));
    crate::scenario::publish_target("render:stage-clear", clear.rect);
    if clear.clicked() {
        let name = app.render_workspace.view.name.clone();
        match patch(app).and_then(|set| app.run("render.stage.clear", json!({"name":name,"set":set})).map(|_| ())) {
            Ok(()) => setups::load(app),
            Err(error) => app.set_status(error, true),
        }
    }
}

fn physical_controls(app: &mut SolveApp, ui: &mut egui::Ui, view: &mut View, id: &str, detail: &Value) {
    ui_kit::text(ui, app.tr("render.physical_separate"), ui_kit::Text::Note("render.physical_separate"));
    let resource = &detail["resource"];
    ui.strong(resource["designation"].as_str().unwrap_or(id));
    ui.small(resource["reference_state"].as_str().unwrap_or(""));
    if let Some(properties) = resource["properties"].as_array() {
        for (index, property) in properties.iter().enumerate() {
            let name = property["quantity"].as_str().unwrap_or("property");
            let title = format!("{}: {} {}", name, property["value"], property["unit"].as_str().unwrap_or(""));
            ui_kit::section(ui, ("physical_property", id, index), &title, false, |ui| {
                ui.small(format!("Source: {} - {}", property["source_id"].as_str().unwrap_or(""), property["source_locator"].as_str().unwrap_or("")));
                ui.small(format!("Qualifier: {}", property["qualifier"].as_str().unwrap_or("unknown")));
                ui.small(format!("Conditions: {}", property["conditions"]));
                ui.small(format!("Uncertainty: {}", property["uncertainty"]));
                ui.small(property["note"].as_str().unwrap_or(""));
            });
        }
    }
    if let Some(omissions) = detail["native_projection"]["not_projected"].as_array() {
        let title = app.tr("render.physical_omissions").to_string();
        ui_kit::section(ui, ("physical_omissions", id), &title, false, |ui| {
            for item in omissions {
                ui.small(format!("{}: {}", item["quantity"].as_str().unwrap_or(""), item["reason"].as_str().unwrap_or("")));
            }
        });
    }
    if let Some(sources) = detail["sources"].as_array() {
        for source in sources {
            if let Some(url) = source["url"].as_str() {
                ui.hyperlink_to(source["title"].as_str().unwrap_or(url), url);
            }
        }
    }
    ui.label(app.tr("render.physical_material_name"));
    ui_kit::bounded_cell_editor(ui, ui.id().with("physical_reference_name"), &mut view.physical_name, "", false, 128);
    ui_kit::checkbox(ui, app.tr("render.physical_acknowledge"), &mut view.physical_ack);
    if ui_kit::button(ui, app.tr("render.physical_prepare_plan"), ui_kit::Button::Enabled(view.physical_ack)).clicked() {
        match app.run("render.library.physical.prepare", json!({"id":id,"name":view.physical_name,"acknowledge_reference":true})) {
            Ok(plan) => view.physical_plan = Some(plan),
            Err(e) => app.set_status(e, true),
        }
    }
    if let Some(plan) = &view.physical_plan {
        ui_kit::text(ui, app.tr("render.physical_plan_only"), ui_kit::Text::Note("render.physical_plan_only"));
        let text = serde_json::to_string_pretty(plan).unwrap_or_default();
        if ui_kit::button(ui, app.tr("render.physical_copy_plan"), ui_kit::Button::Normal).clicked() {
            ui.ctx().copy_text(text);
        }
    }
}

fn short_availability<'a>(app: &'a SolveApp, status: Option<&str>) -> &'a str {
    app.tr(match status {
        Some("native_appearance") => "render.library_appearance_available",
        Some("native_composition") => "render.library_short_native",
        Some("implemented_pending_batch_validation") => "render.library_short_projection",
        Some("blocked") => "render.library_short_blocked",
        Some("specification_only") => "render.library_short_specification",
        _ => "render.library_short_reference",
    })
}

fn availability_label<'a>(app: &'a SolveApp, status: Option<&str>) -> &'a str {
    app.tr(match status {
        Some("native_appearance") => "render.library_appearance_available",
        Some("native_composition") => "render.availability_native",
        Some("implemented_pending_batch_validation") => "render.availability_projection",
        Some("blocked") => "render.library_blocked",
        Some("specification_only") => "render.library_specs",
        _ => "render.availability_reference",
    })
}

#[cfg(test)]
mod studio_browser_tests {
    use super::*;
    use crate::scenario::Harness;
    #[test]
    fn appearance_cards_select_inspect_switch_layout_and_keep_explicit_apply() {
        let mut h = Harness::new();
        h.app.run("solid.box", json!({"length":10,"width":10,"height":10,"body_name":"Sample"})).unwrap();
        h.app.run("appearance.assign", json!({"bodies":["Sample"],"appearance":"ABS - Red"})).unwrap();
        super::super::enter(&mut h.app, true).unwrap();
        h.frames(4);
        let before = h.app.session.doc.to_json();
        h.step(&json!({"click":{"handle":"render:library-scope:in-design"}})).unwrap();
        h.frames(3);
        assert_eq!(h.app.render_workspace.view.library.collection, 3);
        let id = appearance_id(&h.app.render_workspace.view.library.entries[0]);
        h.step(&json!({"click":{"handle":format!("render:library:{id}")}})).unwrap();
        h.frames(3);
        assert_eq!(h.app.render_workspace.view.library.selected.as_deref(), Some(id.as_str()));
        assert_eq!(super::super::inspector::active_panel(&h.app), super::super::inspector::PanelId::Surface);
        assert_eq!(h.app.session.doc.to_json(), before, "browsing and selection must not apply appearances");
        h.step(&json!({"widget":h.app.tr("render.library_favorite")})).unwrap();
        assert!(h.app.preferences.render_library_favorites.contains(&("appearances".into(), id.clone())));
        h.step(&json!({"click":{"handle":"render:library-list"}})).unwrap();
        h.frames(3);
        assert_eq!(h.app.render_workspace.view.library.layout, 1);
        h.step(&json!({"click":{"handle":"render:library-grid"}})).unwrap();
        h.frames(3);
        assert_eq!(h.app.render_workspace.view.library.layout, 0);
        assert_eq!(h.app.session.doc.to_json(), before);
        let catalog = h.app.run("appearance.library", json!({})).unwrap();
        let blue = catalog["appearances"].as_array().unwrap().iter().find(|entry| entry["name"] == "ABS - Blue").unwrap();
        let mut view = std::mem::take(&mut h.app.render_workspace.view.library);
        choose(&mut h.app, &mut view, "appearances", &appearance_id(blue)).unwrap();
        refresh_targets(&mut h.app, &mut view).unwrap();
        view.target = view.targets.first().cloned();
        h.app.render_workspace.view.library = view;
        h.frames(3);
        assert_eq!(h.app.session.doc.to_json(), before);
        h.step(&json!({"widget":h.app.tr("render.library_apply_appearance")})).unwrap();
        assert_eq!(h.app.session.doc.appearances.bodies.get("Sample").unwrap().name, "ABS - Blue");
        if let Some(dir) = std::env::var_os("SOLIDCRAFT_LIBRARY_EVIDENCE") {
            std::fs::create_dir_all(&dir).unwrap();
            h.app.run("appearance.assign", json!({"bodies":["Sample"],"appearance":"Aluminum - Satin"})).unwrap();
            h.app.render_workspace.view.library.collection = 0;
            h.app.render_workspace.view.library.loaded = false;
            let aluminum = h.app.run("appearance.library", json!({})).unwrap()["appearances"][0].clone();
            let mut view = std::mem::take(&mut h.app.render_workspace.view.library);
            choose(&mut h.app, &mut view, "appearances", &appearance_id(&aluminum)).unwrap();
            h.app.render_workspace.view.library = view;
            for dark in [false, true] {
                h.app.preferences.theme = if dark { "dark" } else { "light" }.into();
                h.app.ui.dark = dark;
                let ctx = egui::Context::default();
                crate::fonts::setup_bundled_fonts(&ctx);
                let mut textures = std::collections::HashMap::new();
                let mut final_frame = None;
                for frame in 0..10 {
                    let output = ctx.run_ui(
                        egui::RawInput {
                            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 1000.0))),
                            time: Some(frame as f64 / 10.0),
                            focused: true,
                            ..Default::default()
                        },
                        |ui| {
                            h.app.logic(ui.ctx());
                            h.app.ui(ui);
                        },
                    );
                    ui_kit::update_test_textures(&mut textures, &output.textures_delta);
                    final_frame = Some(output);
                }
                let output = final_frame.unwrap();
                let image = ui_kit::raster_ui(&ctx, output.shapes, output.pixels_per_point, ctx.content_rect(), &textures);
                image.save(std::path::Path::new(&dir).join(if dark { "appearances-dark.png" } else { "appearances-light.png" })).unwrap();
            }
        }
    }
    #[test]
    fn tile_finishes_follow_pbr_and_opacity_and_authored_surface_color() {
        for (opacity, metallic, roughness, expected) in [
            (1.0, 0.0, 0.9, ui_kit::FinishKind::Matte),
            (1.0, 0.0, 0.5, ui_kit::FinishKind::Satin),
            (1.0, 0.0, 0.1, ui_kit::FinishKind::Gloss),
            (1.0, 1.0, 0.1, ui_kit::FinishKind::Metal),
            (0.4, 1.0, 0.9, ui_kit::FinishKind::Glass),
        ] {
            let sphere = tile_sphere(&json!({"color":"#123456","opacity":opacity,"pbr":{"metallic":metallic,"roughness":roughness}}));
            assert_eq!(sphere.kind, expected);
            assert_eq!(sphere.color, egui::Color32::from_rgb(0x12, 0x34, 0x56));
            assert_eq!(sphere.opacity, Some(opacity as f32));
        }
        let source = json!({"resource":{"base_linear_low":[0.0,0.5,1.0],"base_linear_high":[0.0,0.5,1.0],"metallic":1.0,"roughness":0.4}});
        assert_eq!(tile_sphere(&source).color, egui::Color32::from_rgb(0, 188, 255));
        let selene = json!({"resource":{"baseline_projection":{"runtime_parameters":{"color":[20,40,60],"opacity":1.0,"pbr":{"metallic":1.0,"roughness":0.3}}}}});
        assert_eq!(tile_sphere(&selene).color, egui::Color32::from_rgb(20, 40, 60));
        assert_eq!(tile_sphere(&selene).kind, ui_kit::FinishKind::Metal);
    }
    #[test]
    fn scopes_search_categories_and_availability_filter_real_catalog_entries() {
        let mut h = Harness::new();
        h.app.run("solid.box", json!({"length":10,"width":10,"height":10,"body_name":"Sample"})).unwrap();
        h.app.run("appearance.assign", json!({"bodies":["Sample"],"appearance":"ABS - Red"})).unwrap();
        let mut view = View { collection: 3, ..Default::default() };
        refresh(&mut h.app, &mut view).unwrap();
        assert_eq!(view.entries.len(), 1);
        assert_eq!(user_count(&view.entries[0]), 1);
        assert_eq!(user_count(&json!({"bodies":["A"],"faces":[{"body":"B"}],"components":["C"]})), 3);
        assert!(matches_entry(&view.entries[0], "rEd", "", "usable"));
        assert!(!matches_entry(&view.entries[0], "blue", "", "all"));
        assert!(matches_entry(&json!({"name":"Coating","resource":{"tags":["Enclosure"]}}), "enclosure", "", "all"));
        let id = appearance_id(&view.entries[0]);
        choose(&mut h.app, &mut view, "appearances", &id).unwrap();
        h.app.preferences.render_library_favorites.insert(("appearances".into(), id));
        for scope in [1, 2] {
            view.collection = scope;
            refresh(&mut h.app, &mut view).unwrap();
            assert_eq!(view.entries.len(), 1);
            assert_eq!(view.entries[0]["name"], "ABS - Red");
        }
        view.collection = 0;
        refresh(&mut h.app, &mut view).unwrap();
        assert!(view.entries.len() > 10);
        assert!(view.entries.iter().any(|entry| matches_entry(entry, "aBs", "ABS", "all")));
        assert!(!view.entries.iter().any(|entry| matches_entry(entry, "ABS", "missing category", "all")));
        assert!(view.entries.iter().filter(|entry| matches_entry(entry, "", "", "blocked")).all(|entry| entry["available"] == false));
        let source = h.app.run("render.library.describe", json!({"kind":"surfaces","id":"SURF-001"})).unwrap();
        let entry = json!({"id":"SURF-001","name":source["resource"]["name"],"resource":source["resource"]});
        h.app.run("render.library.appearance.apply", json!({"id":"SURF-001","variant":"uniform_opaque","bodies":["Sample"]})).unwrap();
        let reply = h.app.run("appearance.list", json!({})).unwrap();
        assert_eq!(user_count(usage_for(&entry, reply["in_design"].as_array().unwrap()).unwrap()), 1);
    }
    #[test]
    fn preferences_round_trip_favorites_and_recent_without_saving_them_in_design() {
        let mut h = Harness::new();
        let mut view = View::default();
        let before = h.app.session.doc.to_json();
        choose(&mut h.app, &mut view, "surfaces", "SURF-001").unwrap();
        h.app.preferences.render_library_favorites.insert(("surfaces".into(), "SURF-001".into()));
        let saved = serde_json::to_value(&h.app.preferences).unwrap();
        let prefs = crate::prefs::lenient(&saved);
        assert_eq!(prefs.render_library_favorites, h.app.preferences.render_library_favorites);
        assert_eq!(prefs.render_library_recent, h.app.preferences.render_library_recent);
        assert_eq!(h.app.session.doc.to_json(), before);
        let custom = json!({"name":"Workshop look","color":"#123456","opacity":0.5});
        let id = appearance_id(&custom);
        let detail = describe(&mut h.app, "appearances", &id).unwrap();
        assert_eq!(detail["resource"]["color"], custom["color"]);
        assert!(crate::prefs::lenient(&json!({})).render_library_favorites.is_empty());
        let hostile = json!({"render_library_recent":vec![("surfaces","SURF-001");100],"render_library_favorites":[["surfaces",""]]});
        let prefs = crate::prefs::lenient(&hostile);
        assert_eq!(prefs.render_library_recent.len(), 32);
        assert!(prefs.render_library_favorites.is_empty());
    }
    #[test]
    fn favorites_and_recent_keep_exact_status_without_promoting_specs_to_runtime() {
        let mut h = Harness::new();
        let before = h.app.session.doc.to_json();
        let mut view = View { kind: "surfaces".into(), ..Default::default() };
        choose(&mut h.app, &mut view, "native_scenes", "NATIVE-STUDIO-002").unwrap();
        let native = ("native_scenes".into(), "NATIVE-STUDIO-002".into());
        h.app.preferences.render_library_favorites.insert(native.clone());
        choose(&mut h.app, &mut view, "scenes", "SCENE-001").unwrap();
        assert_eq!(view.statuses.get(&("scenes".into(), "SCENE-001".into())).map(String::as_str), Some("specification_only"));
        for i in 1..=48 {
            choose(&mut h.app, &mut view, "surfaces", &format!("SURF-{i:03}")).unwrap();
        }
        assert_eq!(h.app.preferences.render_library_recent.len(), 32);
        assert_eq!(view.statuses.len(), 33);
        assert_eq!(view.statuses.get(&native).map(String::as_str), Some("native_composition"));
        assert_eq!(h.app.session.doc.to_json(), before);
        assert!(h.app.session.undo.is_empty());
    }
    #[test]
    fn physical_reference_navigation_uses_the_same_category_history() {
        let mut h = Harness::new();
        let mut view = View { kind: "surfaces".into(), ..Default::default() };
        choose(&mut h.app, &mut view, "surfaces", "SURF-001").unwrap();
        view.query = "brushed".into();
        view.offset = 48;
        let profiles = h.app.run("render.library.list", json!({"kind":"physical_profiles","limit":1})).unwrap();
        let id = profiles["items"][0]["id"].as_str().unwrap();
        choose(&mut h.app, &mut view, "physical_profiles", id).unwrap();
        assert!(!view.loaded);
        switch_kind(&mut view, "surfaces");
        assert_eq!(view.query, "brushed");
        assert_eq!(view.offset, 48);
        assert_eq!(view.selected.as_deref(), Some("SURF-001"));
    }
    #[test]
    fn category_navigation_preserves_queries_pages_selection_and_declared_scope() {
        let mut view = View {
            kind: "surfaces".into(),
            query: "brushed".into(),
            offset: 48,
            selected: Some("SURF-001".into()),
            details: Some(json!({"resource":{"id":"SURF-001"}})),
            availability: "usable".into(),
            ..Default::default()
        };
        switch_kind(&mut view, "hdr");
        assert!(view.query.is_empty());
        assert!(view.selected.is_none());
        view.query = "studio".into();
        switch_kind(&mut view, "surfaces");
        assert_eq!(view.query, "brushed");
        assert_eq!(view.offset, 48);
        assert_eq!(view.selected.as_deref(), Some("SURF-001"));
        assert_eq!(view.availability, "usable");
        switch_kind(&mut view, "invalid");
        assert_eq!(view.kind, "surfaces");
        switch_kind(&mut view, "hdr");
        assert_eq!(view.query, "studio");
    }
    #[test]
    fn choosing_resources_is_read_only_and_recent_history_is_bounded() {
        let mut h = Harness::new();
        let before = h.app.session.doc.to_json();
        let mut view = View::default();
        for n in 1..=48 {
            choose(&mut h.app, &mut view, "surfaces", &format!("SURF-{n:03}")).unwrap();
        }
        assert_eq!(h.app.preferences.render_library_recent.len(), 32);
        assert_eq!(h.app.session.doc.to_json(), before);
        assert!(h.app.session.undo.is_empty());
        choose(&mut h.app, &mut view, "surfaces", "SURF-048").unwrap();
        assert_eq!(h.app.preferences.render_library_recent.len(), 32);
        assert_eq!(super::super::inspector::active_panel(&h.app), super::super::inspector::PanelId::Surface);
        choose(&mut h.app, &mut view, "native_scenes", "NATIVE-STUDIO-002").unwrap();
        assert_eq!(super::super::inspector::active_panel(&h.app), super::super::inspector::PanelId::Studio);
        assert_eq!(h.app.session.doc.to_json(), before);
    }
}
