//! Standard behaviours are registered here; file declarations alone do not offer a control.
use crate::{
    theme::Tokens,
    theme_files::{OptionDef, OptionValue, ThemeFile},
};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Availability {
    Available,
    /// Message-catalog id, translated at the presentation boundary.
    #[allow(dead_code)] // Hardware-dependent registrations arrive with blur and tab providers.
    Unavailable(&'static str),
}

pub(crate) struct Provider {
    pub id: &'static str,
    pub label: &'static str,
    pub help: &'static str,
    pub availability: fn() -> Availability,
    pub apply: fn(&mut Tokens, &OptionValue),
}

// Future blur/tab_colours/tab_fade_ms implementations register their behaviour here.
pub(crate) const PROVIDERS: &[Provider] = &[Provider {
    id: "opacity",
    label: "prefs.theme_option_opacity",
    help: "prefs.theme_option_opacity_help",
    availability: || Availability::Available,
    apply: apply_opacity,
}];

pub(crate) fn standard(id: &str) -> bool {
    matches!(id, "opacity" | "blur" | "tab_colours" | "tab_fade_ms")
}

pub(crate) fn provider<'a>(id: &str, providers: &'a [Provider]) -> Option<&'a Provider> {
    providers.iter().find(|p| p.id == id)
}

/// A child of an absent provider is absent too, even if it has a custom id.
pub(crate) fn offered<'a>(file: &'a ThemeFile, providers: &[Provider]) -> Vec<&'a OptionDef> {
    let ordered = crate::theme_files::ordered_options(file);
    ordered
        .iter()
        .copied()
        .filter(|option| {
            let mut current = Some(*option);
            for _ in 0..=crate::theme_files::MAX_OPTIONS {
                let Some(def) = current else { return true };
                if standard(&def.id) && provider(&def.id, providers).is_none() {
                    return false;
                }
                current = def.parent.as_ref().and_then(|id| ordered.iter().find(|o| &o.id == id).copied());
            }
            false
        })
        .collect()
}

pub(crate) fn apply(file: &ThemeFile, values: &BTreeMap<String, OptionValue>, tokens: &mut Tokens) {
    let offered = offered(file, PROVIDERS);
    for def in &offered {
        let mut parent = def.parent.as_deref();
        let mut active = true;
        for _ in 0..crate::theme_files::MAX_OPTIONS {
            let Some(id) = parent else { break };
            active &= values.get(id) == Some(&OptionValue::Bool(true));
            active &= provider(id, PROVIDERS).is_none_or(|p| (p.availability)() == Availability::Available);
            parent = offered.iter().find(|o| o.id == id).and_then(|o| o.parent.as_deref());
        }
        if let Some(provider) = provider(&def.id, PROVIDERS)
            && active
            && (provider.availability)() == Availability::Available
            && let Some(value) = values.get(&def.id)
        {
            (provider.apply)(tokens, value);
        }
    }
}

/// Alpha changes only these floating surface fills. Each fill retains its unmultiplied RGB.
/// Check both viewport extremes (black/white also cover background overrides and busy models).
fn apply_opacity(t: &mut Tokens, value: &OptionValue) {
    let OptionValue::Num(percent) = value else { return };
    if !percent.is_finite() || *percent >= 100.0 {
        return;
    }
    let scale = percent.clamp(40.0, 100.0) / 100.0;
    let body_inks = [t.text, t.form_ink, t.dialog_label];
    for fill in [&mut t.panel, &mut t.panel_header, &mut t.dialog_bg, &mut t.dialog_body, &mut t.dialog_footer, &mut t.overlay] {
        *fill = readable_fill(*fill, &body_inks, scale);
    }
    t.nav_tip = readable_fill(t.nav_tip, &[t.nav_tip_ink], scale);
}

pub(crate) fn composite(fill: egui::Color32, background: egui::Color32) -> egui::Color32 {
    let rgb = fill.to_srgba_unmultiplied();
    let a = f64::from(rgb[3]) / 255.0;
    let channel = |index: usize| (f64::from(rgb[index]) * a + f64::from(background[index]) * (1.0 - a)).round() as u8;
    egui::Color32::from_rgb(channel(0), channel(1), channel(2))
}

fn readable_fill(fill: egui::Color32, inks: &[egui::Color32], scale: f32) -> egui::Color32 {
    let [r, g, b, alpha] = fill.to_srgba_unmultiplied();
    let requested = (f32::from(alpha) * scale).round() as u8;
    // At most 256 cheap colour comparisons per changed role; no allocations, GPU or I/O.
    for a in requested..=255 {
        let candidate = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
        if [egui::Color32::BLACK, egui::Color32::WHITE].iter().all(|background| {
            let ground = composite(candidate, *background);
            inks.iter().all(|ink| crate::theme::contrast_ratio(composite(*ink, ground), ground) >= 4.5)
        }) {
            return candidate;
        }
    }
    // An authored foreground may already fail at full opacity. Never alter that foreground;
    // full alpha gives it the strongest protection this option can supply.
    egui::Color32::from_rgb(r, g, b)
}
