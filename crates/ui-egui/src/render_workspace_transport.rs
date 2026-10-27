//! Shared numerical editors for bounded GPU transport extensions.
use super::studio::vector;
use crate::{SolveApp, ui_kit};

/// Pilot allocation and conditional transmission sampling share portable intent.
pub(super) fn sampling_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    adaptive_controls(app, ui);
    glass_controls(app, ui);
}
pub(super) fn adaptive_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let before = app.render_workspace.view.studio.adaptive;
    let mut enabled = before.is_some();
    let response = ui_kit::checkbox(ui, app.tr("render.adaptive_enable"), &mut enabled);
    crate::scenario::publish_target("render:adaptive-enable", response.rect);
    if response.changed() {
        app.render_workspace.view.studio.adaptive = enabled.then(Default::default);
    }
    let mut options = app.render_workspace.view.studio.adaptive.unwrap_or_default();
    if !enabled {
        ui_kit::text(ui, app.tr("render.children.enable_adaptive"), ui_kit::Text::Note("render.children.enable_adaptive"));
    }
    ui.add_enabled_ui(enabled, |ui| {
        for key in
            ["render.adaptive_pilots", "render.adaptive_minimum", "render.adaptive_relative", "render.adaptive_absolute", "render.adaptive_coverage"]
        {
            let label = match key {
                "render.adaptive_pilots" => "render.children.pilots",
                "render.adaptive_minimum" => "render.children.minimum",
                "render.adaptive_relative" => "render.children.relative_error",
                "render.adaptive_absolute" => "render.children.absolute_floor",
                _ => "render.children.coverage_error",
            };
            let response = ui_kit::row(ui, app.tr(label), |ui| match key {
                "render.adaptive_pilots" => ui_kit::number_field(ui, &mut options.pilot_samples, 16..=256, 1.0, ""),
                "render.adaptive_minimum" => ui_kit::number_field(ui, &mut options.min_samples, 16..=4096, 1.0, ""),
                "render.adaptive_relative" => ui_kit::number_field(ui, &mut options.relative_error, 0.0..=1.0, 0.005, ""),
                "render.adaptive_absolute" => ui_kit::number_field(ui, &mut options.absolute_error, 0.000001..=1e6, 0.001, ""),
                _ => ui_kit::number_field(ui, &mut options.coverage_error, 0.001..=0.25, 0.001, ""),
            });
            crate::scenario::publish_target(key, response.rect);
        }
    });
    if enabled {
        app.render_workspace.view.studio.adaptive = Some(options);
    }
    ui_kit::text(ui, app.tr("render.adaptive_help"), ui_kit::Text::Note("render.adaptive_help"));
    app.render_workspace.view.dirty |= before != app.render_workspace.view.studio.adaptive;
}
pub(super) fn glass_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let before = (app.render_workspace.view.studio.nested_media, app.render_workspace.view.studio.transmission_samples);
    let mut strata = match before.1 {
        2 => 1,
        4 => 2,
        _ => 0,
    };
    ui_kit::row(ui, app.tr("render.children.fresnel"), |ui| {
        if ui_kit::segmented(ui, &mut strata, &["1", "2", "4"], Default::default()) {
            app.render_workspace.view.studio.transmission_samples = match strata {
                1 => 2,
                2 => 4,
                _ => 1,
            };
        }
    });
    let label = app.tr("render.children.nested").to_string();
    let response = ui_kit::checkbox(ui, &label, &mut app.render_workspace.view.studio.nested_media);
    crate::scenario::publish_target("render:nested-enable", response.rect);
    ui_kit::text(ui, app.tr("render.nested_help"), ui_kit::Text::Note("render.nested_help"));
    app.render_workspace.view.dirty |=
        before != (app.render_workspace.view.studio.nested_media, app.render_workspace.view.studio.transmission_samples);
}
pub(super) fn volume_controls(app: &mut SolveApp, ui: &mut egui::Ui) {
    let title = app.tr("render.volume_title").to_string();
    ui_kit::section(ui, "render_homogeneous_volume", &title, false, |ui| {
        let before = app.render_workspace.view.studio.volume;
        let mut enabled = before.is_some();
        let toggle = ui_kit::checkbox(ui, app.tr("render.volume_enable"), &mut enabled);
        crate::scenario::publish_target("render:volume-enable", toggle.rect);
        if toggle.changed() {
            app.render_workspace.view.studio.volume = enabled.then(Default::default);
        }
        if enabled {
            for key in ["render.volume_min", "render.volume_max", "render.volume_albedo"] {
                let label = app.tr(key).to_string();
                if let Some(volume) = &mut app.render_workspace.view.studio.volume {
                    let (value, range, speed) = match key {
                        "render.volume_min" => (&mut volume.min_mm, -1e9..=1e9, 1.0),
                        "render.volume_max" => (&mut volume.max_mm, -1e9..=1e9, 1.0),
                        _ => (&mut volume.scattering_albedo, 0.0..=1.0, 0.01),
                    };
                    vector(ui, &label, value, range, speed);
                }
            }
            for key in ["render.volume_extinction", "render.volume_anisotropy"] {
                let label = app.tr(key).to_string();
                if let Some(volume) = &mut app.render_workspace.view.studio.volume {
                    let response = ui_kit::row(ui, &label, |ui| match key {
                        "render.volume_extinction" => ui_kit::number_field(ui, &mut volume.extinction_per_mm, 0.0..=1000.0, 0.0001, " /mm"),
                        _ => ui_kit::number_field(ui, &mut volume.anisotropy, -0.95..=0.95, 0.01, ""),
                    });
                    crate::scenario::publish_target(key, response.rect);
                }
            }
            ui_kit::text(ui, app.tr("render.volume_help"), ui_kit::Text::Note("render.volume_help"));
        }
        if before != app.render_workspace.view.studio.volume {
            app.render_workspace.view.dirty = true;
        }
    });
}
