//! The studio's small scenarios call the production kit and original artwork directly.
use super::*;
use crate::ui_kit::{BackingTooltipExt as _, HistoryChip, HistoryStyle, HistoryTransport, Row, TreeAction};
use egui::{Sense, pos2, vec2};

impl StudioPreview {
    pub(super) fn scene_card(&mut self, ui: &mut Ui, id: &str) {
        let t = Tokens::get();
        let (title, description, surface) = match id {
            "tools" => ("Workspace strip", "Tabs, semantic tool colours and navigation controls", Surface::Chrome),
            "browser" => ("Browser & History", "Fold, hide, select and inspect the actual dense rows", Surface::Panel),
            "controls" => ("Feature inspector", "Actual fields, sliders, disabled states and theme options", Surface::Feature),
            "glass" => ("Layered surfaces", "Real menus, submenus, tooltips and a modal over detailed content", Surface::Window),
            "viewport" => ("Viewport token study", "Analytic geometry with production grid, dimensions, snap markers and cube", Surface::Panel),
            _ => ("Icon lab & File cards", "Original SVG geometry, knockout, masks and optical variants", Surface::Panel),
        };
        let host = Container::floating_card(surface);
        let frame = host
            .frame(ui.ctx(), if id == "controls" || id == "glass" { t.dialog_body } else { t.panel }, crate::frame::Side::Left)
            .inner_margin(t.space_12);
        let bounds = ui.max_rect();
        let response = ui
            .push_id(id, |ui| {
                host.show_frame(ui, frame, bounds, |ui| {
                    ui.set_min_width((ui.available_width() - t.space_8).max(180.0));
                    let heading = ui.label(egui::RichText::new(title).font(crate::fonts::semibold(t.font_heading)).color(t.text));
                    ui.label(egui::RichText::new(description).size(t.font_small).color(t.text_dim));
                    self.record(ui, heading.rect, title, &["text", "text_dim", "panel"], id);
                    if self.focus.as_deref() == Some(id) {
                        ui.scroll_to_rect(heading.rect, Some(egui::Align::Center));
                        self.focus = None;
                    }
                    ui.add_space(t.space_8);
                    ui_kit::scope(ui, |ui| match id {
                        "tools" => self.tools_scene(ui),
                        "browser" => self.browser_scene(ui),
                        "controls" => self.controls_scene(ui),
                        "glass" => self.glass_scene(ui),
                        "viewport" => {
                            let response = crate::theme_studio_viewport::ui(ui);
                            self.record(
                                ui,
                                response.rect,
                                "Analytic native token study",
                                &["grid", "grid_major", "axis_x", "axis_y", "axis_z", "body_edge", "sel_edge", "sketch", "dimension"],
                                id,
                            );
                        }
                        _ => self.icons_scene(ui),
                    });
                })
            })
            .inner;
        self.record(ui, response.response.rect, title, &["panel", "dialog_body", "border", "dialog_border", "borders.primary_inset"], id);
    }

