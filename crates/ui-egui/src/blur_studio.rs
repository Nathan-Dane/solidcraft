//! A studio-only pass owner for the production ordered backdrop compositor.
//!
//! The inner egui context paints ordinary background primitives first, then real UI-kit
//! hosts. Unlike the CAD source adapter, this host needs no document or viewport renderer.
//! The shared compositor still samples the completed preceding UI at every marker boundary.
use super::*;

/// A completed GPU preview, registered with the outer eframe renderer.
pub struct StudioFrame {
    pub texture: egui::TextureId,
    pub size_in_pixels: [u32; 2],
    pub counters: Counters,
    pub platform_output: egui::PlatformOutput,
    pub repaint_after: std::time::Duration,
}

/// Owns a separate egui renderer, sharing the browser's device and queue with eframe.
/// It never installs a compositor on eframe's ordinary canvas renderer.
pub struct StudioRenderer {
    context: egui::Context,
    render_state: egui_wgpu::RenderState,
    outer: Arc<egui::epaint::mutex::RwLock<egui_wgpu::Renderer>>,
    state: State,
    target: Option<Target>,
    texture: Option<egui::TextureId>,
    presented_texture: Option<egui::TextureId>,
    prepared: Option<([u32; 2], u32)>,
    pending_gpu: Option<Gpu>,
}

/// Browser error scopes resolve asynchronously. Do not report validated resources until
/// all three scopes have actually completed; the native synchronous helper cannot do this.
async fn checked<T>(device: &wgpu::Device, create: impl FnOnce() -> T) -> Result<T, String> {
    let scopes =
        [wgpu::ErrorFilter::OutOfMemory, wgpu::ErrorFilter::Internal, wgpu::ErrorFilter::Validation].map(|filter| device.push_error_scope(filter));
    let value = create();
    // pop() changes the device-wide stack immediately. Finish every pop before
    // yielding, so another preview or eframe cannot interleave scopes on this device.
    let pending: Vec<_> = scopes.into_iter().rev().map(|scope| scope.pop()).collect();
    let mut failure = None;
    for result in pending {
        if let Some(error) = result.await {
            failure.get_or_insert_with(|| error.to_string());
        }
    }
    match failure {
        Some(error) => Err(format!("GPU refused the studio preview: {error}")),
        None => Ok(value),
    }
}

impl StudioRenderer {
    /// Initialize once after eframe supplies its real render state. No CAD engine is started.
    pub async fn new(outer: &egui_wgpu::RenderState) -> Result<Self, String> {
        let format = wgpu::TextureFormat::Rgba8Unorm;
        let renderer = checked(&outer.device, || egui_wgpu::Renderer::new(&outer.device, format, egui_wgpu::RendererOptions::default())).await?;
        let mut render_state = outer.clone();
        render_state.renderer = Arc::new(egui::epaint::mutex::RwLock::new(renderer));
        render_state.target_format = format;
        render_state.msaa_samples = 1;
        let mut state = State::default();
        // This pass owner supplies a single ordinary-egui backdrop instead of a CAD split
        // viewport. Its offscreen composition target is 1x, without a depth attachment.
        state.install(&render_state, true);
        let context = egui::Context::default();
        crate::fonts::setup_fonts(&context);
        Ok(Self {
            context,
            render_state,
            outer: outer.renderer.clone(),
            state,
            target: None,
            texture: None,
            presented_texture: None,
            prepared: None,
            pending_gpu: None,
        })
    }

    pub fn context(&self) -> &egui::Context {
        &self.context
    }

    pub fn capability(&self) -> Capability {
        self.state.capability()
    }

    /// Call when physical dimensions or requested radius change, before rendering again.
    /// Keep the last completed image visible while this asynchronous validation is pending.
    pub async fn prepare(&mut self, size: [u32; 2], radius: f32) -> Result<(), String> {
        validate_radius(radius).map_err(str::to_owned)?;
        if self.prepared == Some((size, radius.to_bits())) {
            return Ok(());
        }
        self.prepared = None;
        if self.render_state.device_lost.is_lost() {
            self.state.retire();
            self.pending_gpu = None;
            return Err("studio graphics device lost".into());
        }
        let max = self.render_state.device.limits().max_texture_dimension_2d;
        let steps = schedule(radius);
        if target_bytes(size, steps.map_or(2, |s| s.0), max).is_none() {
            return Err("studio preview exceeds texture dimensions or memory budget".into());
        }
        self.state.set_radius(radius)?;
        if radius > 0.0
            && let Capability::Unavailable(reason) = self.state.base_capability()
        {
            return Err(reason.into());
        }
        let mut counts = self.state.retired;
        counts.clear_live();
        let (target, gpu) = checked(&self.render_state.device, || {
            // The presentation image belongs to the preview host, not to the blur effect's
            // off-path resource counters. Both use the same gamma-space output as egui.
            let target = Target::new(&self.render_state.device, size, self.render_state.target_format, &mut Counters::default());
            let gpu = steps.map(|(steps, offset)| {
                let mut gpu = Gpu::new(&self.render_state.device, self.render_state.target_format, size, steps, offset, &mut counts);
                // Allocate under the awaited error scopes, never lazily during a draw.
                gpu.grow_markers(&self.render_state.device, MAX_MARKERS, &mut counts);
                gpu
            });
            (target, gpu)
        })
        .await?;
        if let Some((steps, _)) = steps {
            counts.live_bytes = target_bytes(size, steps, max).unwrap_or(0);
        }
        self.state.retire();
        self.state.retired = counts;
        self.pending_gpu = gpu;
        let texture = self.outer.write().register_native_texture(&self.render_state.device, &target.view, wgpu::FilterMode::Nearest);
        if let Some(previous) = self.texture.replace(texture)
            && Some(previous) != self.presented_texture
        {
            self.outer.write().free_texture(&previous);
        }
        self.target = Some(target);
        self.prepared = Some((size, radius.to_bits()));
        Ok(())
    }

