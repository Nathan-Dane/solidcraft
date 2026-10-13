//! Design review export from the application's real drawing masters and palette tokens.
//! CPU rasterisation is only for this tool; shipped UI rendering is unchanged.
use egui::{Color32, Pos2, Rect, TextureId, pos2, vec2};
use solidcraft_ui_egui::{icons, theme::Tokens};

fn cross(a: egui::Vec2, b: egui::Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let columns = 12;
    let names: Vec<_> = icons::IDS.iter().chain(icons::FALLBACK_IDS.iter()).copied().collect();
    let rows = names.len().div_ceil(columns);
    let (width, height) = (2400u32, (rows * 86 + 48) as u32);
    let ctx = egui::Context::default();
    let draw = |ui: &mut egui::Ui| {
        let p = ui.painter();
        for (side, t) in [Tokens::light(), Tokens::dark()].into_iter().enumerate() {
            let x = side as f32 * 1200.0;
            p.rect_filled(Rect::from_min_size(pos2(x, 0.0), vec2(1200.0, height as f32)), 0.0, t.toolbar);
            p.text(pos2(x + 12.0, 12.0), egui::Align2::LEFT_TOP, if side == 0 { "Light" } else { "Dark" }, egui::FontId::proportional(18.0), t.text);
            for (i, name) in names.iter().enumerate() {
                let c = pos2(x + (i % columns) as f32 * 100.0 + 50.0, 64.0 + (i / columns) as f32 * 86.0);
                icons::paint(p, Rect::from_center_size(c, vec2(32.0, 32.0)), name, t.icon, t.icon_fill, t.accent);
                p.text(c + vec2(0.0, 22.0), egui::Align2::CENTER_TOP, name, egui::FontId::proportional(10.0), t.text);
                if icons::FALLBACK_IDS.contains(name) {
                    p.text(c + vec2(0.0, 35.0), egui::Align2::CENTER_TOP, "fallback", egui::FontId::proportional(9.0), t.text_dim);
                }
            }
        }
    };
    let input = || egui::RawInput { screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(width as f32, height as f32))), ..Default::default() };
    // Layout a warm-up frame so all font glyphs are available in the final atlas.
    let _ = ctx.run_ui(input(), draw);
    let out = ctx.run_ui(input(), draw);
    let atlas = ctx.fonts(|fonts| fonts.image());
    let mut image = image::RgbaImage::from_pixel(width, height, image::Rgba([255, 255, 255, 255]));
    for primitive in ctx.tessellate(out.shapes, 1.0) {
        let egui::epaint::Primitive::Mesh(mesh) = primitive.primitive else { continue };
        if mesh.texture_id != TextureId::default() {
            return Err("review sheet uses an unexpected texture".into());
        }
        for triangle in mesh.indices.as_chunks::<3>().0 {
            let Some(a) = triangle.first().and_then(|i| mesh.vertices.get(*i as usize)) else { continue };
            let Some(b) = triangle.get(1).and_then(|i| mesh.vertices.get(*i as usize)) else { continue };
            let Some(c) = triangle.get(2).and_then(|i| mesh.vertices.get(*i as usize)) else { continue };
            let area = cross(b.pos - a.pos, c.pos - a.pos);
            if area.abs() < 1e-6 {
                continue;
            }
            let bounds = Rect::from_min_max(a.pos.min(b.pos).min(c.pos), a.pos.max(b.pos).max(c.pos))
                .intersect(primitive.clip_rect)
                .intersect(Rect::from_min_size(Pos2::ZERO, vec2(width as f32, height as f32)));
            for y in (bounds.top().floor().max(0.0) as u32)..(bounds.bottom().ceil().min(height as f32) as u32) {
                for x in (bounds.left().floor().max(0.0) as u32)..(bounds.right().ceil().min(width as f32) as u32) {
                    let q = pos2(x as f32 + 0.5, y as f32 + 0.5);
                    let w = [cross(b.pos - q, c.pos - q) / area, cross(c.pos - q, a.pos - q) / area, cross(a.pos - q, b.pos - q) / area];
                    if w.iter().any(|v| *v < 0.0) {
                        continue;
                    }
                    let uv = a.uv.to_vec2() * w[0] + b.uv.to_vec2() * w[1] + c.uv.to_vec2() * w[2];
                    let tx = (uv.x * atlas.size[0] as f32).floor().clamp(0.0, atlas.size[0].saturating_sub(1) as f32) as usize;
                    let ty = (uv.y * atlas.size[1] as f32).floor().clamp(0.0, atlas.size[1].saturating_sub(1) as f32) as usize;
                    let texel = atlas.pixels.get(ty * atlas.size[0] + tx).copied().unwrap_or(Color32::TRANSPARENT).to_array();
                    let colors = [a.color.to_array(), b.color.to_array(), c.color.to_array()];
                    let mut source = [0.0; 4];
                    for (i, v) in source.iter_mut().enumerate() {
                        *v = (f32::from(colors[0][i]) * w[0] + f32::from(colors[1][i]) * w[1] + f32::from(colors[2][i]) * w[2]) * f32::from(texel[i])
                            / 255.0;
                    }
                    let dst = image.get_pixel_mut(x, y);
                    for (dest, src) in dst.0.iter_mut().take(3).zip(source.iter()) {
                        *dest = (*src + f32::from(*dest) * (1.0 - source[3] / 255.0)).round().clamp(0.0, 255.0) as u8;
                    }
                }
            }
        }
    }
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/resources/ui-icons.png");
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    image.save(&path)?;
    println!("{}", path.display());
    Ok(())
}