    fn tools_scene(&mut self, ui: &mut Ui) {
        let t = Tokens::get();
        let current = self.tab.clone();
        let menu = ui_kit::Menu::text(ui, &format!("Workspace: {current}"), |ui| {
            for tab in theme::TAB_NAMES {
                let response = ui_kit::MenuItem::new(tab, tab, "").show(ui);
                self.record(ui, response.rect, tab, &["selected_bg", "selected_ink", "hover"], "tools");
                if response.clicked() {
                    self.tab = (*tab).into();
                    self.native_revision = self.native_revision.wrapping_add(1);
                    ui.close();
                }
            }
        });
        self.record(ui, menu.response.rect, "Workspace tab", &["panel_header", "text", "accent_soft"], "tools");
        ui.horizontal_wrapped(|ui| {
            for (index, (icon, label)) in
                [("extrude", "Extrude"), ("hole", "Hole"), ("fillet", "Fillet"), ("surface_loft", "Surface loft"), ("flange", "Flange")]
                    .into_iter()
                    .enumerate()
            {
                ui.vertical(|ui| {
                    let (rect, response) = ui.allocate_exact_size(vec2(42.0, 42.0), Sense::click());
                    if self.selected == index {
                        ui.painter().rect_filled(rect, t.control_radius, theme::TabAccent::current(ui.ctx()).soft);
                    }
                    crate::icons::paint_icon(ui.painter(), rect.shrink(t.space_4), icon, 1.0);
                    if response.clicked() {
                        self.selected = index;
                        self.status = format!("Selected {label}");
                    }
                    response.backing_on_hover_text(label);
                    ui.label(egui::RichText::new(label).size(t.font_small));
                    self.record(ui, rect, label, &["icon_palette.solid", "icon_palette.surface", "icon_palette.line", "selected_bg"], "tools");
                });
            }
        });
        ui.add_space(t.space_8);
        let size = crate::navigation_bar::Size::Regular;
        let metrics = ui_kit::NavigationMetrics::for_size(size);
        let inset = ui_kit::NAV_PADDING + ui_kit::NAV_BORDER;
        let menu_width = metrics.icon + ui_kit::NAV_MENU_LEFT + ui_kit::NAV_MENU_RIGHT + ui_kit::NAV_MENU_GAP + ui_kit::NAV_CHEVRON;
        let (rect, _) =
            ui.allocate_exact_size(vec2(2.0 * metrics.button + menu_width + 2.0 * ui_kit::NAV_GAP + 2.0 * inset, metrics.height()), Sense::hover());
        ui_kit::navigation_surface(ui, rect, size, true);
        let mut x = rect.left() + inset;
        for (id, icon, kind, width) in [
            ("pan", "pan", crate::navigation_bar::Kind::Tool, metrics.button),
            ("fit", "fit", crate::navigation_bar::Kind::Action, metrics.button),
            ("visual_style", "style_shaded_edges", crate::navigation_bar::Kind::Menu, menu_width),
        ] {
            let bounds = Rect::from_min_size(pos2(x, rect.top() + inset), vec2(width, metrics.button));
            let response = ui_kit::navigation_button(
                ui,
                bounds,
                ui_kit::NavigationButton {
                    id,
                    icon,
                    label: id,
                    kind,
                    selected: id == "pan",
                    labels: false,
                    menu: kind == crate::navigation_bar::Kind::Menu,
                },
            );
            if response.clicked() {
                self.status = format!("Navigation {id}");
            }
            self.record(ui, bounds, id, &["nav_viewport", "nav_hover", "nav_press", "nav_dim", "icon"], "tools");
            x += width + ui_kit::NAV_GAP;
        }
        ui.add_enabled_ui(false, |ui| {
            ui_kit::button(ui, "Disabled command", ui_kit::Button::Normal);
        });
    }

    fn browser_scene(&mut self, ui: &mut Ui) {
        let t = Tokens::get();
        let actions = [TreeAction { icon: "lookat", hover_only: true, label: "Look at", reason: None, target: "studio_tree_look" }];
        let parent = ui_kit::tree_row(
            ui,
            ui.id().with("body_group"),
            &Row { label: "Bodies", fold: Some(self.tree_open), icon: "folder", count: "3", ..Default::default() },
        );
        if parent.fold {
            self.tree_open = !self.tree_open;
        }
        if let Some(rect) = parent.rect {
            self.record(ui, rect, "Bodies folder", &["text", "text_dim", "icon", "hover"], "browser");
        }
        if self.tree_open {
            for (index, (label, icon, dim, status)) in [
                ("Mounting plate", "body", false, None),
                ("Bend flange", "flange", false, Some(ui_kit::Status::Warning)),
                ("Hidden reference", "sketch", true, None),
            ]
            .into_iter()
            .enumerate()
            {
                let response = ui_kit::tree_row(
                    ui,
                    ui.id().with(("tree", index)),
                    &Row {
                        depth: 1,
                        label,
                        icon,
                        name: true,
                        eye: Some(index != 2 && self.tree_visible),
                        selected: self.tree_selected == index,
                        dim,
                        status,
                        actions: &actions,
                        badges: if index == 1 { &["lock"] } else { &[] },
                        tip: Some("Sample document row — no document is modified"),
                        ..Default::default()
                    },
                );
                if response.clicked {
                    self.tree_selected = index;
                }
                if response.eye {
                    self.tree_visible = !self.tree_visible;
                }
                if response.action {
                    self.status = format!("Look at {label}");
                }
                if let Some(rect) = response.rect {
                    self.record(ui, rect, label, &["text", "text_dim", "selected_bg", "selected_ink", "hover", "warning"], "browser");
                }
            }
        }
        ui.add_space(t.space_8);
        ui_kit::StripHeading::new(ui.painter(), "History").show(ui);
        ui.horizontal(|ui| {
            for (index, (icon, name, kind)) in
                [("sketch", "Base sketch", "Sketch"), ("extrude", "Extrude", "Extrude"), ("fillet", "Fillet", "Fillet")].into_iter().enumerate()
            {
                let (rect, response) = ui.allocate_exact_size(vec2(36.0, ui_kit::HistoryStyle::Icons.height()), Sense::click());
                HistoryChip {
                    icon,
                    name,
                    kind,
                    selected: self.history_selected == index,
                    hovered: response.hovered(),
                    dim: false,
                    suppressed: index == 2,
                    error: false,
                    warning: index == 1,
                    reference: false,
                    accent: theme::TabAccent::current(ui.ctx()),
                    ink: None,
                }
                .paint(ui.painter(), rect, HistoryStyle::Icons);
                if response.clicked() {
                    self.history_selected = index;
                }
                response.backing_on_hover_text(name);
                self.record(ui, rect, name, &["timeline_item", "hover", "selected_bg", "form_error_fill", "warning"], "browser");
            }
            for control in HistoryTransport::WITH_PLAY {
                let (rect, _) = ui.allocate_exact_size(vec2(t.control_height, t.control_height), Sense::hover());
                if control.show(ui, rect, self.playing).clicked() {
                    if control == HistoryTransport::Play {
                        self.playing = !self.playing;
                    } else {
                        self.history_selected = match control {
                            HistoryTransport::First => 0,
                            HistoryTransport::Back => self.history_selected.saturating_sub(1),
                            HistoryTransport::Last => 2,
                            _ => (self.history_selected + 1).min(2),
                        };
                    }
                }
            }
        });
    }

