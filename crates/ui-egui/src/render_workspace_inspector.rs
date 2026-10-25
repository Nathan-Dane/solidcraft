//! Contextual native Render UI. One workspace, existing command authority and shared controls.
//! Panel navigation is per document; unsupported backend capabilities never emit dummy parameters.
use super::{ImageMethod, cpu, library, materials, setups, studio};
use crate::{SolveApp, ui_kit};
use serde_json::{Value, json};
use solidcraft_engine::doc::Document;
use std::sync::{Arc, Weak};

#[path = "render_ui/appearance.rs"]
mod selection_appearance;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum PanelId {
    Appearance,
    Surface,
    Studio,
    Lights,
    Environment,
    Stage,
    Shot,
    Image,
    Effects,
    Check,
}
impl PanelId {
    pub(super) fn panels(tab: &str) -> &'static [Self] {
        match tab {
            "RENDER APPEARANCE" => &[Self::Appearance, Self::Surface],
            "RENDER SCENE" => &[Self::Studio, Self::Lights, Self::Environment, Self::Stage],
            "RENDER CAMERA" => &[Self::Shot],
            _ => &[Self::Image, Self::Effects, Self::Check],
        }
    }
    pub(super) fn tab(self) -> &'static str {
        match self {
            Self::Appearance | Self::Surface => "RENDER APPEARANCE",
            Self::Studio | Self::Lights | Self::Environment | Self::Stage => "RENDER SCENE",
            Self::Shot => "RENDER CAMERA",
            Self::Image | Self::Effects | Self::Check => "RENDER",
        }
    }
    fn tab_index(tab: &str) -> usize {
        match tab {
            "RENDER APPEARANCE" => 0,
            "RENDER SCENE" => 1,
            "RENDER CAMERA" => 2,
            _ => 3,
        }
    }
    pub(super) fn key(self) -> &'static str {
        match self {
            Self::Appearance => "render.panel_appearance",
            Self::Surface => "render.panel_surface",
            Self::Studio => "render.panel_studio",
            Self::Lights => "render.panel_lights",
            Self::Environment => "render.panel_environment",
            Self::Stage => "render.panel_stage",
            Self::Shot => "render.panel_shot",
            Self::Image => "render.panel_image",
            Self::Effects => "render.panel_effects",
            Self::Check => "render.panel_check",
        }
    }
    fn handle(self) -> &'static str {
        match self {
            Self::Appearance => "render:panel:appearance",
            Self::Surface => "render:panel:surface",
            Self::Studio => "render:panel:studio",
            Self::Lights => "render:panel:lights",
            Self::Environment => "render:panel:environment",
            Self::Stage => "render:panel:stage",
            Self::Shot => "render:panel:shot",
            Self::Image => "render:panel:image",
            Self::Effects => "render:panel:effects",
            Self::Check => "render:panel:check",
        }
    }
    fn icon(self) -> &'static str {
        match self {
            Self::Appearance | Self::Surface => "appearance",
            Self::Studio | Self::Stage => "render_scene",
            Self::Lights => "settings",
            Self::Environment => "environment_map",
            Self::Shot | Self::Effects => "camera",
            Self::Image => "export",
            Self::Check => "inspect",
        }
    }
}

pub(super) fn active_panel(app: &SolveApp) -> PanelId {
    app.render_workspace.view.inspector.panels[PanelId::tab_index(&app.ui.tab)]
}

/// Editor navigation changes only the tab and that document's remembered panel.
pub(super) fn open_panel(app: &mut SolveApp, panel: PanelId) {
    app.ui.tab = panel.tab().into();
    app.render_workspace.view.inspector.panels[PanelId::tab_index(panel.tab())] = panel;
}
#[derive(Clone, Debug)]
pub(super) struct View {
    pub effects: super::effects::View,
    panels: [PanelId; 4],
    reveal: Option<&'static str>,
    pending: Option<Pending>,
    shot_name: String,
    scene_review: Option<Review>,
    scene_consent: bool,
    quality_review: Option<Value>,
    quality_ack: bool,
    pub checked: Option<Review>,
    scene_info: Option<Review>,
    ground_margin: f64,
    ground_gap: f64,
    ground_color: [f64; 3],
    replace_ground: bool,
    appearance: Appearance,
    selection_appearance: selection_appearance::View,
    resolution: Resolution,
}
/// Editor-only coupling. The fixed rational anchor prevents repeated rounded
/// edits drifting; external preset/shot changes deliberately establish a new anchor.
#[derive(Clone, Debug, Default)]
struct Resolution {
    locked: bool,
    anchor: Option<[u32; 2]>,
    observed: Option<[u32; 2]>,
    rejected: bool,
}
impl Resolution {
    fn observe(&mut self, size: [u32; 2]) {
        if self.observed != Some(size) {
            self.anchor = (size[0] > 0 && size[1] > 0).then_some(size);
            self.observed = Some(size);
            self.rejected = false;
        }
    }
    fn edit(&mut self, before: [u32; 2], axis: usize, value: u32, maximum: u32) -> Option<[u32; 2]> {
        self.observe(before);
        let mut candidate = before;
        *candidate.get_mut(axis)? = value;
        if value == 0 || value > maximum {
            self.rejected = true;
            return None;
        }
        if self.locked {
            let Some(anchor) = self.anchor else {
                self.rejected = true;
                return None;
            };
            let Some(other) = 1usize.checked_sub(axis) else {
                self.rejected = true;
                return None;
            };
            let denominator = u64::from(*anchor.get(axis)?);
            let numerator = u64::from(value).checked_mul(u64::from(*anchor.get(other)?))?;
            let paired = numerator.checked_add(denominator / 2)?.checked_div(denominator)?;
            if paired == 0 || paired > u64::from(maximum) {
                self.rejected = true;
                return None;
            }
            *candidate.get_mut(other)? = u32::try_from(paired).ok()?;
        } else {
            self.anchor = (candidate[0] > 0 && candidate[1] > 0).then_some(candidate);
        }
        self.observed = Some(candidate);
        self.rejected = false;
        Some(candidate)
    }
}

#[derive(Clone, Debug)]
struct Pending {
    action: PendingAction,
    document: Weak<Document>,
    revision: u64,
}
#[derive(Clone, Debug)]
enum PendingAction {
    Switch(String),
    ManageShot { name: String, command: String },
    Reload,
    Remove(String),
}
#[derive(Clone, Debug)]
pub(super) struct Review {
    request: Value,
    reply: Value,
    revision: u64,
    document: Weak<Document>,
}
impl Review {
    fn current(&self, app: &SolveApp) -> bool {
        self.revision == app.session.revision && Weak::ptr_eq(&self.document, &Arc::downgrade(&app.session.doc))
    }
}
impl View {
    pub(super) fn clear_resource_reviews(&mut self) {
        self.scene_review = None;
        self.scene_consent = false;
        self.quality_review = None;
        self.quality_ack = false;
    }
}
impl Default for View {
    fn default() -> Self {
        Self {
            effects: Default::default(),
            panels: [PanelId::Appearance, PanelId::Studio, PanelId::Shot, PanelId::Image],
            reveal: None,
            pending: None,
            shot_name: String::new(),
            scene_review: None,
            scene_consent: false,
            quality_review: None,
            quality_ack: false,
            checked: None,
            scene_info: None,
            ground_margin: 1.0,
            ground_gap: 0.1,
            ground_color: [0.18; 3],
            replace_ground: false,
            appearance: Appearance::default(),
            selection_appearance: Default::default(),
            resolution: Resolution::default(),
        }
    }
}
pub(super) fn record_check(app: &mut SolveApp, request: &Value, reply: &Value) {
    app.render_workspace.view.inspector.checked =
        Some(Review { request: request.clone(), reply: reply.clone(), revision: app.session.revision, document: Arc::downgrade(&app.session.doc) });
}

