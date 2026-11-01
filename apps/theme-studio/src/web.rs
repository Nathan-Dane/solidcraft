use eframe::{egui, egui_wgpu::RenderState};
use serde_json::{Value, json};
use solidcraft_ui_egui::{
    blur::StudioRenderer,
    theme_studio::{self, StudioPreview},
};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wasm_bindgen::{JsCast, prelude::*};

type Preview = Rc<RefCell<StudioPreview>>;
thread_local! {
    static PREVIEW:RefCell<Option<Preview>>=const{RefCell::new(None)};
    static COMPARISON:RefCell<Option<Preview>>=const{RefCell::new(None)};
    static STATUS:RefCell<Value>=const{RefCell::new(Value::Null)};
    static BASELINE_FOCUSED:Cell<bool>=const{Cell::new(false)};
    static RUNNER:RefCell<Option<eframe::WebRunner>>=const{RefCell::new(None)};
}
fn current() -> Option<Preview> {
    PREVIEW.with(|slot| slot.try_borrow().ok().and_then(|slot| slot.clone()))
}
fn baseline() -> Option<Preview> {
    COMPARISON.with(|slot| slot.try_borrow().ok().and_then(|slot| slot.clone()))
}
fn language() -> String {
    current().and_then(|preview| preview.try_borrow().ok().map(|preview| preview.language().to_owned())).unwrap_or_else(|| "en-GB".into())
}
fn error(error: impl ToString) -> String {
    json!({"ok":false,"errors":[error.to_string()]}).to_string()
}

#[wasm_bindgen]
pub fn contract() -> String {
    theme_studio::contract().to_string()
}
#[wasm_bindgen]
pub fn evaluate(source: &str, values: &str, tab: &str, vision: &str) -> String {
    let language = language();
    let _scope = solidcraft_ui_egui::i18n::scope(&language);
    theme_studio::evaluate(source, values, tab, vision).to_string()
}
#[wasm_bindgen(js_name=exportDefinition)]
pub fn export_definition(source: &str) -> String {
    theme_studio::export_definition(source).to_string()
}
#[wasm_bindgen(js_name=setPreview)]
pub fn set_preview(source: &str, values: &str, tab: &str, vision: &str) -> String {
    let Some(preview) = current() else { return error("Preview is not ready") };
    match preview.try_borrow_mut() {
        Ok(mut preview) => preview.set_preview(source, values, tab, vision).to_string(),
        Err(_) => error("Preview is drawing; retry next animation frame"),
    }
}
#[wasm_bindgen(js_name=setView)]
pub fn set_view(config: &str) -> String {
    let result = (|| -> Result<(), String> {
        if config.len() > 3 * 256 * 1024 {
            return Err("View configuration exceeds input limit".into());
        }
        let config: Value = serde_json::from_str(config).map_err(|e| e.to_string())?;
        if !config.is_object() {
            return Err("View configuration must be an object".into());
        }
        let current = current().ok_or("Preview is not ready")?;
        let candidate: Option<Option<Preview>> = match config.get("comparison") {
            None => None,
            Some(Value::Null) => Some(None),
            Some(value) => {
                let source = value.get("source").and_then(Value::as_str).ok_or("Comparison requires its authored source JSON")?;
                let values =
                    value.get("values").map(|v| v.as_str().map(str::to_owned).unwrap_or_else(|| v.to_string())).unwrap_or_else(|| "{}".into());
                let tab = value.get("tab").and_then(Value::as_str).unwrap_or("MODEL");
                let vision = value.get("vision").and_then(Value::as_str).unwrap_or("off");
                let mut preview = current.try_borrow().map_err(|_| "Preview is drawing")?.comparison(source, &values, tab, vision)?;
                preview.set_view(&config)?;
                Some(Some(Rc::new(RefCell::new(preview))))
            }
        };
        current.try_borrow_mut().map_err(|_| "Preview is drawing")?.set_view(&config)?;
        if let Some(candidate) = candidate {
            COMPARISON.with(|slot| {
                if let Ok(mut slot) = slot.try_borrow_mut() {
                    *slot = candidate;
                }
            });
        } else if let Some(baseline) = baseline() {
            baseline.try_borrow_mut().map_err(|_| "Comparison is drawing")?.set_view(&config)?;
        }
        Ok(())
    })();
    match result {
        Ok(()) => json!({"ok":true}).to_string(),
        Err(e) => error(e),
    }
}
#[wasm_bindgen(js_name=setScriptFont)]
pub fn set_script_font(language: &str, bytes: &[u8]) -> String {
    let result = (|| -> Result<(), String> {
        let preview = current().ok_or("Preview is not ready")?;
        preview.try_borrow_mut().map_err(|_| "Preview is drawing")?.set_script_font(language, bytes.to_vec())?;
        if let Some(baseline) = baseline() {
            baseline.try_borrow_mut().map_err(|_| "Comparison is drawing")?.set_script_font(language, bytes.to_vec())?;
        }
        Ok(())
    })();
    match result {
        Ok(()) => json!({"ok":true}).to_string(),
        Err(e) => error(e),
    }
}
#[wasm_bindgen]
pub fn state() -> String {
    let runtime_error = RUNNER.with(|runner| {
        runner.try_borrow().ok().and_then(|runner| runner.as_ref().and_then(eframe::WebRunner::panic_summary)).map(|summary| summary.message())
    });
    let mut preview = current().and_then(|preview| preview.try_borrow().ok().map(|preview| preview.state()));
    let comparison = baseline().and_then(|preview| preview.try_borrow().ok().map(|preview| preview.state()));
    if let (Some(preview), Some(comparison)) = (&mut preview, &comparison)
        && !comparison["inspection"].is_null()
        && (BASELINE_FOCUSED.get() || preview["inspection"].is_null())
    {
        let mut inspection = comparison["inspection"].clone();
        inspection["baseline"] = json!(true);
        preview["inspection"] = inspection;
    }
    STATUS.with(|status| {
        json!({"preview":preview,"comparison":comparison,"renderer":status.try_borrow().ok().as_deref(),"runtimeError":runtime_error}).to_string()
    })
}