    fn controls_scene(&mut self, ui: &mut Ui) {
        let t = Tokens::get();
        ui_kit::settings_form(ui, 80.0, |ui| {
            let response = ui.scope(|ui| ui_kit::text_row(ui, "Name", &mut self.name, "Feature name"));
            self.record(ui, response.response.rect, "Name field", &["field", "control_fill", "form_ink", "control_border"], "controls");
            ui_kit::row(ui, "Distance", |ui| {
                let response = ui_kit::number_field(ui, &mut self.distance, 0.0..=100.0, 0.1, " mm");
                self.record(ui, response.rect, "Distance field", &["control_fill", "form_ink", "control_border", "form_primary"], "controls");
            });
            ui_kit::row(ui, "Draft", |ui| {
                let response = ui_kit::slider(ui, &mut self.distance, 0.0..=100.0, ui_kit::Slider::default());
                self.record(ui, response.rect, "Slider", &["dialog_rule", "form_primary", "control_fill"], "controls");
            });
            ui_kit::row(ui, "Options", |ui| {
                let response = ui_kit::checkbox(ui, "Symmetric", &mut self.checked);
                self.record(ui, response.rect, "Symmetric", &["control_fill", "control_border", "form_primary_fill"], "controls");
            });
        });
        ui.horizontal(|ui| {
            if ui_kit::button(ui, "Apply", ui_kit::Button::Primary).clicked() {
                self.status = "Applied the sample inspector values".into();
            }
            if ui_kit::button(ui, "Cancel", ui_kit::Button::Normal).clicked() {
                self.distance = 24.0;
                self.name = "Mounting plate".into();
            }
            ui.add_enabled_ui(false, |ui| {
                ui_kit::button(ui, "Disabled", ui_kit::Button::Normal);
            });
        });
        let file = self.file.clone();
        let definitions = theme_files::ordered_options(&file);
        let rows = definitions
            .iter()
            .map(|definition| {
                let provider = crate::theme_options::provider(&definition.id, crate::theme_options::PROVIDERS);
                ui_kit::ThemeOptionRow {
                    definition,
                    label: provider
                        .map(|p| crate::i18n::tr(p.label).to_owned())
                        .unwrap_or_else(|| crate::prefs::theme_option_text(&definition.label, &self.language, &definition.id)),
                    help: provider
                        .map(|p| crate::i18n::tr(p.help).to_owned())
                        .unwrap_or_else(|| crate::prefs::theme_option_text(&definition.help, &self.language, "")),
                    choices: match &definition.kind {
                        theme_files::OptionKind::Choice { values, .. } => {
                            values.iter().map(|v| crate::prefs::theme_option_text(&v.label, &self.language, &v.id)).collect()
                        }
                        _ => Vec::new(),
                    },
                    reason: None,
                    blur: definition.id == "blur",
                }
            })
            .collect::<Vec<_>>();
        let mut effective = theme_files::effective_values(&file, &self.values).0;
        let before = effective.clone();
        let response = ui.scope(|ui| ui_kit::theme_options_list(ui, &file.id, &file.name, &rows, &mut effective));
        self.record(ui, response.response.rect, "Theme option controls", &["form_primary", "control_fill", "text", "dialog_rule"], "controls");
        if response.inner || before != effective {
            self.values = if response.inner { BTreeMap::new() } else { effective };
            self.native_revision = self.native_revision.wrapping_add(1);
            self.dirty_style = true;
            ui.ctx().request_repaint();
        }
        if !self.status.is_empty() {
            ui.label(egui::RichText::new(&self.status).color(t.form_dim).size(t.font_small));
        }
    }

