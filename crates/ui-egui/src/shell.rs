//! Transient shell geometry shared by automation, drawing and viewport occlusion.
use egui::{Rect, pos2, vec2};
use serde::{Deserialize, Serialize};

use crate::{SolveApp, theme::Tokens, toolbar};

/// Shell frame selected by automation for this session.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LayoutMode {
    #[default]
    Floating,
    Docked,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Cards {
    pub workspace: Rect,
    pub ribbon: Rect,
    pub bottom: f32,
}

impl Cards {
    pub fn new(available: Rect, mode: LayoutMode) -> Self {
        let t = Tokens::get();
        let margin = if mode == LayoutMode::Floating { t.space_8 } else { 0.0 };
        let gap = if mode == LayoutMode::Floating { t.space_6 } else { 1.0 };
        let left = available.left() + margin;
        let top = available.top() + margin;
        let right = (available.right() - margin).max(left);
        let workspace = Rect::from_min_size(
            pos2(left, top),
            vec2(toolbar::WORKSPACE_WIDTH.min((right - left).max(0.0)), toolbar::RIBBON_HEIGHT - if mode == LayoutMode::Docked { 1.0 } else { 0.0 }),
        );
        let ribbon = Rect::from_min_max(pos2((workspace.right() + gap).min(right), top), pos2(right, top + toolbar::RIBBON_HEIGHT));
        Self { workspace, ribbon, bottom: ribbon.bottom() + margin }
    }
}

/// Shell panels measured after drawing the full-bleed viewport. No design or preferences data.
#[derive(Default)]
pub(crate) struct State {
    pub cards: Vec<Rect>,
    pub panels: Vec<Rect>,
    pub content: Option<Rect>,
}

/// One source of occlusion for navigation, picking, overlays and fit.
pub(crate) fn covered_rects(app: &SolveApp) -> Vec<Rect> {
    let mut rects = Vec::new();
    if app.ui.layout == LayoutMode::Floating {
        rects.extend(app.shell.cards.iter().copied());
        rects.extend(app.shell.panels.iter().copied());
    }
    rects.extend(app.viewport.palette_rect.filter(|_| app.session.active_sketch.is_some()));
    rects.extend(app.viewport.dialog_rect.filter(|_| app.dialog.is_some()));
    rects.retain(|r| r.is_finite() && r.is_positive());
    rects
}

pub(crate) fn covered(app: &SolveApp, p: egui::Pos2) -> bool {
    covered_rects(app).iter().any(|r| r.contains(p))
}

/// The former central panel remains the anchor for controls and docked panels.
pub(crate) fn content_rect(app: &SolveApp, viewport: Rect) -> Rect {
    if app.ui.layout == LayoutMode::Floating { app.shell.content.unwrap_or(viewport).intersect(viewport) } else { viewport }
}

pub(crate) fn panel_top(app: &SolveApp, viewport: Rect, bottom: f32, side: crate::frame::Side) -> f32 {
    let content = content_rect(app, viewport);
    if app.ui.layout == LayoutMode::Floating && side == crate::frame::Side::Right {
        // Unlike compact mode, short panels never reclaim the compass's reserved area.
        app.viewport.layout.as_ref().map_or(
            content.top(),
            |l| {
                if l.axis_view.is_positive() { l.axis_view.bottom().min(bottom) } else { content.top() }
            },
        )
    } else {
        (viewport.top() + crate::viewport::AXIS_VIEW_CLEARANCE).min((bottom - 360.0).max(viewport.top()))
    }
}

#[cfg(test)]
#[path = "shell_tests.rs"]
mod tests;