pub(super) fn panel(app: &mut SolveApp, ui: &mut egui::Ui) {
    setups::refresh_dirty(app);
    if app.render_workspace.revision != app.session.revision && !app.render_workspace.view.dirty {
        setups::load(app);
    }
    let bounds = crate::shell::panel_bounds(app, ui.available_rect_before_wrap());
    let mut side = app.preferences.render_inspector_side.clone();
    let mut width = app.preferences.render_inspector_width;
    let panel = active_panel(app);
    let title = app.tr(panel.key()).to_string();
    let icon = panel.icon();
    let content = ui_kit::Panel::workspace(
        ui,
        ui_kit::WorkspaceOptions { id: "render:inspector", title: &title, icon, bounds, fill_height: true, header_actions: None },
        &mut side,
        &mut width,
        |ui| {
            ui_kit::scope(ui, |ui| {
                panel_switcher(app, ui);
                let panel = active_panel(app);
                egui::ScrollArea::vertical().id_salt(("render_inspector", panel.handle(), app.docs.active)).auto_shrink([false, false]).show(
                    ui,
                    |ui| {
                        panel_body(app, ui, panel);
                        setups::refresh_dirty(app);
                    },
                );
            });
        },
    );
    app.preferences.render_inspector_width = width;
    if app.preferences.render_inspector_side != side {
        app.preferences.render_browser_side = if side == "left" { "right" } else { "left" }.into();
    }
    app.preferences.render_inspector_side = side.clone();
    let rect = content.map(|c| c.response.rect);
    crate::shell::register_panel(app, "render:inspector", rect, Some(if side == "left" { ui_kit::Edge::Left } else { ui_kit::Edge::Right }));
    if let Some(rect) = rect {
        crate::scenario::publish_rect("render:inspector", rect);
    }
    confirmation(app, ui.ctx());
}
fn panel_switcher(app: &mut SolveApp, ui: &mut egui::Ui) {
    let panels = PanelId::panels(&app.ui.tab);
    if panels.len() <= 1 {
        return;
    }
    let mut index = panels.iter().position(|panel| *panel == active_panel(app)).unwrap_or(0);
    let labels: Vec<_> = panels.iter().map(|panel| app.tr(panel.key()).to_string()).collect();
    let handles: Vec<_> = panels.iter().map(|panel| panel.handle()).collect();
    ui.push_id("render_panel_switcher", |ui| {
        if ui_kit::segmented(
            ui,
            &mut index,
            &labels.iter().map(String::as_str).collect::<Vec<_>>(),
            ui_kit::Segments { compact: true, proportional: true, handles: &handles, handle: Some("render:panels"), ..Default::default() },
        ) && let Some(panel) = panels.get(index)
        {
            open_panel(app, *panel);
        }
    });
}

fn panel_body(app: &mut SolveApp, ui: &mut egui::Ui, panel: PanelId) {
    match panel {
        PanelId::Appearance => appearance_controls(app, ui),
        PanelId::Surface => {
            surface_controls(app, ui);
            super::effects::aging_controls(app, ui);
        }
        PanelId::Studio => scene_controls(app, ui),
        PanelId::Lights => lighting_controls(app, ui),
        PanelId::Environment => environment_controls(app, ui),
        PanelId::Stage => background_controls(app, ui),
        PanelId::Shot => {
            setup_controls(app, ui);
            camera_controls(app, ui);
            shot_controls(app, ui);
        }
        PanelId::Image => output_controls(app, ui),
        PanelId::Effects => super::effects::presentation_controls(app, ui),
        PanelId::Check => {
            output_check(app, ui);
            diagnostics(app, ui);
        }
    }
}

fn setup_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let current = app.render_workspace.view.name.clone();
    let names: Vec<_> = app.session.doc.render.setups.keys().cloned().collect();
    let mut chosen = None;
    let menu = ui_kit::dropdown_menu(ui, "render_setups", &current, |ui| {
        for name in names {
            let r = ui_kit::list_picker(ui, &name, &name, name == current);
            crate::scenario::publish_handle(&format!("render:setup:{name}"), r.rect.center());
            if r.clicked() {
                chosen = Some(name);
                ui.close();
            }
        }
    });
    crate::scenario::publish_target("render:setups", menu.rect);
    if let Some(name) = chosen {
        switch_shot(app, name);
    }
    let field = ui_kit::bounded_cell_editor(ui, ui.id().with("render_setup_name"), &mut app.render_workspace.view.name, "", false, 128);
    crate::scenario::publish_target("render:setup-name", field.rect);
    if field.changed() {
        app.render_workspace.view.dirty = true;
    }
    ui.horizontal_wrapped(|ui| {
        if ui_kit::button(ui, app.tr("render.save_setup"), ui_kit::Button::Enabled(setups::snapshot(app).is_ok())).clicked()
            && let Err(error) = setups::save(app)
        {
            app.set_status(error, true);
        }
        if ui_kit::button(ui, app.tr("render.reload_settings"), ui_kit::Button::Normal).clicked() {
            request_action(app, PendingAction::Reload);
        }
    });
    let status = if app.render_workspace.view.dirty { app.tr("render.unsaved") } else { app.tr("render.studio_saved") };
    ui.add_sized([ui.available_width(), ui.text_style_height(&egui::TextStyle::Body)], egui::Label::new(status).truncate());
}

pub(super) fn switch_shot(app: &mut SolveApp, name: String) {
    setups::refresh_dirty(app);
    request_action(app, PendingAction::Switch(name));
}

pub(super) fn shot_row_action(app: &mut SolveApp, name: String, command: &str) {
    setups::refresh_dirty(app);
    if command == "render.shot.remove" {
        request_action(app, PendingAction::Remove(name));
    } else {
        request_action(app, PendingAction::ManageShot { name, command: command.into() });
    }
}

