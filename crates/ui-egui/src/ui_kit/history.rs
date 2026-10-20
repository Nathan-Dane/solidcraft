//! Shared history chips, dense name rows, drop bars and rollback markers.
use crate::{
    icons,
    theme::{TabAccent, Tokens},
};
use egui::{Painter, Rect, Stroke, pos2, vec2};

/// Browser and named history rows use the same density.
pub(crate) const TREE_ROW_HEIGHT: f32 = 26.0;

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum HistoryStyle {
    Bottom,
    Icons,
    Names,
}

impl HistoryStyle {
    pub fn height(self) -> f32 {
        let t = Tokens::get();
        match self {
            Self::Bottom => t.control_height,
            Self::Icons => t.control_height + t.space_10,
            Self::Names => TREE_ROW_HEIGHT,
        }
    }
}

pub(crate) struct HistoryChip<'a> {
    pub icon: &'a str,
    pub name: &'a str,
    pub kind: &'a str,
    pub selected: bool,
    pub hovered: bool,
    pub dim: bool,
    pub suppressed: bool,
    pub error: bool,
    pub warning: bool,
    pub accent: TabAccent,
}

/// Share the feature classification across chips and name rows. Surface aliases also
/// cover Patch/Stitch/Thicken and other surface features without a Surface-prefixed kind.
fn feature_icon_accent(kind: &str, icon: &str) -> Option<&'static str> {
    let tab = if kind.starts_with("Sketch") {
        "SKETCH"
    } else if kind.starts_with("Surface") || icon.starts_with("surface_") || icon == "continuity" {
        "SURFACE"
    } else if kind.starts_with("Mesh") {
        "MESH"
    } else if kind.starts_with("Form") {
        "FORM"
    } else {
        "MODEL"
    };
    crate::theme::icon_accent_for_tab(tab)
}

impl HistoryChip<'_> {
    /// All strip variants use this painter; no connections are drawn between features.
    pub fn paint(&self, painter: &Painter, rect: Rect, style: HistoryStyle) {
        let t = Tokens::get();
        let mut p = painter.clone();
        if self.dim || self.suppressed {
            p.multiply_opacity(0.42);
        }
        let fill = if self.error {
            t.form_error_fill
        } else if self.selected {
            self.accent.soft
        } else if self.hovered {
            t.hover
        } else if style == HistoryStyle::Names {
            egui::Color32::TRANSPARENT
        } else {
            t.timeline_item
        };
        let edge = if self.error {
            t.form_error
        } else if self.selected || self.hovered {
            self.accent.color
        } else {
            t.border
        };
        let radius = if style == HistoryStyle::Icons { t.radius_lg } else { t.radius_sm };
        if style != HistoryStyle::Names || self.error {
            let weight = if style == HistoryStyle::Icons && (self.selected || self.error) { 1.5 } else { 1.0 };
            p.rect(rect, radius, fill, Stroke::new(weight, edge), egui::StrokeKind::Inside);
        } else {
            p.rect_filled(rect, radius, fill);
        }
        let names = style == HistoryStyle::Names;
        let side = if names {
            t.space_14 + 1.0
        } else if style == HistoryStyle::Icons {
            t.space_16 + t.space_3
        } else {
            rect.height() - t.space_8
        };
        let icon =
            Rect::from_center_size(if names { pos2(rect.left() + t.space_8 + side / 2.0, rect.center().y) } else { rect.center() }, vec2(side, side));
        icons::paint_accented(&p, icon, self.icon, feature_icon_accent(self.kind, self.icon), 1.0);
        if names {
            let ink = if self.selected { self.accent.ink } else { t.text };
            let font = if self.selected { crate::fonts::semibold(t.font_body) } else { egui::FontId::proportional(t.font_body) };
            let text_rect = Rect::from_min_max(
                pos2(icon.right() + t.space_6 + 1.0, rect.top()),
                pos2(rect.right() - if self.error || self.warning { t.space_16 } else { t.space_8 }, rect.bottom()),
            );
            let galley = p.layout_no_wrap(self.name.to_owned(), font, ink);
            p.with_clip_rect(text_rect.intersect(p.clip_rect())).galley(pos2(text_rect.left(), rect.center().y - galley.size().y / 2.0), galley, ink);
        }
        if self.suppressed {
            p.line_segment([icon.left_bottom(), icon.right_top()], Stroke::new(1.5, t.text_dim));
        }
        if self.error || self.warning {
            let side = if names { t.space_12 } else { t.space_10 };
            let at = if names { pos2(rect.right() - t.space_3 - side, rect.center().y - side / 2.0) } else { rect.right_top() - vec2(side, 0.0) };
            icons::paint_icon(&p, Rect::from_min_size(at, vec2(side, side)), if self.error { "error" } else { "warning" }, 1.0);
        }
    }
}

