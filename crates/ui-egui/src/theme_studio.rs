//! Optional authoring facade: production theme codec, token resolver, fonts, artwork and kit.
//! No SolveApp, document, model evaluation or second theme implementation is instantiated.
use crate::{
    theme::{self, ColorVision, Tokens},
    theme_files::{self, OptionValue, ThemeFile},
    ui_kit::{self, Container, Surface},
};
use egui::{Rect, Ui};
#[path = "theme_studio_scenes.rs"]
mod scenes;
use serde_json::{Value, json};
use std::collections::BTreeMap;

pub const ONYX_SOURCE: &str = include_str!("../../../assets/styles/ui/themes/onyx.json");

fn failure(error: impl ToString) -> Value {
    json!({"ok":false,"errors":[error.to_string()]})
}

fn input(source: &str, values: &str, tab: &str, vision: &str) -> Result<(ThemeFile, BTreeMap<String, OptionValue>, ColorVision), String> {
    let file = theme_files::parse(source)?;
    if values.len() > theme_files::MAX_BYTES {
        return Err("Preview values exceed the theme input limit.".into());
    }
    if !theme::TAB_NAMES.contains(&tab) {
        return Err(format!("Unknown preview tab: {tab}"));
    }
    let vision = ColorVision::from_id(vision).ok_or_else(|| format!("Unknown colour-vision mode: {vision}"))?;
    let values: BTreeMap<String, OptionValue> = serde_json::from_str(values).map_err(|e| format!("Preview option values: {e}"))?;
    Ok((file, values, vision))
}

/// Describe native vocabulary, including scalar/list arity, without a duplicated JS schema.
pub fn contract() -> Value {
    let light = theme_files::export("Light", theme_files::Base::Light, &Tokens::light());
    let dark = theme_files::export("Dark", theme_files::Base::Dark, &Tokens::dark());
    let colors = theme_files::keys()
        .into_iter()
        .map(|key| {
            let arity = light.get("colors").and_then(|c| c.get(key)).and_then(Value::as_array).map_or(1, Vec::len);
            json!({"key":key,"arity":arity,"alphaAllowed":true})
        })
        .collect::<Vec<_>>();
    let mut out = theme_files::studio_shape_contract();
    if let Some(out) = out.as_object_mut() {
        out.insert("apiVersion".into(), json!(1));
        out.insert("format".into(), json!(theme_files::FORMAT));
        out.insert("supportsLevel".into(), json!(theme_files::REQUIREMENTS_LEVEL));
        out.insert("sourceCommit".into(), json!(option_env!("SOLIDCRAFT_THEME_STUDIO_REVISION").unwrap_or("development")));
        out.insert("colorKeys".into(), json!(colors));
        out.insert("baseDefaults".into(), json!({"light":light,"dark":dark}));
        out.insert("iconDefaults".into(), json!({"light":crate::icon_svg::studio_defaults(false),"dark":crate::icon_svg::studio_defaults(true)}));
        out.insert(
            "capabilities".into(),
            json!({"inspect":true,"comparison":true,"sceneSelection":true,"actualWidgets":true,"actualBlurPipeline":true}),
        );
        out.insert("iconRoleKeys".into(), json!(crate::icon_svg::ROLE_NAMES.iter().filter(|name| **name != "gap").collect::<Vec<_>>()));
        out.insert("iconAccentKeys".into(), json!(std::iter::once("solid").chain(crate::icon_svg::ACCENT_NAMES.iter().copied()).collect::<Vec<_>>()));
        out.insert("starter".into(), serde_json::from_str(ONYX_SOURCE).unwrap_or(Value::Null));
        let standard = theme_files::from_value(&json!({"format":theme_files::FORMAT,"name":"Standard controls","base":"dark","options":[
            {"id":"opacity","type":"range"},{"id":"blur","type":"range","default":0},
            {"id":"tab_colours","type":"bool","default":false},{"id":"tab_fade_ms","type":"range","default":240}]}));
        out.insert("standardOptions".into(), standard.map(|f| json!(f.options)).unwrap_or(Value::Null));
        out.insert("scenes".into(), json!(["all", "tools", "browser", "controls", "glass", "viewport", "icons"]));
        out.insert(
            "languages".into(),
            json!(crate::i18n::LOCALES.iter().map(|locale| json!({"id":locale.id,"label":locale.name})).collect::<Vec<_>>()),
        );
        out.insert("defaultLanguage".into(), json!("en-GB"));
        out.insert(
            "scriptFonts".into(),
            json!([
                {"language":"ja","url":"./fonts/BIZUDPGothic-Regular.ttf"},
                {"language":"zh","url":"./fonts/CraftUI-SC.otf"},
                {"language":"zh-TW","url":"./fonts/CraftUI-TC.otf"},
                {"language":"ko","url":null,"reason":"The production web font pack does not yet include Hangul."}
            ]),
        );
    }
    out
}