fn request_action(app: &mut SolveApp, action: PendingAction) {
    if app.render_workspace.view.dirty || matches!(&action, PendingAction::Remove(_)) {
        app.render_workspace.view.inspector.pending =
            Some(Pending { action, document: Arc::downgrade(&app.session.doc), revision: app.session.revision });
    } else if let Err(error) = perform_action(app, action) {
        app.set_status(error, true);
    }
}
fn perform_action(app: &mut SolveApp, action: PendingAction) -> Result<(), String> {
    match action {
        PendingAction::Switch(name) => {
            app.render_workspace.view.name = name;
            setups::load(app);
        }
        PendingAction::ManageShot { name, command } => {
            app.render_workspace.view.name = name;
            setups::load(app);
            // The existing manager owns name validation and command execution.
            app.start(&command);
        }
        PendingAction::Reload => setups::load(app),
        PendingAction::Remove(name) => {
            app.run("render.shot.remove", json!({"name":name,"expected_revision":app.session.revision}))?;
            if name == app.render_workspace.view.name {
                app.render_workspace.view.name = app.session.doc.render.setups.keys().next().cloned().unwrap_or_else(|| "Studio".into());
                setups::load(app);
            }
        }
    }
    Ok(())
}
fn confirmation(app: &mut SolveApp, ctx: &egui::Context) {
    super::shots::confirmation(app, ctx);
    let Some(pending) = app.render_workspace.view.inspector.pending.clone() else { return };
    if pending.revision != app.session.revision || !Weak::ptr_eq(&pending.document, &Arc::downgrade(&app.session.doc)) {
        app.render_workspace.view.inspector.pending = None;
        app.set_status(app.tr("render.review_stale").to_string(), true);
        return;
    }
    let removing = matches!(&pending.action, PendingAction::Remove(_));
    let title = app.tr(if removing { "render.shot_remove_title" } else { "render.draft_confirm_title" }).to_string();
    let keep = app.tr("render.keep_editing").to_string();
    let primary = app.tr(if removing { "render.shot_remove" } else { "render.save_continue" }).to_string();
    let discard = app.tr("render.discard_continue").to_string();
    let message = app.tr(if removing { "render.shot_remove_help" } else { "render.draft_confirm_help" }).to_string();
    let (closed, answer) = ui_kit::Confirm::new(egui::Id::new("render_pending_shot"), if removing { "delete" } else { "save" }, &title)
        .tone(if removing { ui_kit::Tone::Danger } else { ui_kit::Tone::Warning })
        .actions(&primary, &keep, (!removing).then_some(discard.as_str()))
        .selected(ui_kit::Action::Cancel)
        .show(ctx, removing || setups::snapshot(app).is_ok(), |ui| {
            ui.label(&message);
        });
    if closed || answer == ui_kit::Action::Cancel {
        app.render_workspace.view.inspector.pending = None;
        return;
    }
    if answer == ui_kit::Action::None {
        return;
    }
    if answer == ui_kit::Action::Ok
        && !removing
        && let Err(error) = setups::save(app)
    {
        app.set_status(error, true);
        return;
    }
    app.render_workspace.view.inspector.pending = None;
    if let Err(error) = perform_action(app, pending.action) {
        app.set_status(error, true);
    }
}
fn browse_button(app: &mut SolveApp, ui: &mut egui::Ui, kind: &str, key: &str) {
    if ui_kit::button(ui, app.tr(key), ui_kit::Button::Normal).clicked() {
        library::open_category(app, kind);
    }
}
fn scene_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    // After a library selection, primary native-resource actions lead the panel.
    // Shared method/help and catalog navigation remain below, not between the
    // chosen resource and its explicit review/consent. No state is changed by layout.
    let selected_native = library::selected_kind(app, "native_scenes");
    if selected_native {
        library::inspect_kinds(app, ui, &["native_scenes"]);
    }
    ui_kit::text(ui, app.tr("render.scene_preserves_design"), ui_kit::Text::Note("render.scene_preserves_design"));
    browse_button(app, ui, "native_scenes", "render.browse_native_scenes");
    browse_button(app, ui, "scenes", "render.browse_scene_specs");
    if !selected_native {
        library::inspect_kinds(app, ui, &["native_scenes", "scenes", "selene_shots"]);
    }
}
fn physical_stage_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.passive_stage").to_string();
    reveal_section(app, ui, "render_ground_details");
    ui_kit::section(ui, "render_ground_details", &title, false, |ui| {
        ui.label(app.tr("render.ground_margin"));
        ui_kit::number_field(ui, &mut app.render_workspace.view.inspector.ground_margin, 0.0..=10.0, 0.05, " x");
        ui.label(app.tr("render.ground_gap"));
        ui_kit::number_field(ui, &mut app.render_workspace.view.inspector.ground_gap, 0.0001..=1000.0, 0.01, " mm");
        let label = app.tr("render.ground_linear_rgb").to_string();
        vector(ui, &label, &mut app.render_workspace.view.inspector.ground_color, 0.0..=1.0, 0.01);
        let label = app.tr("render.replace_ground").to_string();
        let replace = ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.inspector.replace_ground);
        crate::scenario::publish_target("render:stage-replace-consent", replace.rect);
        let saved = setups::snapshot(app);
        if ui_kit::button(
            ui,
            app.tr("render.ground_apply"),
            ui_kit::Button::Enabled(saved.is_ok() && app.render_workspace.view.method != ImageMethod::Raster),
        )
        .clicked()
            && let Ok(set) = saved
        {
            let v = &app.render_workspace.view.inspector;
            let request = json!({"name":app.render_workspace.view.name,"set":set,"margin_fraction":v.ground_margin,"gap_mm":v.ground_gap,"reflectance_linear":v.ground_color,"replace_stage":v.replace_ground});
            match app.run("render.stage.ground", request) {
                Ok(_) => setups::load(app),
                Err(e) => app.set_status(e, true),
            }
        }
        ui_kit::text(ui, app.tr("render.physical_background_help"), ui_kit::Text::Note("render.physical_background_help"));
        for (kind, key) in [("floor_wall", "render.physical_floor_wall"), ("cyclorama", "render.physical_cyclorama")] {
            let set = setups::snapshot(app);
            let button =
                ui_kit::button(ui, app.tr(key), ui_kit::Button::Enabled(set.is_ok() && app.render_workspace.view.method != ImageMethod::Raster));
            crate::scenario::publish_target(&format!("render:stage-{kind}"), button.rect);
            if button.clicked()
                && let Ok(set) = set
            {
                let v = &app.render_workspace.view.inspector;
                let p = json!({"name":app.render_workspace.view.name,"set":set,"kind":kind,"expected_revision":app.session.revision,
                    "replace_stage":v.replace_ground,"reflectance_linear":v.ground_color});
                match app.run("render.stage.backdrop", p) {
                    Ok(_) => setups::load(app),
                    Err(error) => app.set_status(error, true),
                }
            }
        }
        library::stage_content(app, ui);
    });
}