struct GpuState {
    renderer: Option<StudioRenderer>,
    pending: bool,
    prepared: Option<([u32; 2], u32)>,
    error: Option<String>,
    effective_radius: f32,
}
struct Pane {
    preview: Preview,
    gpu: Rc<RefCell<GpuState>>,
    last_texture: Option<egui::TextureId>,
}
impl Pane {
    fn new(preview: Preview, state: &RenderState, ctx: &egui::Context) -> Self {
        let gpu = Rc::new(RefCell::new(GpuState { renderer: None, pending: true, prepared: None, error: None, effective_radius: 0.0 }));
        let shared = gpu.clone();
        let ctx = ctx.clone();
        let state = state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let result = StudioRenderer::new(&state).await;
            if let Ok(mut gpu) = shared.try_borrow_mut() {
                gpu.pending = false;
                match result {
                    Ok(renderer) => gpu.renderer = Some(renderer),
                    Err(error) => gpu.error = Some(error),
                }
            }
            ctx.request_repaint();
        });
        Self { preview, gpu, last_texture: None }
    }
    fn render(&mut self, ui: &mut egui::Ui, bounds: egui::Rect, focused: bool) -> Value {
        let ppp = ui.ctx().pixels_per_point();
        let size = [(bounds.width() * ppp).round().max(1.0) as u32, (bounds.height() * ppp).round().max(1.0) as u32];
        let radius = self.preview.try_borrow().map(|p| p.radius()).unwrap_or(0.0);
        let desired = (size, radius.to_bits());
        let mut prepare = None;
        if let Ok(mut gpu) = self.gpu.try_borrow_mut() {
            if !gpu.pending
                && gpu.prepared != Some(desired)
                && let Some(renderer) = gpu.renderer.take()
            {
                gpu.pending = true;
                prepare = Some(renderer);
            }
        }
        if let Some(mut renderer) = prepare {
            let shared = self.gpu.clone();
            let ctx = ui.ctx().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let result = renderer.prepare(size, radius).await;
                let (error, effective_radius) = match result {
                    Ok(()) => (None, radius),
                    Err(error) => {
                        let fallback = renderer.prepare(size, 0.0).await;
                        (
                            Some(match fallback {
                                Ok(()) => error,
                                Err(fallback) => format!("{error}; {fallback}"),
                            }),
                            0.0,
                        )
                    }
                };
                if let Ok(mut gpu) = shared.try_borrow_mut() {
                    gpu.renderer = Some(renderer);
                    gpu.pending = false;
                    gpu.prepared = Some(desired);
                    gpu.error = error;
                    gpu.effective_radius = effective_radius;
                }
                ctx.request_repaint();
            });
        }
        let mut status = json!({"pending":true});
        if let Ok(mut gpu) = self.gpu.try_borrow_mut() {
            status = json!({"pending":gpu.pending,"error":gpu.error,"requestedRadius":radius,"effectiveRadius":gpu.effective_radius,
                "physicalSize":size,"pixelsPerPoint":ppp,"mode":if gpu.pending{"preparing"}else if gpu.error.is_some(){"unavailable"}else if radius==0.0{"off"}else{"available"}});
            if !gpu.pending
                && let Some(renderer) = gpu.renderer.as_mut()
            {
                let mut input = ui.ctx().input(|i| i.raw.clone());
                input.screen_rect = Some(egui::Rect::from_min_size(egui::Pos2::ZERO, bounds.size()));
                input.focused &= focused;
                for event in &mut input.events {
                    match event {
                        egui::Event::PointerMoved(pos) | egui::Event::PointerButton { pos, .. } | egui::Event::Touch { pos, .. } => {
                            *pos -= bounds.min.to_vec2()
                        }
                        _ => {}
                    }
                }
                if !focused {
                    input.events.retain(|event| !matches!(event, egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Ime(_)));
                }
                if let Ok(mut preview) = self.preview.try_borrow_mut() {
                    preview.prepare_input(&mut input);
                }
                let preview = self.preview.clone();
                match renderer.render(input, |ui| {
                    if let Ok(mut preview) = preview.try_borrow_mut() {
                        preview.ui(ui);
                    }
                }) {
                    Ok(frame) => {
                        self.last_texture = Some(frame.texture);
                        ui.ctx().request_repaint_after(frame.repaint_after);
                        if focused {
                            ui.ctx().output_mut(|out| out.append(frame.platform_output));
                        } else if ui.ctx().pointer_hover_pos().is_some_and(|pos| bounds.contains(pos)) {
                            ui.ctx().set_cursor_icon(frame.platform_output.cursor_icon);
                        }
                        status["counters"] = json!(frame.counters);
                    }
                    Err(error) => {
                        status["error"] = json!(error);
                        status["mode"] = json!("unavailable");
                    }
                }
            }
            if let Some(texture) = self.last_texture {
                ui.painter().image(texture, bounds, egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
            } else {
                ui.painter().text(
                    bounds.center(),
                    egui::Align2::CENTER_CENTER,
                    gpu.error.as_deref().unwrap_or("Preparing the actual SolidCraft preview…"),
                    egui::FontId::proportional(13.0),
                    egui::Color32::LIGHT_GRAY,
                );
            }
        }
        status
    }
}
struct App {
    current: Pane,
    comparison: Option<Pane>,
    render_state: RenderState,
    canvas: web_sys::HtmlCanvasElement,
}

