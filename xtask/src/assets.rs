//! Resource inventory and provenance gate; no runtime resource registry.
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::{Component, Path};

const ASSET_EXT: &[&str] = &[
    "png",
    "jpg",
    "jpeg",
    "svg",
    "ico",
    "icns",
    "car",
    "vectorcraft",
    "ai",
    "psd",
    "afdesign",
    "fig",
    "sketch",
    "ttf",
    "otf",
    "woff",
    "woff2",
    "gif",
    "bmp",
    "webp",
    "wav",
    "mp3",
    "mp4",
    "stl",
    "step",
    "stp",
    "obj",
];

fn walk(dir: &Path, root: &Path, out: &mut Vec<String>) -> Result<(), String> {
    for e in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let e = e.map_err(|e| e.to_string())?;
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || name.starts_with("target") || matches!(name.as_str(), "plan" | "dist" | "node_modules") {
            continue;
        }
        let p = e.path();
        let kind = e.file_type().map_err(|e| e.to_string())?;
        if kind.is_symlink() {
            if p.starts_with(root.join("assets")) {
                return Err(format!("resource symlink: {}", p.display()));
            }
        } else if kind.is_dir() {
            walk(&p, root, out)?;
        } else if kind.is_file() {
            out.push(p.strip_prefix(root).map_err(|e| e.to_string())?.to_string_lossy().replace('\\', "/"));
        }
    }
    Ok(())
}

pub(super) fn file(root: &Path, name: &str) -> Result<(), String> {
    if name.is_empty() || name.contains('\\') || Path::new(name).components().any(|c| !matches!(c, Component::Normal(_))) {
        return Err(format!("resource path must be repository-relative: {name}"));
    }
    let p = root.join(name);
    if !p.is_file() || !p.canonicalize().map_err(|e| e.to_string())?.starts_with(root.canonicalize().map_err(|e| e.to_string())?) {
        return Err(format!("missing or external resource/consumer: {name}"));
    }
    Ok(())
}
pub(super) fn digest(root: &Path, name: &str) -> Result<String, String> {
    file(root, name)?;
    Ok(format!("{:x}", Sha256::digest(std::fs::read(root.join(name)).map_err(|e| format!("{name}: {e}"))?)))
}
fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str, String> {
    v.get(key).and_then(Value::as_str).ok_or_else(|| format!("manifest: missing string {key}"))
}
fn array<'a>(v: &'a Value, key: &str) -> Result<&'a [Value], String> {
    v.get(key).and_then(Value::as_array).map(Vec::as_slice).ok_or_else(|| format!("manifest: missing array {key}"))
}