fn background_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    ui_kit::text(ui, app.tr("render.background_workflow"), ui_kit::Text::Note("render.background_workflow"));
    let title = app.tr("render.background_camera_section").to_string();
    ui_kit::section(ui, "render_camera_background_section", &title, true, |ui| {
        ui.label(app.tr("render.background"));
        let color = ui_kit::color_field(ui, &mut app.render_workspace.view.draft.background);
        crate::scenario::publish_target("render:background-color", color.rect);
        app.render_workspace.view.dirty |= color.changed();
        if app.render_workspace.view.method == ImageMethod::GpuRaytrace {
            studio::backdrop_controls(app, ui);
        }
        if !app.render_workspace.view.draft.stage.is_empty() {
            ui_kit::text(ui, app.tr("render.background_stage_occlusion"), ui_kit::Text::Note("render.background_stage_occlusion"));
        }
        if app.render_workspace.view.result {
            ui_kit::text(ui, app.tr("render.background_completed_frozen"), ui_kit::Text::Note("render.background_completed_frozen"));
        }
        let enabled = super::ribbon::action_ready(app, "render.result.regrade").is_ok();
        let apply = ui_kit::button(ui, app.tr("render.background_regrade"), ui_kit::Button::Enabled(enabled));
        crate::scenario::publish_target("render:background-regrade", apply.rect);
        if apply.clicked() {
            if let Err(error) = super::ribbon::perform_job(app, "render.result.regrade") {
                app.set_status(error, true);
            }
            ui.ctx().request_repaint();
        }
        ui_kit::text(ui, app.tr("render.background_no_alpha_override"), ui_kit::Text::Note("render.background_no_alpha_override"));
    });
    physical_stage_controls(app, ui);
    let title = app.tr("render.background_lighting_section").to_string();
    ui_kit::section(ui, "render_background_hdr_section", &title, false, |ui| {
        ui_kit::text(ui, app.tr("render.background_hdr_separate"), ui_kit::Text::Note("render.background_hdr_separate"));
        if ui_kit::button(ui, app.tr("render.browse_environments"), ui_kit::Button::Normal).clicked() {
            open_panel(app, PanelId::Environment);
            library::open_category(app, "hdr");
        }
    });
}

fn shot_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.shot_manager").to_string();
    reveal_section(app, ui, "render_shot_manager");
    ui_kit::section(ui, "render_shot_manager", &title, false, |ui| {
        ui_kit::text(ui, app.tr("render.shot_saved_only"), ui_kit::Text::Note("render.shot_saved_only"));
        let label = app.tr("render.shot_new_name").to_string();
        ui_kit::bounded_cell_editor(ui, ui.id().with("render_shot_new_name"), &mut app.render_workspace.view.inspector.shot_name, &label, false, 128);
        for (command, key) in [("render.shot.duplicate", "render.shot_duplicate"), ("render.shot.rename", "render.shot_rename")] {
            let enabled = !app.render_workspace.view.dirty
                && app.session.doc.render.setups.contains_key(&app.render_workspace.view.name)
                && !app.render_workspace.view.inspector.shot_name.trim().is_empty();
            if ui_kit::button(ui, app.tr(key), ui_kit::Button::Enabled(enabled)).clicked() {
                let name = app.render_workspace.view.inspector.shot_name.clone();
                match app.run(command, json!({"name":app.render_workspace.view.name,"new_name":name,"expected_revision":app.session.revision})) {
                    Ok(_) => {
                        app.render_workspace.view.name = name;
                        setups::load(app);
                    }
                    Err(e) => app.set_status(e, true),
                }
            }
        }
        if ui_kit::button(
            ui,
            app.tr("render.shot_remove"),
            ui_kit::Button::Enabled(app.session.doc.render.setups.contains_key(&app.render_workspace.view.name)),
        )
        .clicked()
        {
            request_action(app, PendingAction::Remove(app.render_workspace.view.name.clone()));
        }
    });
}
fn appearance_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    if selection_appearance::controls(app, ui) {
        return;
    }
    base_appearance(app, ui);
}
fn surface_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    ui_kit::text(ui, app.tr("render.appearance_physics_separate"), ui_kit::Text::Note("render.appearance_physics_separate"));
    browse_button(app, ui, "surfaces", "render.browse_surfaces");
    browse_button(app, ui, "physical_profiles", "render.browse_physical");
    library::inspect_kinds(app, ui, &["surfaces", "materials", "selene_appearances", "physical_profiles"]);
    library::saved_surface_controls(app, ui);
    materials::controls(app, ui);
    if app.render_workspace.view.method == ImageMethod::CpuRaytrace {
        cpu::reflection_controls(app, ui);
    }
}
fn lighting_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    browse_button(app, ui, "rigs", "render.browse_rigs");
    library::inspect_kinds(app, ui, &["rigs", "lighting"]);
    match app.render_workspace.view.method {
        ImageMethod::GpuRaytrace => studio::lighting_controls(app, ui),
        ImageMethod::CpuRaytrace => {
            super::cpu_lighting_controls(app, ui);
            cpu::light_controls(app, ui);
        }
        ImageMethod::Raster => {
            ui_kit::text(ui, app.tr("render.method_limit_raster"), ui_kit::Text::Note("render.method_limit_raster"));
        }
    }
}
fn environment_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    browse_button(app, ui, "hdr", "render.browse_environments");
    library::inspect_kinds(app, ui, &["hdr", "environments"]);
    match app.render_workspace.view.method {
        ImageMethod::GpuRaytrace => studio::environment_controls(app, ui),
        ImageMethod::CpuRaytrace => cpu::environment_controls(app, ui),
        ImageMethod::Raster => {
            ui_kit::text(ui, app.tr("render.method_limit_raster"), ui_kit::Text::Note("render.method_limit_raster"));
        }
    }
}
fn vector(ui: &mut egui::Ui, label: &str, value: &mut [f64; 3], range: std::ops::RangeInclusive<f64>, speed: f64) {
    ui.label(label);
    ui.horizontal(|ui| {
        for component in value {
            ui_kit::number_field(ui, component, range.clone(), speed, "");
        }
    });
}
fn apply_camera_draft(app: &mut SolveApp, camera: solidcraft_engine::render::Camera) -> Result<(), String> {
    app.run("render.camera.set", json!({"camera":camera}))?;
    app.follow_view_request(false);
    app.render_workspace.view.dirty = true;
    Ok(())
}