/// eframe 0.36 uses ResizeObserver.devicePixelContentBoxSize for the backing size,
/// but pointer events use the displayed CSS rectangle. Some browser hosts report
/// a different device-pixel box even at DPR 1. Keep the borderless studio canvas
/// at its actual displayed size times DPR, so native layout, artwork and input all
/// use the same scale. No pointer correction is necessary in the normal path.
fn normalize_canvas_size(canvas: &web_sys::HtmlCanvasElement) -> bool {
    let rect = canvas.get_bounding_client_rect();
    let dpr = web_sys::window().map_or(1.0, |window| window.device_pixel_ratio());
    let width = rect.width() * dpr;
    let height = rect.height() * dpr;
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return false;
    }
    let width = width.round().max(1.0) as u32;
    let height = height.round().max(1.0) as u32;
    if canvas.width() == width && canvas.height() == height {
        return false;
    }
    canvas.set_width(width);
    canvas.set_height(height);
    true
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, canvas: web_sys::HtmlCanvasElement) -> Result<Self, String> {
        solidcraft_ui_egui::fonts::setup_fonts(&cc.egui_ctx);
        let preview = Rc::new(RefCell::new(StudioPreview::new(&cc.egui_ctx)?));
        PREVIEW.with(|slot| {
            if let Ok(mut slot) = slot.try_borrow_mut() {
                *slot = Some(preview.clone());
            }
        });
        let render_state = cc.wgpu_render_state.clone().ok_or("Theme Studio requires its real WebGPU renderer.")?;
        Ok(Self { current: Pane::new(preview, &render_state, &cc.egui_ctx), comparison: None, render_state, canvas })
    }
}
impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if normalize_canvas_size(&self.canvas) {
            // This frame's RawInput was gathered before the backing-size correction.
            // Repaint with eframe's newly matching screen rectangle before accepting input.
            ui.ctx().request_repaint();
            return;
        }
        let baseline = baseline();
        match baseline {
            Some(preview) if self.comparison.as_ref().is_none_or(|pane| !Rc::ptr_eq(&pane.preview, &preview)) => {
                self.comparison = Some(Pane::new(preview, &self.render_state, ui.ctx()))
            }
            None => {
                self.comparison = None;
                BASELINE_FOCUSED.set(false);
            }
            _ => {}
        }
        let bounds = ui.max_rect();
        let mut current_bounds = bounds;
        let mut comparison_bounds = None;
        if self.comparison.is_some() {
            let gap = 8.0;
            if bounds.width() >= 720.0 {
                let half = (bounds.width() - gap) / 2.0;
                current_bounds.max.x = current_bounds.min.x + half;
                comparison_bounds = Some(egui::Rect::from_min_max(egui::pos2(current_bounds.right() + gap, bounds.top()), bounds.max));
            } else {
                let half = (bounds.height() - gap) / 2.0;
                current_bounds.max.y = current_bounds.min.y + half;
                comparison_bounds = Some(egui::Rect::from_min_max(egui::pos2(bounds.left(), current_bounds.bottom() + gap), bounds.max));
            }
            ui.painter().text(
                current_bounds.left_top() + egui::vec2(4.0, 4.0),
                egui::Align2::LEFT_TOP,
                "Current",
                egui::FontId::proportional(12.0),
                egui::Color32::LIGHT_GRAY,
            );
            current_bounds.min.y += 24.0;
            if let Some(rect) = &mut comparison_bounds {
                ui.painter().text(
                    rect.left_top() + egui::vec2(4.0, 4.0),
                    egui::Align2::LEFT_TOP,
                    "Pinned baseline",
                    egui::FontId::proportional(12.0),
                    egui::Color32::LIGHT_GRAY,
                );
                rect.min.y += 24.0;
            }
        }
        if ui.input(|i| i.pointer.primary_pressed())
            && let Some(pointer) = ui.ctx().pointer_hover_pos()
        {
            BASELINE_FOCUSED.set(comparison_bounds.is_some_and(|rect| rect.contains(pointer)));
        }
        if BASELINE_FOCUSED.get()
            && let Some(comparison) = &self.comparison
            && let (Ok(mut current), Ok(baseline)) = (self.current.preview.try_borrow_mut(), comparison.preview.try_borrow())
        {
            current.sync_interaction_from(&baseline);
        }
        let mut status = self.current.render(ui, current_bounds, !BASELINE_FOCUSED.get());
        if let (Some(comparison), Some(bounds)) = (&mut self.comparison, comparison_bounds) {
            if !BASELINE_FOCUSED.get()
                && let (Ok(current), Ok(mut baseline)) = (self.current.preview.try_borrow(), comparison.preview.try_borrow_mut())
            {
                baseline.sync_interaction_from(&current);
            }
            status["comparison"] = comparison.render(ui, bounds, BASELINE_FOCUSED.get());
        }
        status["sourceVersion"] = json!(option_env!("SOLIDCRAFT_THEME_STUDIO_REVISION").unwrap_or("development"));
        STATUS.with(|slot| {
            if let Ok(mut slot) = slot.try_borrow_mut() {
                *slot = status;
            }
        });
    }
    fn persist_egui_memory(&self) -> bool {
        false
    }
}
#[wasm_bindgen]
pub async fn start(canvas_id: &str) -> Result<(), JsValue> {
    let canvas = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(canvas_id))
        .ok_or_else(|| JsValue::from_str("Theme Studio preview canvas is missing."))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    normalize_canvas_size(&canvas);
    let app_canvas = canvas.clone();
    let runner = eframe::WebRunner::new();
    RUNNER.with(|slot| {
        if let Ok(mut slot) = slot.try_borrow_mut() {
            *slot = Some(runner.clone());
        }
    });
    runner
        .start(
            canvas,
            eframe::WebOptions::default(),
            Box::new(move |cc| App::new(cc, app_canvas).map(|app| Box::new(app) as Box<dyn eframe::App>).map_err(|e| e.into())),
        )
        .await
}