    fn glass_scene(&mut self, ui: &mut Ui) {
        let t = Tokens::get();
        for index in 0..4 {
            let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 18.0), Sense::hover());
            ui.painter().rect_filled(rect, t.radius_sm, if index % 2 == 0 { t.selected_bg } else { t.panel_header });
            crate::icons::paint_icon(ui.painter(), Rect::from_min_size(rect.min, vec2(18.0, 18.0)), "sketch", 1.0);
            ui.painter().text(
                rect.min + vec2(26.0, 2.0),
                egui::Align2::LEFT_TOP,
                format!("Detailed content layer {}", index + 1),
                crate::fonts::medium(t.font_small),
                t.text,
            );
        }
        ui.horizontal(|ui| {
            let menu = ui_kit::Menu::text(ui, "Open menu", |ui| {
                for (id, label, icon) in [("open", "Open", "open"), ("copy", "Copy", "copy"), ("delete", "Delete", "delete")] {
                    let response = ui_kit::MenuItem::new(id, label, icon).show(ui);
                    self.record(ui, response.rect, label, &["dialog_body", "form_ink", "selected_bg", "hover"], "glass");
                    if response.clicked() {
                        self.status = format!("{label} sample");
                        ui.close();
                    }
                }
                ui_kit::Menu::text(ui, "Advanced…", |ui| {
                    let response = ui_kit::MenuItem::new("advanced_sample", "Inspect details", "inspect").show(ui);
                    self.record(ui, response.rect, "Submenu row", &["dialog_body", "form_ink", "hover"], "glass");
                    if response.clicked() {
                        self.status = "Inspected sample details".into();
                        ui.close();
                    }
                });
            });
            self.record(ui, menu.response.rect, "Open menu", &["dialog_body", "control_fill", "control_border"], "glass");
            if ui_kit::button(ui, "Open modal", ui_kit::Button::Normal).clicked() {
                self.modal_open = true;
            }
            let tip = ui_kit::button(ui, "Hover for tooltip", ui_kit::Button::Ghost);
            Container::tooltip_ui(&tip, |ui| {
                ui.label("Production tooltip, including native timing and layering.");
                self.record(ui, ui.min_rect(), "Tooltip", &["dialog_body", "dialog_border", "text"], "glass");
            });
        });
        let nested = Container::floating_card(Surface::Panel);
        let frame = nested.frame(ui.ctx(), t.panel, crate::frame::Side::Left).inner_margin(t.space_8);
        let response = nested.show_frame(ui, frame, ui.max_rect(), |ui| {
            ui.label("Nested backing surface");
            ui.label(egui::RichText::new("The ordered compositor samples the content below each actual surface.").size(t.font_small));
        });
        self.record(ui, response.response.rect, "Nested backing surface", &["panel", "border", "text"], "glass");
    }

    fn icons_scene(&mut self, ui: &mut Ui) {
        let t = Tokens::get();
        let ppp = ui.ctx().pixels_per_point();
        ui.label(egui::RichText::new(format!("Physical pixels · scale {ppp:.2}×")).color(t.text_dim).size(t.font_small));
        for (index, (id, role)) in [
            ("extrude", "solid"),
            ("surface_loft", "surface"),
            ("arc_slot", "sketch"),
            ("coil", "solid"),
            ("bendtable", "solid"),
            ("interference", "component-orange-1"),
            ("c_parallel", "constraint"),
            ("plane_midplane", "construction"),
        ]
        .into_iter()
        .enumerate()
        {
            ui.horizontal(|ui| {
                let response = ui_kit::choice_chip(ui, id, id, self.icon_selected == index);
                if response.clicked() {
                    self.icon_selected = index;
                }
                for physical in [16.0, 24.0, 32.0, 48.0] {
                    let side = physical / ppp;
                    let (rect, response) = ui.allocate_exact_size(vec2(side, side), Sense::hover());
                    crate::icons::paint_icon(ui.painter(), rect, id, 1.0);
                    response.backing_on_hover_text(format!("{id} · {physical:.0} physical px"));
                    self.record(ui, rect, id, &["icon_palette.line", "icon_palette.ink", role], "icons");
                }
                ui.add_enabled_ui(false, |ui| {
                    let (rect, _) = ui.allocate_exact_size(vec2(24.0 / ppp, 24.0 / ppp), Sense::hover());
                    crate::icons::paint_icon(ui.painter(), rect, id, 1.0);
                });
            });
        }
        ui.add_space(t.space_8);
        let response =
            ui_kit::FileView::action_tile(ui, "studio_new_file", ui.available_width(), "new", "New design", "Actual File-page action card", true);
        self.record(ui, response.rect, "File action card", &["file_primary", "file_primary_ink", "file_primary_border", "file_surface"], "icons");
        if response.clicked() {
            self.status = "Opened the sample File action".into();
        }
    }
}