fn camera_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    if super::shots::camera_modified(app, &app.render_workspace.view.name) {
        ui_kit::banner(ui, ui_kit::AssistantTone::Warning, |ui| {
            ui_kit::text(ui, app.tr("render.shots_camera_changed"), ui_kit::Text::Note("render.shots_camera_changed"));
            ui.horizontal_wrapped(|ui| {
                let update = ui_kit::primary_button(ui, app.tr("render.shots_update"));
                crate::scenario::publish_target("render:shots:update", update.rect);
                if update.clicked()
                    && let Err(error) = super::shots::update_camera(app)
                {
                    app.set_status(error, true);
                }
                let revert = ui_kit::button(ui, app.tr("render.shots_revert"), ui_kit::Button::Normal);
                crate::scenario::publish_target("render:shots:revert", revert.rect);
                if revert.clicked()
                    && let Err(error) = restore_camera(app)
                {
                    app.set_status(error, true);
                }
            });
        });
    }
    ui_kit::text(ui, app.tr("render.camera_exact_help"), ui_kit::Text::Note("render.camera_exact_help"));
    let before = app.cam;
    let mut camera = before;
    let mut perspective = camera.fov > 0.0;
    if ui_kit::checkbox(ui, app.tr("render.perspective"), &mut perspective).changed() {
        camera.fov = if perspective { std::f64::consts::FRAC_PI_4 } else { 0.0 };
    }
    if perspective {
        let mut fov = camera.fov.to_degrees();
        let label = app.tr("render.camera_fov").to_string();
        if ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut fov, 1.0..=150.0, 0.1, " deg")).changed() {
            camera.fov = fov.to_radians();
        }
    }
    let label = app.tr("render.camera_distance").to_string();
    ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut camera.distance, 0.000001..=1e9, 1.0, " mm"));
    let mut target = [camera.target.x, camera.target.y, camera.target.z];
    vector(ui, app.tr("render.camera_target"), &mut target, -1e9..=1e9, 1.0);
    camera.target = solidcraft_engine::geom::Vec3::new(target[0], target[1], target[2]);
    for (key, value) in [("render.camera_yaw", &mut camera.yaw), ("render.camera_pitch", &mut camera.pitch), ("render.camera_roll", &mut camera.roll)]
    {
        let mut degrees = value.to_degrees();
        let label = app.tr(key).to_string();
        if ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut degrees, -360000.0..=360000.0, 0.5, " deg")).changed() {
            *value = degrees.to_radians();
        }
    }
    if camera != before
        && camera.is_valid()
        && let Err(error) = apply_camera_draft(app, camera)
    {
        app.set_status(error, true);
    }
    camera_actions(app, ui);
    if app.render_workspace.view.method == ImageMethod::GpuRaytrace {
        studio::lens_controls(app, ui);
    }
    if app.render_workspace.view.result {
        if ui_kit::button(ui, app.tr("render.result_fit"), ui_kit::Button::Normal).clicked() {
            super::fit_result(app);
        }
        if ui_kit::button(ui, "100%", ui_kit::Button::Enabled(!super::reduced_display(app))).clicked() {
            app.render_workspace.view.result_scale = super::ResultScale::Actual;
        }
    }
}
fn output_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    studio::export_controls(app, ui);
    super::method_selector(app, ui);
    let side = match app.render_workspace.view.method {
        ImageMethod::CpuRaytrace => 16384,
        ImageMethod::GpuRaytrace => 8192,
        ImageMethod::Raster => 4096,
    };
    let before = [app.render_workspace.view.draft.width, app.render_workspace.view.draft.height];
    app.render_workspace.view.inspector.resolution.observe(before);
    let mut width = before[0];
    let mut height = before[1];
    let label = app.tr("render.image_width").to_string();
    let w = ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut width, 1..=side, 1.0, " px"));
    crate::scenario::publish_target("render:output-width", w.rect);
    let label = app.tr("render.image_height").to_string();
    let h = ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut height, 1..=side, 1.0, " px"));
    crate::scenario::publish_target("render:output-height", h.rect);
    let edit = if w.changed() {
        Some((0, width))
    } else if h.changed() {
        Some((1, height))
    } else {
        None
    };
    if let Some((axis, value)) = edit
        && let Some([width, height]) = app.render_workspace.view.inspector.resolution.edit(before, axis, value, side)
    {
        app.render_workspace.view.draft.width = width;
        app.render_workspace.view.draft.height = height;
    }
    let label = app.tr("render.output_lock_ratio").to_string();
    let lock = ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.inspector.resolution.locked);
    crate::scenario::publish_target("render:aspect-lock", lock.rect);
    let swap = ui_kit::button(ui, app.tr("render.output_swap_orientation"), ui_kit::Button::Normal);
    crate::scenario::publish_target("render:swap-size", swap.rect);
    if swap.clicked() {
        let draft = &mut app.render_workspace.view.draft;
        std::mem::swap(&mut draft.width, &mut draft.height);
        app.render_workspace.view.inspector.resolution.observe([draft.width, draft.height]);
    }
    if app.render_workspace.view.inspector.resolution.rejected {
        ui_kit::text(ui, app.tr("render.output_ratio_refused"), ui_kit::Text::Error("render.output_ratio_refused"));
    }
    let largest = largest_output(app);
    let (w, h) = largest.map_or_else(|| ("—".to_string(), "—".to_string()), |(w, h)| (w.to_string(), h.to_string()));
    let largest_label = crate::i18n::format("render.output_largest", &[("width", &w), ("height", &h)]);
    ui.horizontal_wrapped(|ui| {
        for (key, label, size) in [
            ("720", "1280 × 720".to_string(), Some((1280, 720))),
            ("1080", "1920 × 1080".to_string(), Some((1920, 1080))),
            ("largest", largest_label, largest),
        ] {
            let allowed = size.filter(|&(w, h)| cpu::image_limits(app).fit(w, h, None) == Some((w, h)));
            let response = ui
                .add_enabled_ui(allowed.is_some(), |ui| {
                    ui_kit::choice_chip(ui, key, &label, size == Some((u64::from(before[0]), u64::from(before[1]))))
                })
                .inner;
            crate::scenario::publish_target(&format!("render:size:{key}"), response.rect);
            let help = allowed.map_or_else(|| app.tr("render.output_over_budget").to_string(), |(w, h)| format!("{w} × {h} px"));
            if response.on_hover_text(help).clicked()
                && let Some((w, h)) = allowed
            {
                let _ = app.run("render.output.size", json!({"width":w,"height":h}));
            }
        }
    });
    let size = (u64::from(app.render_workspace.view.draft.width), u64::from(app.render_workspace.view.draft.height));
    if cpu::image_limits(app).fit(size.0, size.1, None) != Some(size) {
        ui_kit::text(ui, app.tr("render.output_over_budget"), ui_kit::Text::Error("render.output_over_budget"));
    }
    let title = app.tr("render.output_quality").to_string();
    let reason = app.tr("render.output_quality_waiting").to_string();
    let labels: Vec<_> = ["render.quality_draft", "render.quality_balanced", "render.quality_high", "render.quality_custom"]
        .iter()
        .map(|k| app.tr(k).to_string())
        .collect();
    ui_kit::text(ui, &title, ui_kit::Text::Heading("render.output_quality"));
    let mut quality = 3;
    ui_kit::segmented(
        ui,
        &mut quality,
        &labels.iter().map(String::as_str).collect::<Vec<_>>(),
        ui_kit::Segments { compact: true, disabled_reasons: &[Some(&reason), Some(&reason), Some(&reason), None], ..Default::default() },
    );
    ui_kit::text(ui, &reason, ui_kit::Text::Note("render.output_quality_waiting"));
    match app.render_workspace.view.method {
        ImageMethod::GpuRaytrace => {
            let label = app.tr("render.image_samples").to_string();
            ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut app.render_workspace.view.samples, 1..=4096, 1.0, ""));
            let label = app.tr("render.gpu_depth").to_string();
            ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut app.render_workspace.view.max_depth, 1..=16, 1.0, ""));
            let label = app.tr("render.gpu_seed").to_string();
            ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut app.render_workspace.view.seed, 0..=u32::MAX, 1.0, ""));
            studio::finishing_controls(app, ui);
        }
        ImageMethod::CpuRaytrace => super::cpu_sampling_controls(app, ui),
        ImageMethod::Raster => {
            ui_kit::text(ui, app.tr("render.method_limit_raster"), ui_kit::Text::Note("render.method_limit_raster"));
        }
    }
    if app.render_workspace.view.method != ImageMethod::Raster {
        let label = app.tr("render.image_exposure").to_string();
        ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, &mut app.render_workspace.view.draft.exposure_ev, -32.0..=32.0, 0.1, " EV"));
    }
    let label = app.tr("render.image_transparent").to_string();
    ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.transparent);
    ui_kit::text(ui, app.tr("render.output_format_status"), ui_kit::Text::Note("render.output_format_status"));
    let label = app.tr("render.image_overwrite").to_string();
    ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.overwrite);
    browse_button(app, ui, "quality", "render.browse_quality");
    library::inspect_kinds(app, ui, &["quality"]);
    super::completed_image(app, ui);
    ui_kit::text(ui, app.tr("render.adaptive_pending"), ui_kit::Text::Note("render.adaptive_pending"));
}

