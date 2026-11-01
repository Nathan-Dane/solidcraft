//! Analytic viewport token study, not a CAD model or a substitute for the scene renderer.
//! Geometry is deliberately simple. Grid spacing, dimension layout, marker artwork and
//! navigation-cube facets are the production implementations rather than website facsimiles.
use crate::theme::Tokens;
use egui::{Color32, FontId, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, pos2, vec2};
use solidcraft_engine::geom::Vec2;
use solidcraft_engine::render::Camera;
use solidcraft_engine::sketch::{DimFrame, dim_layout};

pub(crate) fn ui(ui: &mut Ui) -> Response {
    with_camera(ui, &Camera::default())
}

pub(crate) fn with_camera(ui: &mut Ui, camera: &Camera) -> Response {
    let width = ui.available_width().max(1.0);
    let (rect, response) = ui.allocate_exact_size(vec2(width, 280.0), Sense::hover());
    if !ui.is_rect_visible(rect) {
        return response;
    }
    let t = Tokens::view();
    let painter = ui.painter_at(rect);
    let mut background = egui::Mesh::default();
    for (position, color) in [
        (rect.left_top(), t.viewport_top),
        (rect.right_top(), t.viewport_top),
        (rect.right_bottom(), t.viewport_bottom),
        (rect.left_bottom(), t.viewport_bottom),
    ] {
        background.colored_vertex(position, color);
    }
    background.add_triangle(0, 1, 2);
    background.add_triangle(0, 2, 3);
    painter.add(Shape::mesh(background));
    let study = Rect::from_min_max(rect.min + vec2(16.0, 16.0), rect.max - vec2(16.0, 16.0));
    if !study.is_positive() {
        return response;
    }
    let scale = (study.width() / 95.0).min(study.height() / 75.0).max(0.1);
    let origin = pos2(study.left() + study.width() * 0.30, study.top() + study.height() * 0.68);
    let to = |point: Vec2| origin + vec2(point.x as f32, -point.y as f32) * scale;
    // Use the same grid-level spacing/cross-fade policy as the native viewport. This
    // flat token study intentionally omits the perspective scene's distance fade.
    for line in solidcraft_engine::view::grid_lines(37.5, 0.0, 100.0, 32) {
        let color = if line.major >= 0.5 { t.grid_major } else { t.grid }.gamma_multiply(line.weight);
        let stroke = Stroke::new(if line.major >= 0.5 { 1.2 } else { 0.8 }, color);
        let at = line.offset as f32 * scale;
        painter.line_segment([pos2(origin.x + at, study.top()), pos2(origin.x + at, study.bottom())], stroke);
        painter.line_segment([pos2(study.left(), origin.y - at), pos2(study.right(), origin.y - at)], stroke);
    }
    painter.line_segment([pos2(study.left(), origin.y), pos2(study.right(), origin.y)], Stroke::new(1.5, t.axis_x));
    painter.line_segment([pos2(origin.x, study.top()), pos2(origin.x, study.bottom())], Stroke::new(1.5, t.axis_y));
    let points = [Vec2::ZERO, Vec2::new(40.0, 0.0), Vec2::new(40.0, 25.0), Vec2::new(0.0, 25.0)].map(to);
    painter.add(Shape::convex_polygon(points.to_vec(), t.sel_profile, Stroke::new(1.5, t.sel_profile_edge)));
    for segment in [[points[0], points[1]], [points[1], points[2]], [points[2], points[3]], [points[3], points[0]]] {
        painter.line_segment(segment, Stroke::new(1.5, t.sketch));
    }
    // Show selection and hover as actual token overlays on explicit analytic edges.
    painter.line_segment([points[0], points[1]], Stroke::new(4.0, t.sel_edge_rim));
    painter.line_segment([points[0], points[1]], Stroke::new(2.0, t.sel_edge));
    if response.hovered() {
        painter.line_segment([points[1], points[2]], Stroke::new(4.0, t.hover_edge_halo));
        painter.line_segment([points[1], points[2]], Stroke::new(2.0, t.hover_edge));
    }
    painter.circle_stroke(to(Vec2::new(20.0, 12.5)), 7.0 * scale, Stroke::new(1.5, t.sketch_fixed));
    for (index, point) in points.into_iter().enumerate() {
        painter.circle_filled(point, 3.0, if index == 0 { t.sketch_point } else { t.sketch_point_free });
    }
    let construction = [to(Vec2::new(-8.0, 12.5)), to(Vec2::new(48.0, 12.5))];
    painter.extend(Shape::dashed_line(&construction, Stroke::new(1.0, t.sketch_construction), 6.0, 4.0));
    draw_dimension(&painter, &t, scale, to, t.dimension, -9.0, "40 mm");
    draw_dimension(&painter, &t, scale, to, t.dimension_driven, -20.0, "(40 mm)");
    // These are the production assembly snap markers, including their native halo.
    crate::icons::markers::snap(&painter, to(Vec2::new(20.0, 12.5)), None, crate::dialogs_assembly::SnapKind::CircleCenter, t.sketch_locked, None);
    crate::icons::markers::snap_hint(&painter, points[2], crate::dialogs_assembly::SnapKind::Vertex, t.sketch_conflict);
    let cube_size = (study.width() * 0.36).min(crate::viewport::navigation_cube::DESIGN_SIZE).min(study.height());
    let cube = Rect::from_min_size(pos2(study.right() - cube_size, study.top()), vec2(cube_size, cube_size));
    crate::viewport::navigation_cube::studio_cube(ui, cube, camera);
    response
}

fn draw_dimension(painter: &egui::Painter, t: &Tokens, scale: f32, to: impl Fn(Vec2) -> Pos2, color: Color32, offset: f64, label: &str) {
    let font = FontId::monospace(t.font_body);
    let galley = painter.layout_no_wrap(label.to_owned(), font, color);
    let size = galley.size() + vec2(6.0, 2.0);
    let px = 1.0 / f64::from(scale);
    let frame = DimFrame::Linear { p0: Vec2::ZERO, p1: Vec2::new(40.0, 0.0), dir: Vec2::X };
    let layout = dim_layout(&frame, Some(Vec2::new(0.0, offset)), px, Vec2::new(f64::from(size.x) * px / 2.0, f64::from(size.y) * px / 2.0));
    for line in layout.lines {
        painter.add(Shape::line(line.into_iter().map(&to).collect(), Stroke::new(1.0, color)));
    }
    for (tip, direction) in layout.arrows {
        crate::icons::markers::dimension_arrow(painter, to(tip), to(tip - direction * px), color);
    }
    let text = Rect::from_center_size(to(layout.text), size);
    painter.rect_filled(text, t.radius_sm / 2, t.viewport_top.gamma_multiply(0.85));
    painter.galley(text.min + vec2(3.0, 1.0), galley, color);
}
