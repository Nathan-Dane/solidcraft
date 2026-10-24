//! Render shell composition; all hosting and chrome belong to the shared kit.
use crate::{
    SolveApp,
    theme::Tokens,
    ui_kit::{self, Container, Edge},
};
use egui::{Rect, pos2, vec2};

pub(crate) fn reset_shell(app: &mut SolveApp) {
    for id in ["render:inspector", "render:browser", "render:appearances", "render:shots_strip", "render:hud", "render:shots:show"] {
        crate::shell::register_panel(app, id, None, None);
    }
}

pub(crate) fn canvas(app: &SolveApp, viewport: Rect) -> Rect {
    crate::shell::content_rect(app, viewport)
}

pub(crate) fn overlays(app: &mut SolveApp, ui: &mut egui::Ui) {
    let Some(viewport) = app.viewport.rect else { return };
    let free = canvas(app, viewport);
    if !free.is_positive() {
        return;
    }
    let ctx = ui.ctx().clone();
    let hud = Container::navigation_card(&ctx, egui::Id::new("sc_render_hud"), pos2(free.center().x, free.top()), egui::Align2::CENTER_TOP, |ui| {
        ui.set_max_width((free.width() - Tokens::get().space_24).max(0.0));
        ui.horizontal_wrapped(|ui| {
            let mode = current_pick_mode(app);
            ui_kit::text(ui, app.tr("render.pick_label"), ui_kit::Text::Inline);
            ui.add_enabled_ui(!crate::render_workspace::result_active(app), |ui| {
                let label = pick_label(mode);
                let response = ui_kit::dropdown_menu(ui, "render_pick", app.tr(label), |ui| {
                    for mode in ["body", "face", "component"] {
                        let response = ui_kit::choice_chip(ui, mode, app.tr(pick_label(mode)), mode == current_pick_mode(app));
                        crate::scenario::publish_target(&format!("render:pick:{mode}"), response.rect);
                        if response.clicked() {
                            let _ = app.run("select.filter", serde_json::json!({"mode":mode}));
                            ui.close();
                        }
                    }
                })
                .on_hover_text(app.tr("render.pick_help"));
                crate::scenario::publish_target("render:pick", response.rect);
            });
            ui_kit::text(ui, crate::i18n::tr_label("Integrated preview"), ui_kit::Text::Inline)
                .on_hover_text(crate::i18n::tr_label("The live preview shows direct light only and no shadows."));
        });
    });
    crate::shell::register_panel(app, "render:hud", Some(hud.response.rect), None);
    crate::scenario::publish_rect("render:hud", hud.response.rect);
    crate::viewport::nav_bar(app, ui, viewport);
    shots_pill(app, ui, free);
}

fn current_pick_mode(app: &SolveApp) -> &'static str {
    if app.ui.pick_components {
        "component"
    } else if app.ui.pick_bodies {
        "body"
    } else {
        "face"
    }
}

fn pick_label(mode: &str) -> &'static str {
    match mode {
        "body" => "render.pick_body",
        "component" => "render.pick_component",
        _ => "render.pick_face",
    }
}

fn shots_pill(app: &mut SolveApp, ui: &mut egui::Ui, free: Rect) {
    crate::shell::register_panel(app, "render:shots:show", None, None);
    if app.preferences.render_shots_strip {
        return;
    }
    let ctx = ui.ctx().clone();
    let pill = Container::navigation_card(
        &ctx,
        egui::Id::new("sc_render_shots_pill"),
        free.left_bottom() + vec2(Tokens::get().space_8, -Tokens::get().space_8),
        egui::Align2::LEFT_BOTTOM,
        |ui| {
            let response =
                ui_kit::button(ui, crate::i18n::tr_label("Shots"), ui_kit::Button::Normal).on_hover_text(crate::i18n::tr_label("Show Shots strip"));
            crate::scenario::publish_target("render:shots:show", response.rect);
            if response.clicked() {
                let _ = app.run("view.shots_strip", serde_json::json!({"visible":true}));
            }
        },
    );
    crate::shell::register_panel(app, "render:shots:show", Some(pill.response.rect), None);
}

/// Existing saved-shot and job actions inside the shared floating bottom-strip host.
pub(crate) fn shots_strip(app: &mut SolveApp, ui: &mut egui::Ui) {
    crate::shell::register_panel(app, "render:shots_strip", None, None);
    if !app.preferences.render_shots_strip {
        return;
    }
    let available = ui.available_rect_before_wrap();
    let content = ui_kit::Panel::bottom_strip(ui, "sc_render_shots_strip", available, Tokens::get().form_footer_height, |ui| {
        crate::render_workspace::shots::contents(app, ui);
    });
    let rect = content.map(|c| c.response.rect);
    crate::shell::register_panel(app, "render:shots_strip", rect, Some(Edge::Bottom));
    if let Some(rect) = rect {
        crate::scenario::publish_rect("render:shots_strip", rect);
    }
}