/// Fit an oversized same-aspect candidate, since fit deliberately never upscales.
pub(super) fn largest_output(app: &SolveApp) -> Option<(u64, u64)> {
    let limits = cpu::image_limits(app);
    let draft = &app.render_workspace.view.draft;
    limits.fit(u64::from(draft.width).checked_mul(limits.max_dimension)?, u64::from(draft.height).checked_mul(limits.max_dimension)?, None)
}
fn output_check(app: &SolveApp, ui: &mut egui::Ui) {
    ui_kit::text(ui, app.tr("render.output_check"), ui_kit::Text::Heading("render.output_check"));
    let Some(check) = &app.render_workspace.view.inspector.checked else {
        ui_kit::text(ui, app.tr("render.output_check_help"), ui_kit::Text::Note("render.output_check_help"));
        return;
    };
    if !check.current(app) || super::image_params(app).as_ref().ok() != Some(&check.request) {
        ui_kit::check_line(ui, "render:check:stale", app.tr("render.review_stale"), true);
        return;
    }
    ui_kit::check_line(ui, "render:check:ready", app.tr("render.output_ready"), false);
    if let (Some(w), Some(h)) = (check.reply["request"]["width"].as_u64(), check.reply["request"]["height"].as_u64()) {
        let label = crate::i18n::format("render.output_size_ok", &[("width", &w.to_string()), ("height", &h.to_string())]);
        ui_kit::check_line(ui, "render:check:size", &label, false);
    }
    if let Some(transparency) = check.reply["transparency"].as_str() {
        let key = match transparency {
            "physical_transmission" => "render.check_transmission",
            "opaque_only" => "render.check_opaque",
            _ => "render.check_sorted_alpha",
        };
        ui_kit::check_line(ui, "render:check:transparency", app.tr(key), transparency != "physical_transmission");
    }
    if let Some(warnings) = check.reply["warnings"].as_array() {
        for warning in warnings.iter().take(32) {
            ui_kit::check_line(ui, "render:check:warning", &warning.to_string(), true);
        }
    }
}

