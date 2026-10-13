//! Stable UI rendering boundary for the collected artwork. Compiled only at L5.
#[path = "../../../assets/icons/ui/source/icons.rs"]
mod source;
pub use source::paint;
#[path = "../../../assets/icons/ui/source/ids.rs"]
mod ids;
pub use ids::{FALLBACK_IDS, IDS};
#[path = "../../../assets/icons/ui/source/glyphs.rs"]
pub(crate) mod glyphs;
#[path = "../../../assets/icons/ui/source/markers.rs"]
pub(crate) mod markers;
#[path = "../../../assets/icons/ui/source/navigation.rs"]
pub(crate) mod navigation;
#[path = "../../../assets/icons/ui/source/pointer.rs"]
pub(crate) mod pointer;

#[cfg(test)]
mod reference_tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn index_matches_artwork_and_command_references() {
        let source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/icons/ui/source/icons.rs"));
        let drawn: BTreeSet<_> = source
            .lines()
            .map(str::trim)
            .filter(|s| s.starts_with('"') && s.contains("=>"))
            .flat_map(|s| s.split("=>").next().unwrap_or_default().split('|').map(|s| s.trim().trim_matches('"')))
            .collect();
        assert_eq!(drawn, IDS.iter().copied().collect());
        assert_eq!(IDS.len(), drawn.len(), "duplicate icon IDs");
        let known: BTreeSet<_> = IDS.iter().chain(FALLBACK_IDS).copied().collect();
        for command in solidcraft_engine::command_specs() {
            assert!(known.contains(command.icon), "{}: unindexed icon {}", command.id, command.icon);
        }
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/manifest.json"))).unwrap();
        let entry = manifest["resources"].as_array().unwrap().iter().find(|r| r["id"] == "icon.ui").unwrap();
        let names = |key: &str| entry[key].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect::<BTreeSet<_>>();
        assert_eq!(names("symbols"), drawn);
        assert_eq!(names("fallback_symbols"), FALLBACK_IDS.iter().copied().collect());
    }

    /// Extract a literal third argument, accounting for nested Rust expressions and strings.
    fn literal_icon(call: &str) -> Option<&str> {
        let mut depth = 0;
        let mut quoted = false;
        let mut escaped = false;
        let mut start = 0;
        let mut argument = 0;
        for (i, c) in call.char_indices() {
            if quoted {
                if escaped {
                    escaped = false;
                } else if c == '\\' {
                    escaped = true;
                } else if c == '"' {
                    quoted = false;
                }
                continue;
            }
            match c {
                '"' => quoted = true,
                '(' | '[' | '{' => depth += 1,
                ')' if depth == 0 => break,
                ')' | ']' | '}' => depth -= 1,
                ',' if depth == 0 => {
                    if argument == 2 {
                        let s = call.get(start..i)?.trim();
                        return s.strip_prefix('"')?.strip_suffix('"');
                    }
                    argument += 1;
                    start = i + 1;
                }
                _ => {}
            }
        }
        None
    }

    #[test]
    fn direct_ui_icon_references_are_indexed() {
        let known: BTreeSet<_> = IDS.iter().chain(FALLBACK_IDS).copied().collect();
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut checked = 0;
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&p).unwrap();
            for suffix in text.split("icons::paint(").skip(1) {
                if let Some(name) = literal_icon(suffix) {
                    assert!(known.contains(name), "{}: unindexed direct icon {name}", p.display());
                    checked += 1;
                }
            }
        }
        assert!(checked > 0, "no direct references were validated");
        assert_eq!(literal_icon("&p, Rect::from_min_size(pos2(1., 2.), vec2(3., 4.)), \"home\", t.icon, t.fill, t.accent)"), Some("home"));
    }
}