    /// Run actual UI-kit scenes inside the inner context, then submit their real GPU image.
    /// The caller forwards canvas-local egui events and the actual device scale in RawInput.
    /// Draw the viewport/background as ordinary primitives before translucent UI hosts.
    pub fn render(&mut self, input: egui::RawInput, mut draw: impl FnMut(&mut egui::Ui)) -> Result<StudioFrame, String> {
        let Some((size, _)) = self.prepared else { return Err("studio preview resources are not prepared".into()) };
        if self.render_state.device_lost.is_lost() {
            self.prepared = None;
            self.state.retire();
            self.pending_gpu = None;
            return Err("studio graphics device lost".into());
        }
        let Some(target) = &self.target else { return Err("studio preview has no output target".into()) };
        let Some(texture) = self.texture else { return Err("studio preview has no registered image".into()) };
        let output = self.context.run_ui(input, |ui| {
            let _scope = self.state.begin(ui.ctx(), true);
            draw(ui);
        });
        let repaint_after = output.viewport_output.get(&egui::ViewportId::ROOT).map_or(std::time::Duration::MAX, |viewport| viewport.repaint_delay);
        let screen = ScreenDescriptor { size_in_pixels: size, pixels_per_point: output.pixels_per_point };
        let jobs = self.context.tessellate(output.shapes, output.pixels_per_point);
        let shared = self.state.active.clone();
        if let Some(shared) = &shared {
            let mut inner = shared.inner.lock().map_err(|_| "studio blur state unavailable")?;
            if let Some(gpu) = self.pending_gpu.take() {
                inner.gpu = Some(gpu);
            }
            let serial = inner.serial;
            let Some(gpu) = &mut inner.gpu else { return Err("studio blur resources changed; prepare the preview again".into()) };
            if gpu.size != size {
                return Err("studio blur dimensions changed; prepare the preview again".into());
            }
            gpu.ready_serial = serial;
            // This host's source is the ordinary ordered primitive stream itself, not a
            // ViewportCallback. render() paints it before sampling any backdrop boundary.
            inner.source_seen = true;
        }
        let mut renderer = self.render_state.renderer.write();
        for (id, deltas) in &output.textures_delta.set {
            for delta in deltas {
                renderer.update_texture(&self.render_state.device, &self.render_state.queue, *id, delta);
            }
        }
        let mut encoder = self.render_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("studio ordered preview") });
        let buffers = renderer.update_buffers(&self.render_state.device, &self.render_state.queue, &mut encoder, &jobs, &screen);
        let composed =
            shared.as_ref().is_some_and(|effect| effect.render(&renderer, &mut encoder, &target.view, &jobs, &screen, [0.0, 0.0, 0.0, 1.0]));
        if !composed {
            let mut pass = pass(&mut encoder, "studio ordinary preview", &target.view, wgpu::Color::BLACK);
            renderer.render(&mut pass, &jobs, &screen);
        }
        self.render_state.queue.submit(buffers.into_iter().chain([encoder.finish()]));
        if let Some(shared) = &shared {
            shared.submitted();
        }
        for id in &output.textures_delta.free {
            renderer.free_texture(id);
        }
        if self.state.radius > 0.0 && !composed {
            return Err("studio blur compositor could not complete this frame".into());
        }
        // Preserve the last complete image across asynchronous preparation and failed
        // draws. Retire it only when the replacement has actually been submitted.
        if let Some(previous) = self.presented_texture.replace(texture)
            && previous != texture
        {
            self.outer.write().free_texture(&previous);
        }
        Ok(StudioFrame { texture, size_in_pixels: size, counters: self.state.counters(), platform_output: output.platform_output, repaint_after })
    }
}

impl Drop for StudioRenderer {
    fn drop(&mut self) {
        self.state.retire();
        if let Some(texture) = self.texture.take() {
            self.outer.write().free_texture(&texture);
            if self.presented_texture == Some(texture) {
                self.presented_texture = None;
            }
        }
        if let Some(texture) = self.presented_texture.take() {
            self.outer.write().free_texture(&texture);
        }
    }
}