pub(super) fn scene_resource(app: &mut SolveApp, ui: &mut egui::Ui, id: &str) {
    let snapshot = setups::snapshot(app);
    let resolve = ui_kit::button(
        ui,
        app.tr("render.scene_review"),
        ui_kit::Button::Enabled(snapshot.is_ok() && app.render_workspace.view.method == ImageMethod::GpuRaytrace),
    );
    crate::scenario::publish_target("render:scene-resolve", resolve.rect);
    if resolve.clicked()
        && let Ok(set) = snapshot
    {
        let request = json!({"id":id,"name":app.render_workspace.view.name,"set":set,"expected_revision":app.session.revision});
        match app.run("render.preset.resolve", request.clone()) {
            Ok(reply) => {
                app.render_workspace.view.inspector.scene_review =
                    Some(Review { request, reply, revision: app.session.revision, document: Arc::downgrade(&app.session.doc) });
                app.render_workspace.view.inspector.scene_consent = false;
            }
            Err(error) => app.set_status(error, true),
        }
    }
    let review = app.render_workspace.view.inspector.scene_review.clone().filter(|r| r.request["id"] == id);
    if let Some(review) = review {
        let unchanged =
            review.current(app) && setups::snapshot(app).ok().and_then(|v| serde_json::to_value(v).ok()).as_ref() == Some(&review.request["set"]);
        if !unchanged {
            ui_kit::text(ui, app.tr("render.review_stale"), ui_kit::Text::Error("render.review_stale"));
        }
        if let Some(fields) = review.reply["replaces"].as_array() {
            ui.small(format!("{}: {}", app.tr("render.scene_replaces"), fields.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")));
        }
        let label = app.tr("render.scene_consent").to_string();
        let consent = ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.inspector.scene_consent).on_hover_text(&label);
        crate::scenario::publish_target("render:scene-consent", consent.rect);
        let apply =
            ui_kit::button(ui, app.tr("render.scene_apply"), ui_kit::Button::Enabled(unchanged && app.render_workspace.view.inspector.scene_consent));
        crate::scenario::publish_target("render:scene-apply", apply.rect);
        if apply.clicked() {
            let mut request = review.request;
            request["replace_lighting_and_environment"] = json!(true);
            request["replace_stage"] = json!(true);
            match app.run("render.preset.apply", request) {
                Ok(_) => {
                    setups::load(app);
                    app.render_workspace.view.inspector.scene_review = None;
                    app.set_status(app.tr("render.library_applied").to_string(), false);
                }
                Err(error) => app.set_status(error, true),
            }
        }
    }
    let title = app.tr("render.scene_scope_details").to_string();
    ui_kit::section(ui, ("render_native_composition_scope", id), &title, false, |ui| {
        for key in ["render.scene_review_help", "render.scene_preserved_fields", "render.library_authored_help"] {
            ui_kit::text(ui, app.tr(key), ui_kit::Text::Note(key));
        }
    });
}
pub(super) fn quality_resource(app: &mut SolveApp, ui: &mut egui::Ui, id: &str) {
    ui_kit::text(ui, app.tr("render.quality_projection_help"), ui_kit::Text::Note("render.quality_projection_help"));
    let label = app.tr("render.quality_ack").to_string();
    ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.inspector.quality_ack);
    let enabled = app.render_workspace.view.inspector.quality_ack && app.render_workspace.view.method == ImageMethod::GpuRaytrace;
    if ui_kit::button(ui, app.tr("render.quality_review"), ui_kit::Button::Enabled(enabled)).clicked() {
        match app.run("render.quality.prepare", json!({"id":id,"variant":"fixed_samples_png","acknowledge_omissions":true})) {
            Ok(reply) => app.render_workspace.view.inspector.quality_review = Some(reply),
            Err(error) => {
                app.render_workspace.view.inspector.quality_review = None;
                app.set_status(error, true);
            }
        }
    }
    if let Some(plan) = app.render_workspace.view.inspector.quality_review.clone().filter(|p| p["id"] == id) {
        ui.label(format!(
            "{} x {} px / {} spp / {}",
            plan["settings"]["width"], plan["settings"]["height"], plan["gpu_options"]["samples"], plan["gpu_options"]["max_depth"]
        ));
        if ui_kit::button(ui, app.tr("render.quality_apply"), ui_kit::Button::Enabled(enabled)).clicked() {
            let result = apply_quality(app, &plan);
            if let Err(error) = result {
                app.set_status(error, true);
            }
        }
    }
}
fn apply_quality(app: &mut SolveApp, plan: &Value) -> Result<(), String> {
    if app.render_workspace.view.method != ImageMethod::GpuRaytrace || plan["variant"] != "fixed_samples_png" {
        return Err("quality projection requires the same explicit GPU method".into());
    }
    let u32_at = |section: &str, key: &str| {
        plan[section][key].as_u64().and_then(|n| u32::try_from(n).ok()).ok_or_else(|| "invalid native quality projection".to_string())
    };
    let (width, height, samples, depth, seed) = (
        u32_at("settings", "width")?,
        u32_at("settings", "height")?,
        u32_at("gpu_options", "samples")?,
        u32_at("gpu_options", "max_depth")?,
        u32_at("gpu_options", "seed")?,
    );
    let denoise = plan["gpu_options"]["denoise"].as_bool().ok_or("invalid native denoise setting")?;
    if !(1..=4096).contains(&samples)
        || !(1..=16).contains(&depth)
        || solidcraft_engine::render::job::GPU_PATH_LIMITS.fit(u64::from(width), u64::from(height), None)
            != Some((u64::from(width), u64::from(height)))
    {
        return Err("quality projection exceeds native bounds".into());
    }
    let view = &mut app.render_workspace.view;
    view.draft.width = width;
    view.draft.height = height;
    view.samples = samples;
    view.max_depth = depth;
    view.seed = seed;
    view.studio.denoise = denoise;
    view.dirty = true;
    Ok(())
}
fn diagnostics(app: &mut SolveApp, ui: &mut egui::Ui) {
    gpu_status(app, ui);
    if let Some(bytes) = app.render_workspace.preview_upload_bytes {
        ui.label(format!("{}: {:.2} / {} MiB", app.tr("render.preview_upload"), bytes as f64 / 1048576.0, super::MAX_GEOMETRY_BYTES / 1048576));
        ui_kit::text(ui, app.tr("render.preview_upload_help"), ui_kit::Text::Note("render.preview_upload_help"));
    }
    if let Some(error) = &app.render_workspace.scene_error {
        ui_kit::text(ui, error, ui_kit::Text::Error("render.preview_cached_error"));
    }
    let request = super::image_request(app);
    cpu::checked_status(app, ui, super::image_params(app).as_ref().ok());
    if let Ok(backends) = app.session.execute("render.backends", &json!({})) {
        json_details(app, ui, "cmd.render_backends", &backends);
    }
    ui.strong(app.tr(app.render_workspace.view.method.label()));
    if let Err(error) = super::image_params(app) {
        ui_kit::text(ui, &error.to_string(), ui_kit::Text::Error("render.native-error"));
    }
    if ui_kit::button(ui, app.tr("render.inspect_scene"), ui_kit::Button::Normal).clicked() {
        match app.run("render.scene", json!({})) {
            Ok(reply) => {
                app.render_workspace.view.inspector.scene_info =
                    Some(Review { request: json!({}), reply, revision: app.session.revision, document: Arc::downgrade(&app.session.doc) })
            }
            Err(error) => app.set_status(error, true),
        }
    }
    if let Some(info) = &app.render_workspace.view.inspector.scene_info {
        if !info.current(app) {
            ui_kit::text(ui, app.tr("render.review_stale"), ui_kit::Text::Note("render.review_stale"));
        }
        let bodies = info.reply["bodies"].as_array();
        let count = bodies.map_or(0, Vec::len);
        let triangles = bodies.map_or(0, |b| b.iter().filter_map(|v| v["triangles"].as_u64()).sum::<u64>());
        ui.label(format!("{}: {count} / {}: {triangles}", app.tr("render.diag_bodies"), app.tr("render.diag_triangles")));
    }
    if let Some(check) = &app.render_workspace.view.inspector.checked {
        ui.strong(app.tr("render.diag_last_check"));
        if !check.current(app) || request.as_ref().ok() != Some(&check.request) {
            ui_kit::text(ui, app.tr("render.review_stale"), ui_kit::Text::Note("render.review_stale"));
        }
        if let Some(bytes) = check.reply["admission"]["estimated_total_bytes"].as_u64() {
            ui.label(format!("{}: {:.2} MiB", app.tr("render.diag_memory"), bytes as f64 / 1048576.0));
        }
        if let Some(warnings) = check.reply["warnings"].as_array() {
            for warning in warnings {
                ui_kit::text(ui, &warning.to_string(), ui_kit::Text::Note("render.native-warning"));
            }
        }
        let detail = check.reply.clone();
        json_details(app, ui, "render.diag_preflight_details", &detail);
    }
    if let Some(job) = app.render_workspace.view.job
        && let Ok(status) = app.session.execute("render.status", &json!({"job":job}))
    {
        if let Some(error) = status["error"]["message"].as_str() {
            ui_kit::text(ui, error, ui_kit::Text::Error("render.native-error"));
        }
        cpu::captured_status(app, ui, &status);
        json_details(app, ui, "render.diag_job_details", &status);
    }
    if let Ok(request) = &request {
        json_details(app, ui, "render.diag_request", request);
    }
    ui.separator();
    ui.strong(app.tr("render.diag_not_implemented"));
    for key in
        ["render.adaptive_pending", "render.exr_pending", "render.transmission_scope", "render.advanced_material_scope", "render.queue_pending"]
    {
        ui_kit::text(ui, app.tr(key), ui_kit::Text::Note(key));
    }
    browse_button(app, ui, "capabilities", "render.browse_capabilities");
    library::inspect_kinds(app, ui, &["capabilities"]);
}
fn json_details(app: &SolveApp, ui: &mut egui::Ui, key: &str, value: &Value) {
    let title = app.tr(key).to_string();
    ui_kit::section(ui, key, &title, false, |ui| {
        let text = serde_json::to_string_pretty(value).unwrap_or_default();
        if ui_kit::button(ui, app.tr("render.copy_request"), ui_kit::Button::Normal).clicked() {
            ui.ctx().copy_text(text.clone());
        }
        egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
            ui.monospace(text);
        });
    });
}