pub fn run(root: &Path) -> Result<(), String> {
    let v: Value = serde_json::from_slice(&std::fs::read(root.join("assets/manifest.json")).map_err(|e| format!("manifest: {e}"))?)
        .map_err(|e| format!("manifest: {e}"))?;
    if v["version"].as_u64() != Some(1) {
        return Err("manifest: unsupported version".into());
    }
    let attribution = std::fs::read_to_string(root.join("ATTRIBUTION.md")).map_err(|e| e.to_string())?;
    let rows: BTreeSet<_> = attribution.lines().filter(|l| l.starts_with('|')).filter_map(|l| l.split('|').nth(1)).map(str::trim).collect();
    let mut ids = BTreeSet::new();
    let mut declared = BTreeSet::new();
    let mut aliases = Vec::new();
    for r in array(&v, "resources")? {
        let id = string(r, "id")?;
        if !ids.insert(id) {
            return Err(format!("duplicate resource id: {id}"));
        }
        let ownership = string(r, "ownership")?;
        if !matches!(ownership, "repository" | "dependency" | "system" | "user" | "absent") {
            return Err(format!("{id}: invalid ownership"));
        }
        if !matches!(string(r, "status")?, "current" | "archive" | "external" | "absent") {
            return Err(format!("{id}: invalid status"));
        }
        for p in array(r, "consumers")?.iter().chain(array(r, "licences")?) {
            file(root, p.as_str().ok_or("manifest: expected file path")?)?;
        }
        for f in array(r, "files")? {
            let p = string(f, "path")?;
            file(root, p)?;
            if ownership == "repository" && !p.starts_with("assets/") {
                return Err(format!("{id}: collected resource outside assets/: {p}"));
            }
            if !declared.insert(p) {
                return Err(format!("duplicate resource file: {p}"));
            }
            if ownership == "repository" && !rows.contains(p) {
                return Err(format!("missing attribution row: {p}"));
            }
            let role = string(f, "role")?;
            if !matches!(role, "source" | "export" | "licence") {
                return Err(format!("{id}: invalid role"));
            }
            if ownership == "repository" && ((role == "export" && !p.contains("/exports/")) || (role == "source" && p.contains("/exports/"))) {
                return Err(format!("{id}: source/export directory mismatch: {p}"));
            }
            if r["status"] == "archive" && !p.starts_with("assets/archive/") {
                return Err(format!("{id}: archive file outside archive/: {p}"));
            }
            if role == "export" && r["generator"].as_str().is_none() {
                return Err(format!("{id}: export has no generator"));
            }
            if let Some(h) = f["sha256"].as_str()
                && digest(root, p)? != h
            {
                return Err(format!("{id}: changed export {p}; regenerate and review"));
            }
        }
        for f in r["inputs"].as_array().into_iter().flatten() {
            let p = string(f, "path")?;
            if digest(root, p)? != string(f, "sha256")? {
                return Err(format!("{id}: changed source {p}; regenerate its exports"));
            }
        }
        if let Some(alias) = r["alias_of"].as_str() {
            aliases.push((id, alias));
        }
    }
    for (id, alias) in aliases {
        if !ids.contains(alias) || id == alias {
            return Err(format!("{id}: invalid alias {alias}"));
        }
    }
    let docs: BTreeSet<_> =
        array(&v, "documentation")?.iter().map(|v| v.as_str().ok_or("invalid documentation exemption")).collect::<Result<_, _>>()?;
    for p in &docs {
        file(root, p)?;
        if !p.starts_with("assets/") || !p.ends_with(".md") {
            return Err(format!("invalid documentation exemption: {p}"));
        }
    }
    let mut files = Vec::new();
    walk(root, root, &mut files)?;
    for p in files {
        if p.starts_with("assets/") && p != "assets/manifest.json" && !declared.contains(p.as_str()) && !docs.contains(p.as_str()) {
            return Err(format!("undeclared resource: {p}"));
        }
        let ext = Path::new(&p).extension().map(|e| e.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
        if ASSET_EXT.contains(&ext.as_str()) && !attribution.contains(&p) {
            return Err(format!("asset missing from ATTRIBUTION.md: {p}"));
        }
    }
    composer(root)?;
    println!("assets: {} resources, {} files; coverage, provenance and export digests verified", ids.len(), declared.len());
    Ok(())
}
fn composer(root: &Path) -> Result<(), String> {
    let dir = "assets/icons/app/macos/source/SolidCraft.icon";
    if !root.join(dir).exists() {
        return Ok(());
    }
    let v: Value = serde_json::from_slice(&std::fs::read(root.join(dir).join("icon.json")).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    fn images(v: &Value, names: &mut Vec<String>) {
        match v {
            Value::Object(m) => {
                for (k, v) in m {
                    if k == "image-name"
                        && let Some(n) = v.as_str()
                    {
                        names.push(n.into());
                    } else if k == "image-name-specializations" {
                        names.extend(v.as_array().into_iter().flatten().filter_map(|v| v["value"].as_str().map(str::to_string)));
                    } else {
                        images(v, names);
                    }
                }
            }
            Value::Array(a) => a.iter().for_each(|v| images(v, names)),
            _ => {}
        }
    }
    let mut names = Vec::new();
    images(&v, &mut names);
    if names.is_empty() {
        return Err("Composer project has no artwork layers".into());
    }
    for name in names {
        file(root, &format!("{dir}/Assets/{name}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(std::path::PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!("solidcraft-assets-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            std::fs::create_dir_all(root.join("assets/source")).unwrap();
            std::fs::write(root.join("assets/source/icon.svg"), "original").unwrap();
            std::fs::write(root.join("consumer.rs"), "").unwrap();
            std::fs::write(root.join("LICENSE-MIT"), "licence").unwrap();
            std::fs::write(root.join("ATTRIBUTION.md"), "| assets/source/icon.svg | author | original | MIT |\n").unwrap();
            let v = serde_json::json!({"version":1,"documentation":[],"resources":[{"id":"icon","ownership":"repository","status":"current","consumers":["consumer.rs"],"licences":["LICENSE-MIT"],"files":[{"path":"assets/source/icon.svg","role":"source"}],"inputs":[{"path":"assets/source/icon.svg","sha256":digest(&root,"assets/source/icon.svg").unwrap()}]}]});
            std::fs::write(root.join("assets/manifest.json"), v.to_string()).unwrap();
            Self(root)
        }
        fn change(&self, f: impl FnOnce(&mut Value)) {
            let p = self.0.join("assets/manifest.json");
            let mut v: Value = serde_json::from_slice(&std::fs::read(&p).unwrap()).unwrap();
            f(&mut v);
            std::fs::write(p, v.to_string()).unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn missing_consumers_uncollected_work_and_stale_inputs_fail() {
        let f = Fixture::new();
        run(&f.0).unwrap();
        std::fs::write(f.0.join("assets/drawing.ai"), "editable").unwrap();
        assert!(run(&f.0).unwrap_err().contains("undeclared"));
        std::fs::remove_file(f.0.join("assets/drawing.ai")).unwrap();
        std::fs::write(f.0.join("assets/source/icon.svg"), "changed").unwrap();
        assert!(run(&f.0).unwrap_err().contains("changed source"));
        std::fs::remove_file(f.0.join("consumer.rs")).unwrap();
        assert!(run(&f.0).unwrap_err().contains("missing or external"));
    }
    #[test]
    fn exact_attribution_unique_ids_and_safe_paths_are_required() {
        let f = Fixture::new();
        std::fs::write(f.0.join("ATTRIBUTION.md"), "Mention of assets/source/icon.svg").unwrap();
        assert!(run(&f.0).unwrap_err().contains("attribution row"));
        std::fs::write(f.0.join("ATTRIBUTION.md"), "| assets/source/icon.svg | author | original | MIT |\n").unwrap();
        f.change(|v| {
            let r = v["resources"][0].clone();
            v["resources"].as_array_mut().unwrap().push(r);
        });
        assert!(run(&f.0).unwrap_err().contains("duplicate resource id"));
        assert!(file(&f.0, "../outside").is_err());
        assert!(file(&f.0, "/tmp/outside").is_err());
    }
    #[test]
    fn export_tampering_and_missing_generators_fail() {
        let f = Fixture::new();
        f.change(|v| {
            v["resources"][0]["files"][0]["role"] = "export".into();
        });
        assert!(run(&f.0).unwrap_err().contains("source/export directory mismatch"));
        std::fs::create_dir_all(f.0.join("assets/exports")).unwrap();
        std::fs::write(f.0.join("assets/exports/icon.svg"), "export").unwrap();
        std::fs::write(f.0.join("ATTRIBUTION.md"), "| assets/exports/icon.svg | author | original | MIT |\n").unwrap();
        f.change(|v| {
            v["resources"][0]["files"][0]["path"] = "assets/exports/icon.svg".into();
        });
        assert!(run(&f.0).unwrap_err().contains("no generator"));
        f.change(|v| {
            v["resources"][0]["generator"] = "generate icon".into();
            v["resources"][0]["files"][0]["sha256"] = "wrong".into();
        });
        assert!(run(&f.0).unwrap_err().contains("changed export"));
    }
}