/// Evaluate a portable definition. Raw author fields remain separate from normalized tables.
pub fn evaluate(source: &str, values: &str, tab: &str, vision: &str) -> Value {
    let (file, saved, vision) = match input(source, values, tab, vision) {
        Ok(v) => v,
        Err(e) => return failure(e),
    };
    let (values, notices) = theme_files::effective_values(&file, &saved);
    let tokens = theme_files::resolve_for_tab(&file, &saved, Some(tab)).with_vision(file.base.is_dark(), vision);
    let definitions = theme_files::ordered_options(&file)
        .into_iter()
        .map(|option| {
            let mut definition = serde_json::to_value(option).unwrap_or(Value::Null);
            if let Some(obj) = definition.as_object_mut() {
                let availability = crate::theme_options::availability(
                    &file,
                    &option.id,
                    crate::theme_options::PROVIDERS,
                    crate::theme_options::Availability::Available,
                );
                let available = availability == crate::theme_options::Availability::Available;
                let active = available && theme_files::studio_option_active(&file, &option.id, &values);
                obj.insert("active".into(), json!(active));
                obj.insert("available".into(), json!(available));
                let reason = match availability {
                    crate::theme_options::Availability::Unavailable(reason) => Some(crate::i18n::tr(reason).to_owned()),
                    _ if !active => Some("An ancestor option is off.".to_owned()),
                    _ => None,
                };
                obj.insert("reason".into(), json!(reason));
                if let Some(provider) = crate::theme_options::provider(&option.id, crate::theme_options::PROVIDERS) {
                    obj.insert("resolvedLabel".into(), json!(crate::i18n::tr(provider.label)));
                    obj.insert("resolvedHelp".into(), json!(crate::i18n::tr(provider.help)));
                } else {
                    obj.insert("resolvedLabel".into(), json!(crate::prefs::theme_option_text(&option.label, crate::i18n::language(), &option.id)));
                    obj.insert("resolvedHelp".into(), json!(crate::prefs::theme_option_text(&option.help, crate::i18n::language(), "")));
                }
            }
            definition
        })
        .collect::<Vec<_>>();
    json!({"ok":true,"errors":[],"definition":serde_json::from_str::<Value>(source).unwrap_or(Value::Null),
        "canonical":theme_files::export_file(&file),"definitions":definitions,"effectiveValues":values,
        "notices":notices.iter().map(|n|json!({"id":n.id,"kind":format!("{:?}",n.kind)})).collect::<Vec<_>>(),
        "needsNewer":file.needs_newer(),"requires":file.requires,"tab":tab,"vision":vision,
        "resolved":theme_files::export(&file.name,file.base,&tokens)})
}

/// Preserve the author document, including future fields, after the real importer validates it.
pub fn export_definition(source: &str) -> Value {
    match theme_files::parse(source) {
        Ok(file) => match serde_json::from_str::<Value>(source) {
            Ok(definition) => json!({"ok":true,"definition":definition,"canonical":theme_files::export_file(&file),"needsNewer":file.needs_newer()}),
            Err(error) => failure(error),
        },
        Err(error) => failure(error),
    }
}

