//! A read-only declaration of persistent viewport effects. Settings remain in the session's
//! existing effect fields; only activation order is recorded separately, never in a design.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Session, SurfaceAnalysis};

/// Stable identity of an effect slot, retained when its analysis is replaced.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusId {
    SurfaceAnalysis,
    Section,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusKind {
    Mode,
    Result,
    Attention,
}

/// Structured settings, without unit formatting or translated text.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum StatusSetting {
    Zebra {
        stripes: f64,
    },
    Draft {
        /// Normalised pull direction.
        pull: [f64; 3],
        /// The stored Draft threshold is in degrees.
        angle_deg: f64,
    },
    Environment,
    Accessibility {
        /// Normalised direction from which the tool approaches.
        direction: [f64; 3],
    },
    Curvature {
        radius_mm: f64,
    },
    Continuity {
        /// Number of persistent seam records currently shaded.
        edge_count: usize,
    },
    Section {
        origin_mm: [f64; 3],
        /// Normalised world-space plane normal.
        normal: [f64; 3],
        hatch: bool,
    },
}

/// An executable request that ends this effect, through the same command as other clients.
#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusClear {
    pub command: String,
    pub params: StatusClearParams,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusClearParams {
    pub clear: bool,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusItem {
    pub id: StatusId,
    pub kind: StatusKind,
    /// Source command ID, whose catalog label consumers can translate.
    pub command: String,
    pub setting: StatusSetting,
    pub has_key: bool,
    pub clear: StatusClear,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StatusItems {
    /// Oldest active slot first; replacing an analysis retains its position.
    pub items: Vec<StatusItem>,
}

impl Session {
    /// Current effects, derived without evaluating the model or changing any session state.
    pub fn status_items(&self) -> Vec<StatusItem> {
        let analysis = self
            .analysis
            .map(|a| match a {
                SurfaceAnalysis::Zebra { stripes } => ("inspect.zebra", StatusSetting::Zebra { stripes }, false),
                SurfaceAnalysis::Draft { pull, angle } => ("inspect.draft", StatusSetting::Draft { pull: pull.into(), angle_deg: angle }, true),
                SurfaceAnalysis::Environment => ("inspect.environment_map", StatusSetting::Environment, false),
                SurfaceAnalysis::Access { dir } => ("inspect.accessibility", StatusSetting::Accessibility { direction: dir.into() }, true),
                SurfaceAnalysis::Curvature { radius } => ("inspect.curvature_map", StatusSetting::Curvature { radius_mm: radius }, true),
            })
            .or_else(|| {
                (!self.continuity.is_empty()).then_some((
                    "inspect.continuity",
                    StatusSetting::Continuity { edge_count: self.continuity.len() },
                    false,
                ))
            });
        let analysis = analysis.map(|(command, setting, has_key)| item(StatusId::SurfaceAnalysis, command, setting, has_key));
        let section = self.section.map(|(origin, normal)| {
            item(
                StatusId::Section,
                "inspect.section",
                StatusSetting::Section { origin_mm: origin.into(), normal: normal.into(), hatch: self.section_hatch },
                false,
            )
        });
        // Public effect fields can also be populated by embedders without command chronology.
        // Include those effects after recorded slots, without inventing or storing timestamps.
        let mut items = Vec::new();
        for id in self.status_order.iter().chain([StatusId::SurfaceAnalysis, StatusId::Section].iter()) {
            let effect = match id {
                StatusId::SurfaceAnalysis => &analysis,
                StatusId::Section => &section,
            };
            if let Some(effect) = effect
                && !items.iter().any(|item: &StatusItem| item.id == *id)
            {
                items.push(effect.clone());
            }
        }
        items
    }

    pub(crate) fn status_slot(&mut self, id: StatusId, active: bool) {
        if !active {
            self.status_order.retain(|slot| *slot != id);
        } else if !self.status_order.contains(&id) {
            self.status_order.push(id);
        }
    }
}

fn item(id: StatusId, command: &str, setting: StatusSetting, has_key: bool) -> StatusItem {
    StatusItem {
        id,
        kind: StatusKind::Mode,
        command: command.into(),
        setting,
        has_key,
        clear: StatusClear { command: command.into(), params: StatusClearParams { clear: true } },
    }
}