/// A single drop indicator in either strip orientation.
pub(crate) fn history_drop_bar(p: &Painter, rect: Rect, vertical: bool, colour: egui::Color32) {
    let points = if vertical {
        [pos2(rect.left(), rect.center().y), pos2(rect.right(), rect.center().y)]
    } else {
        [pos2(rect.center().x, rect.top()), pos2(rect.center().x, rect.bottom())]
    };
    p.line_segment(points, Stroke::new(Tokens::get().space_2, colour));
}

/// Rollback line and hollow end circle; a vertical strip uses a horizontal line.
pub(crate) fn history_marker(p: &Painter, rect: Rect, vertical: bool, accent: TabAccent) {
    let t = Tokens::get();
    let outer_radius = t.space_10 / 2.0;
    let centre = if vertical { pos2(rect.right() - outer_radius, rect.center().y) } else { pos2(rect.center().x, rect.bottom() - outer_radius) };
    let direction = if vertical { vec2(1.0, 0.0) } else { vec2(0.0, 1.0) };
    let start = if vertical { rect.left_center() } else { rect.center_top() };
    // The line reaches one point inside the circle's outer edge, as on the reference board.
    let end = centre - direction * (outer_radius - 1.0);
    p.line_segment([start, end], Stroke::new(t.space_2, accent.color));
    // Circle strokes are centred on their radius: inset by half the stroke for a 10 pt outside diameter.
    p.circle(centre, outer_radius - t.space_2 / 2.0, if vertical { t.panel } else { t.timeline }, Stroke::new(t.space_2, accent.color));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip_selection_error_and_rollback_use_tokens_in_both_themes() {
        let previous = crate::theme::is_dark();
        for dark in [false, true] {
            crate::theme::set_dark(dark);
            let t = Tokens::get();
            let accent = TabAccent::for_tab(&t, "SURFACE");
            for style in [HistoryStyle::Icons, HistoryStyle::Names] {
                for (selected, error, dim) in [(true, false, false), (false, true, false), (true, false, true)] {
                    let rect = Rect::from_min_size(pos2(20.0, 20.0), vec2(158.0, style.height()));
                    let ctx = crate::fonts::test_context();
                    let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                        HistoryChip {
                            icon: "surface_extrude",
                            name: "Offset1",
                            kind: "SurfaceOffsetFeature",
                            selected,
                            hovered: false,
                            dim,
                            suppressed: false,
                            error,
                            warning: false,
                            accent,
                        }
                        .paint(ui.painter(), rect, style)
                    });
                    out.textures_delta.clear();
                    let background = out
                        .shapes
                        .iter()
                        .find_map(|s| match &s.shape {
                            egui::Shape::Rect(r) if r.rect == rect => Some(r),
                            _ => None,
                        })
                        .unwrap();
                    let opacity = if dim { 0.42 } else { 1.0 };
                    assert_eq!(background.fill, if error { t.form_error_fill } else { accent.soft }.gamma_multiply(opacity));
                    if style == HistoryStyle::Icons || error {
                        assert_eq!(background.stroke.color, if error { t.form_error } else { accent.color }.gamma_multiply(opacity));
                    }
                    if style == HistoryStyle::Names {
                        let text = out
                            .shapes
                            .iter()
                            .find_map(|s| match &s.shape {
                                egui::Shape::Text(t) => Some(t),
                                _ => None,
                            })
                            .unwrap();
                        assert_eq!(text.galley.text(), "Offset1");
                        assert_eq!(text.fallback_color, if selected { accent.ink } else { t.text }.gamma_multiply(opacity));
                    }
                }
            }
        }
        crate::theme::set_dark(previous);
    }

    #[test]
    fn chips_have_no_connector_shapes_in_either_theme_or_width() {
        let previous = crate::theme::is_dark();
        for dark in [false, true] {
            crate::theme::set_dark(dark);
            let t = Tokens::get();
            for style in [HistoryStyle::Bottom, HistoryStyle::Icons, HistoryStyle::Names] {
                let ctx = crate::fonts::test_context();
                let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                    for index in 0..3 {
                        let rect = Rect::from_min_size(
                            pos2(20.0, 20.0 + index as f32 * (style.height() + t.space_4)),
                            vec2(if style == HistoryStyle::Names { 158.0 } else { style.height() }, style.height()),
                        );
                        HistoryChip {
                            icon: "box",
                            name: "Box",
                            kind: "BoxFeature",
                            selected: index == 0,
                            hovered: false,
                            dim: index == 2,
                            suppressed: false,
                            error: false,
                            warning: false,
                            accent: TabAccent::for_tab(&t, "MODEL"),
                        }
                        .paint(ui.painter(), rect, style);
                    }
                });
                out.textures_delta.clear();
                let mut connections = 0;
                for clipped in out.shapes {
                    crate::test_support::each_shape(&clipped.shape, &mut |shape| {
                        if matches!(shape, egui::Shape::LineSegment { .. } | egui::Shape::Path(_)) {
                            connections += 1;
                        }
                    });
                }
                assert_eq!(connections, 0, "dark={dark}");
            }
        }
        crate::theme::set_dark(previous);
    }

    #[test]
    fn both_strip_orientations_share_the_drop_bar_and_marker_geometry() {
        let t = Tokens::get();
        let accent = TabAccent::for_tab(&t, "SURFACE");
        for vertical in [false, true] {
            let rect = Rect::from_min_size(pos2(20.0, 20.0), if vertical { vec2(160.0, 12.0) } else { vec2(12.0, 28.0) });
            let ctx = crate::fonts::test_context();
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| history_drop_bar(ui.painter(), rect, vertical, accent.color));
            out.textures_delta.clear();
            let lines: Vec<_> = out
                .shapes
                .iter()
                .filter_map(|s| if let egui::Shape::LineSegment { points, stroke } = s.shape { Some((points, stroke)) } else { None })
                .collect();
            assert_eq!(lines.len(), 1);
            assert_eq!(lines[0].1, Stroke::new(t.space_2, accent.color));
            assert_eq!(lines[0].0, if vertical { [rect.left_center(), rect.right_center()] } else { [rect.center_top(), rect.center_bottom()] });
        }
        let previous = crate::theme::is_dark();
        for dark in [false, true] {
            crate::theme::set_dark(dark);
            let t = Tokens::get();
            let accent = TabAccent::for_tab(&t, "SURFACE");
            // Both vertical widths retain the 12 pt marker row; the bottom strip rotates it.
            for (vertical, size) in [(true, vec2(46.0, 12.0)), (true, vec2(158.0, 12.0)), (false, vec2(12.0, 28.0))] {
                let rect = Rect::from_min_size(pos2(20.0, 20.0), size);
                let ctx = crate::fonts::test_context();
                let mut out = ctx.run_ui(egui::RawInput::default(), |ui| history_marker(ui.painter(), rect, vertical, accent));
                out.textures_delta.clear();
                let lines: Vec<_> = out
                    .shapes
                    .iter()
                    .filter_map(|s| match s.shape {
                        egui::Shape::LineSegment { points, stroke } => Some((points, stroke)),
                        _ => None,
                    })
                    .collect();
                assert_eq!(lines.len(), 1);
                assert_eq!(lines[0].1, Stroke::new(2.0, accent.color));
                let circles: Vec<_> = out
                    .shapes
                    .iter()
                    .filter_map(|s| match &s.shape {
                        egui::Shape::Circle(c) => Some(c),
                        _ => None,
                    })
                    .collect();
                assert_eq!(circles.len(), 1);
                let circle = circles[0];
                assert_eq!(circle.stroke, Stroke::new(2.0, accent.color));
                assert_eq!(circle.fill, if vertical { t.panel } else { t.timeline });
                assert_ne!(circle.fill, accent.color);
                let outer = circle.radius + circle.stroke.width / 2.0;
                assert_eq!(outer * 2.0, 10.0);
                let bounds = Rect::from_center_size(circle.center, vec2(outer * 2.0, outer * 2.0));
                assert!(rect.contains_rect(bounds));
                assert_eq!(lines[0].0[0], if vertical { rect.left_center() } else { rect.center_top() });
                if vertical {
                    assert_eq!(rect.height(), 12.0);
                    assert_eq!(bounds.right(), rect.right());
                    assert_eq!(lines[0].0[1], pos2(bounds.left() + 1.0, circle.center.y));
                } else {
                    assert_eq!(bounds.bottom(), rect.bottom());
                    assert_eq!(lines[0].0[1], pos2(circle.center.x, bounds.top() + 1.0));
                }
                assert!(!out.shapes.iter().any(|s| matches!(s.shape, egui::Shape::Path(_))), "no arrowheads");
            }
        }
        crate::theme::set_dark(previous);
    }
}