/// Isolated scenario, palette environment and temporary values for one actual preview pane.
pub struct StudioPreview {
    file: ThemeFile,
    values: BTreeMap<String, OptionValue>,
    tab: String,
    vision: ColorVision,
    language: String,
    repaint: egui::Context,
    dirty_style: bool,
    environment: theme::StudioEnvironment,
    scene: String,
    focus: Option<String>,
    inspect: bool,
    inspection: Option<Value>,
    hover_inspection: Option<Value>,
    inspect_locked: bool,
    inspect_click: bool,
    hover_area: f32,
    native_revision: u64,
    name: String,
    distance: f64,
    checked: bool,
    selected: usize,
    tree_open: bool,
    tree_visible: bool,
    tree_selected: usize,
    history_selected: usize,
    playing: bool,
    modal_open: bool,
    icon_selected: usize,
    status: String,
    script_fonts: BTreeMap<String, Vec<u8>>,
    font_revision: u64,
    applied_font_revision: u64,
    applied_language: String,
}

impl StudioPreview {
    pub fn new(ctx: &egui::Context) -> Result<Self, String> {
        let file = theme_files::parse(ONYX_SOURCE)?;
        Ok(Self {
            file,
            values: BTreeMap::new(),
            tab: "MODEL".into(),
            vision: ColorVision::Off,
            language: "en-GB".into(),
            repaint: ctx.clone(),
            dirty_style: true,
            environment: Default::default(),
            scene: "all".into(),
            focus: None,
            inspect: false,
            inspection: None,
            hover_inspection: None,
            inspect_locked: false,
            inspect_click: false,
            hover_area: f32::INFINITY,
            native_revision: 0,
            name: "Mounting plate".into(),
            distance: 24.0,
            checked: true,
            selected: 0,
            tree_open: true,
            tree_visible: true,
            tree_selected: 1,
            history_selected: 1,
            playing: false,
            modal_open: false,
            icon_selected: 0,
            status: String::new(),
            script_fonts: BTreeMap::new(),
            font_revision: 0,
            applied_font_revision: u64::MAX,
            applied_language: String::new(),
        })
    }
    pub fn language(&self) -> &str {
        &self.language
    }
    pub fn set_preview(&mut self, source: &str, values: &str, tab: &str, vision: &str) -> Value {
        let _language = crate::i18n::scope(&self.language);
        let result = evaluate(source, values, tab, vision);
        if let Ok((file, values, vision)) = input(source, values, tab, vision) {
            self.file = file;
            self.values = values;
            self.tab = tab.into();
            self.vision = vision;
            self.dirty_style = true;
            self.repaint.request_repaint();
        }
        result
    }
    pub fn set_view(&mut self, config: &Value) -> Result<(), String> {
        let scene = config.get("scene").and_then(Value::as_str).unwrap_or(&self.scene);
        if !["all", "tools", "browser", "controls", "glass", "viewport", "icons"].contains(&scene) {
            return Err("Unknown preview scene".into());
        }
        let language = config.get("language").and_then(Value::as_str).unwrap_or(&self.language);
        if !crate::i18n::is_supported(language) {
            return Err("Unknown preview language".into());
        }
        let focus = config.get("focusScene").and_then(Value::as_str);
        if focus.is_some_and(|focus| !["tools", "browser", "controls", "glass", "viewport", "icons"].contains(&focus)) {
            return Err("Unknown focused preview scene".into());
        }
        if config.get("inspect").is_some_and(|value| !value.is_boolean()) {
            return Err("Inspect must be boolean".into());
        }
        self.scene = scene.into();
        self.language = language.into();
        if let Some(focus) = focus {
            self.focus = Some(focus.into());
        }
        if let Some(inspect) = config.get("inspect").and_then(Value::as_bool) {
            self.inspect = inspect;
            if !inspect {
                self.inspect_locked = false;
                self.inspection = None;
            }
        }
        self.dirty_style = true;
        self.repaint.request_repaint();
        Ok(())
    }
    pub fn comparison(&self, source: &str, values: &str, tab: &str, vision: &str) -> Result<Self, String> {
        let mut comparison = Self::new(&self.repaint)?;
        comparison.language = self.language.clone();
        comparison.scene = self.scene.clone();
        let result = comparison.set_preview(source, values, tab, vision);
        if result["ok"] != true {
            return Err(result["errors"].to_string());
        }
        Ok(comparison)
    }
    pub fn sync_interaction_from(&mut self, current: &Self) {
        self.name.clone_from(&current.name);
        self.distance = current.distance;
        self.checked = current.checked;
        self.selected = current.selected;
        self.tree_open = current.tree_open;
        self.tree_visible = current.tree_visible;
        self.tree_selected = current.tree_selected;
        self.history_selected = current.history_selected;
        self.playing = current.playing;
        self.icon_selected = current.icon_selected;
    }
    pub fn set_script_font(&mut self, language: &str, bytes: Vec<u8>) -> Result<(), String> {
        if !["ja", "zh", "zh-TW"].contains(&language) || bytes.len() > 32 * 1024 * 1024 {
            return Err("Unsupported script font or font exceeds32MiB".into());
        }
        if ttf_parser::Face::parse(&bytes, 0).is_err() {
            return Err("Script font is invalid".into());
        }
        if self.script_fonts.get(language).is_some_and(|current| current == &bytes) {
            return Ok(());
        }
        self.script_fonts.insert(language.into(), bytes);
        self.font_revision = self.font_revision.wrapping_add(1);
        self.repaint.request_repaint();
        Ok(())
    }
    pub fn prepare_input(&mut self, input: &mut egui::RawInput) {
        self.inspect_click = false;
        if self.inspect {
            self.inspect_click = input
                .events
                .iter()
                .any(|event| matches!(event, egui::Event::PointerButton { button: egui::PointerButton::Primary, pressed: false, .. }));
            input.events.retain(|event| {
                !matches!(event, egui::Event::PointerButton { .. } | egui::Event::Text(_) | egui::Event::Key { .. } | egui::Event::Ime(_))
            });
        }
    }
    pub fn radius(&self) -> f32 {
        let values = theme_files::effective_values(&self.file, &self.values).0;
        crate::theme_options::blur_radius(&self.file, &values, crate::theme_options::Availability::Available)
    }
    pub fn state(&self) -> Value {
        json!({"values":self.values,"effectiveValues":theme_files::effective_values(&self.file,&self.values).0,
            "tab":self.tab,"vision":self.vision,"language":self.language,"scene":self.scene,"inspection":self.inspection,
            "inspect":self.inspect,"status":self.status,"nativeRevision":self.native_revision,"fontCoverage":if self.language=="ko" {"The production web font pack does not yet include Hangul."} else {"native"}})
    }
    pub fn ui(&mut self, ui: &mut Ui) {
        let environment = self.environment.enter();
        let _language = crate::i18n::scope(&self.language);
        self.ui_bound(ui);
        self.environment = environment.finish();
    }
    fn ui_bound(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx().clone();
        if self.applied_language != self.language || self.applied_font_revision != self.font_revision {
            let cjk = self.script_fonts.get("ja").cloned();
            let chinese = self.script_fonts.get(if self.language == "zh-TW" { "zh-TW" } else { "zh" }).cloned();
            crate::fonts::install_web_script_fonts(&ctx, cjk, chinese, &self.language);
            self.applied_language = self.language.clone();
            self.applied_font_revision = self.font_revision;
        }
        theme::set_dark(self.file.base.is_dark());
        theme::set_vision(self.vision);
        let raw = self.values.iter().filter_map(|(k, v)| serde_json::to_value(v).ok().map(|v| (k.clone(), v))).collect::<BTreeMap<_, _>>();
        let custom = theme::prepare_tabs(&ctx, Some(&self.file), Some(&raw), || theme_files::effective_values(&self.file, &self.values).0);
        if theme::set_custom(custom) || self.dirty_style {
            theme::apply(&ctx);
            self.dirty_style = false;
        }
        theme::TabAccent::publish_preview(&ctx, &self.tab);
        let t = Tokens::get();
        let bounds = ui.max_rect();
        let mut gradient = egui::Mesh::default();
        gradient.colored_vertex(bounds.left_top(), t.viewport_top);
        gradient.colored_vertex(bounds.right_top(), t.viewport_top);
        gradient.colored_vertex(bounds.right_bottom(), t.viewport_bottom);
        gradient.colored_vertex(bounds.left_bottom(), t.viewport_bottom);
        gradient.add_triangle(0, 1, 2);
        gradient.add_triangle(0, 2, 3);
        ui.painter().add(gradient);
        self.hover_inspection = None;
        self.hover_area = f32::INFINITY;
        egui::ScrollArea::vertical().id_salt("studio_scene_board").auto_shrink([false, false]).show(ui, |ui| {
            let ids = ["tools", "browser", "controls", "glass", "viewport", "icons"];
            if self.scene != "all" {
                let scene = self.scene.clone();
                self.scene_card(ui, &scene);
            } else if ui.available_width() >= 840.0 {
                for pair in ids.chunks(2) {
                    ui.columns(2, |columns| {
                        for (column, id) in columns.iter_mut().zip(pair) {
                            self.scene_card(column, id);
                        }
                    });
                    ui.add_space(t.space_12);
                }
            } else {
                for id in ids {
                    self.scene_card(ui, id);
                    ui.add_space(t.space_12);
                }
            }
        });
        if self.inspect_click {
            if let Some(mut inspection) = self.hover_inspection.clone() {
                inspection["locked"] = json!(true);
                self.inspection = Some(inspection);
                self.inspect_locked = true;
            } else {
                self.inspect_locked = false;
                self.inspection = None;
            }
        } else if !self.inspect_locked {
            self.inspection = self.hover_inspection.clone();
        }
        if self.modal_open {
            let host = Container::new(&ctx, Surface::Window);
            let frame = host.frame(&ctx, t.dialog_body, crate::frame::Side::Left).inner_margin(t.space_16);
            let shown = host.modal(egui::Id::new("studio_modal")).frame(frame).show(&ctx, |ui| {
                ui.set_width(300.0);
                ui_kit::scope(ui, |ui| {
                    ui.label(egui::RichText::new("Review change").font(crate::fonts::semibold(t.font_heading)));
                    ui.label("This is the production modal over the current preview.");
                    let close = ui_kit::button(ui, "Close", ui_kit::Button::Primary);
                    self.record(ui, close.rect, "Modal close", &["form_primary_fill", "on_accent"], "glass");
                    close.clicked()
                })
            });
            if shown.inner || shown.should_close() {
                self.modal_open = false;
            }
        }
    }
    fn record(&mut self, ui: &Ui, rect: Rect, label: &str, roles: &[&str], scene: &str) {
        if !self.inspect {
            return;
        }
        let hovered = ui.rect_contains_pointer(rect);
        if hovered && rect.area() <= self.hover_area {
            self.hover_area = rect.area();
            self.hover_inspection = Some(json!({"label":label,"roles":roles,"scene":scene,"locked":false}));
        }
        let locked = self.inspect_locked && self.inspection.as_ref().is_some_and(|v| v["label"] == label && v["scene"] == scene);
        if hovered || locked {
            ui.painter().rect_stroke(
                rect.expand(2.0),
                Tokens::get().radius_sm,
                egui::Stroke::new(1.5, Tokens::get().accent),
                egui::StrokeKind::Outside,
            );
        }
    }
}