#[derive(Clone, Debug)]
struct Appearance {
    query: String,
    items: Vec<Value>,
    selected: Option<String>,
    document: Option<Weak<Document>>,
    revision: Option<u64>,
    color: [u8; 3],
    metallic: f64,
    roughness: f64,
    ior: f64,
    opaque: bool,
}
impl Default for Appearance {
    fn default() -> Self {
        Self {
            query: String::new(),
            items: Vec::new(),
            selected: None,
            document: None,
            revision: None,
            color: [160, 170, 180],
            metallic: 0.0,
            roughness: 0.5,
            ior: 1.5,
            opaque: false,
        }
    }
}
fn base_appearance(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.base_appearance").to_string();
    ui_kit::section(ui, "render_base_appearance", &title, false, |ui| {
        let mut view = std::mem::take(&mut app.render_workspace.view.inspector.appearance);
        ui_kit::bounded_cell_editor(ui, ui.id().with("render_base_target_search"), &mut view.query, app.tr("render.material_filter"), false, 128);
        if ui_kit::button(ui, app.tr("render.material_refresh"), ui_kit::Button::Normal).clicked() {
            match app.run("render.library.targets", json!({"query":view.query,"limit":128})) {
                Ok(reply) => {
                    view.items = reply["items"].as_array().cloned().unwrap_or_default();
                    view.selected = None;
                    view.document = Some(Arc::downgrade(&app.session.doc));
                    view.revision = Some(app.session.revision);
                }
                Err(error) => app.set_status(error, true),
            }
        }
        let label = view.selected.clone().unwrap_or_else(|| app.tr("render.material_choose_body").into());
        ui_kit::dropdown_menu(ui, "render_base_target", &label, |ui| {
            for item in &view.items {
                let Some(name) = item["name"].as_str() else { continue };
                if ui_kit::list_picker(ui, &format!("base:{name}"), name, view.selected.as_deref() == Some(name)).clicked() {
                    view.selected = Some(name.into());
                    view.color = serde_json::from_value(item["color"].clone()).unwrap_or([160, 170, 180]);
                    view.opaque = item["opacity"].as_f64() == Some(1.0);
                    view.metallic = item["pbr"]["metallic"].as_f64().unwrap_or(0.0);
                    view.roughness = item["pbr"]["roughness"].as_f64().unwrap_or(0.5);
                    view.ior = item["pbr"]["dielectric_ior"].as_f64().unwrap_or(1.5);
                    ui.close();
                }
            }
        });
        if view.selected.is_some() {
            ui.label(app.tr("render.base_color"));
            ui_kit::color_field(ui, &mut view.color);
            for (key, value, range) in [
                ("render.base_metallic", &mut view.metallic, 0.0..=1.0),
                ("render.material_roughness", &mut view.roughness, 0.045..=1.0),
                ("render.material_ior", &mut view.ior, 1.0..=3.0),
            ] {
                let label = app.tr(key).to_string();
                ui_kit::row(ui, &label, |ui| ui_kit::number_field(ui, value, range, 0.01, ""));
            }
            let current = view.revision == Some(app.session.revision)
                && view.document.as_ref().is_some_and(|d| Weak::ptr_eq(d, &Arc::downgrade(&app.session.doc)))
                && !app.session.model.is_stale();
            if !current {
                ui_kit::text(ui, app.tr("render.material_refresh_needed"), ui_kit::Text::Note("render.material_refresh_needed"));
            }
            if !view.opaque {
                ui_kit::text(ui, app.tr("render.base_opaque_only"), ui_kit::Text::Note("render.base_opaque_only"));
            }
            if ui_kit::button(ui, app.tr("render.base_apply"), ui_kit::Button::Enabled(current && view.opaque)).clicked()
                && let Some(name) = &view.selected
            {
                match app.run(
                    "appearance.assign",
                    json!({"bodies":[name],"color":view.color,"opacity":1.0,
                    "pbr":{"metallic":view.metallic,"roughness":view.roughness,"dielectric_ior":view.ior}}),
                ) {
                    Ok(_) => {
                        view.revision = Some(app.session.revision);
                        view.document = Some(Arc::downgrade(&app.session.doc));
                    }
                    Err(error) => app.set_status(error, true),
                }
            }
            ui_kit::text(ui, app.tr("render.base_apply_help"), ui_kit::Text::Note("render.base_apply_help"));
        }
        app.render_workspace.view.inspector.appearance = view;
    });
}

#[cfg(test)]
#[path = "render_workspace_inspector_tests.rs"]
mod tests;

pub(super) fn fit_subject(app: &mut SolveApp) -> Result<(), String> {
    // Capture the current navigation orientation before the engine's view command.
    app.run("render.camera.set", json!({"camera":app.cam}))?;
    app.run("render.camera.fit", json!({}))?;
    app.follow_view_request(false);
    app.fit_view();
    Ok(())
}

pub(super) fn restore_camera(app: &mut SolveApp) -> Result<(), String> {
    let name = app.render_workspace.view.name.clone();
    app.run("render.settings.apply", json!({"name":name}))?;
    app.follow_view_request(false);
    Ok(())
}
fn camera_actions(app: &mut SolveApp, ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        if ui_kit::button(ui, app.tr("render.fit"), ui_kit::Button::Normal).clicked()
            && let Err(error) = fit_subject(app)
        {
            app.set_status(error, true);
        }
        if ui_kit::button(ui, app.tr("render.restore_camera"), ui_kit::Button::Normal).clicked()
            && let Err(error) = restore_camera(app)
        {
            app.set_status(error, true);
        }
    });
}

pub(super) fn confirmation_open(app: &SolveApp) -> bool {
    app.render_workspace.view.inspector.pending.is_some() || app.render_workspace.view.save_shot_name.is_some()
}
pub(super) fn reveal(app: &mut SolveApp, section: &'static str) {
    app.render_workspace.view.inspector.reveal = Some(section);
}
fn reveal_section(app: &mut SolveApp, ui: &mut egui::Ui, section: &'static str) {
    if app.render_workspace.view.inspector.reveal == Some(section) {
        let identity = ui.make_persistent_id(section);
        ui.ctx().data_mut(|data| data.insert_persisted(identity, true));
        ui.scroll_to_cursor(Some(egui::Align::Min));
        app.render_workspace.view.inspector.reveal = None;
    }
}

fn gpu_status(app: &SolveApp, ui: &mut egui::Ui) {
    match &app.render_workspace.path_host {
        super::PathHost::Preparing(_, started) => super::setup_progress(ui, app, *started),
        super::PathHost::Failed(error) => {
            ui_kit::text(ui, &format!("{}: {error}", app.tr("render.gpu_path_failed")), ui_kit::Text::Error("render.gpu_path_failed"));
        }
        super::PathHost::NotStarted => {
            ui_kit::text(ui, app.tr("render.gpu_unavailable"), ui_kit::Text::Note("render.gpu_unavailable"));
        }
        super::PathHost::Ready(_) => {}
    }
}
