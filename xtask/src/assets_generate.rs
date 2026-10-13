//! Explicit resource regeneration. Normal builds consume checked-in exports.
use serde_json::Value;
use std::path::Path;
#[path = "../../assets/icons/app/shared/source/app_icon.rs"]
mod app_icon;
#[path = "../../assets/icons/app/shared/source/windows_ico.rs"]
mod windows_ico;

pub fn command(root: &Path, args: &[&str]) -> Result<(), String> {
    match args {
        [] => crate::assets::run(root),
        ["--generate", "icon.ui.sheet"] => {
            let mut c = crate::cargo();
            c.args(["run", "--profile", "ci", "-p", "solidcraft-ui-egui", "--example", "icon-sheet"]);
            crate::run(c, "UI icon review sheet")
        }
        ["--generate", id @ ("icon.app.linux" | "icon.app.windows" | "icon.app.macos")] => generate(root, id),
        ["--check-macos"] => check_macos(root),
        _ => Err("usage: cargo xtask assets [--generate icon.app.linux|icon.app.windows|icon.app.macos|icon.ui.sheet | --check-macos]".into()),
    }
}

fn cli(out: &Path, size: usize) -> Result<(), String> {
    let mut c = crate::cargo();
    c.args(["run", "-q", "--profile", "ci", "-p", "solidcraft-cli", "--", "icon", "--size", &size.to_string(), "--out"]).arg(out);
    crate::run(c, "solidcraft-cli icon")
}
fn apple(root: &Path, stage: &Path) -> Result<(), String> {
    let mut c = std::process::Command::new("xcrun");
    c.current_dir(root)
        .arg("actool")
        .arg(root.join("assets/icons/app/macos/source/SolidCraft.icon"))
        .arg("--compile")
        .arg(stage)
        .args(["--platform", "macosx", "--minimum-deployment-target", "11.0", "--app-icon", "SolidCraft", "--output-partial-info-plist"])
        .arg(stage.join("icon-info.plist"))
        .args(["--output-format", "human-readable-text"]);
    crate::run(c, "actool (requires Xcode with Icon Composer support)")
}
fn generate(root: &Path, id: &str) -> Result<(), String> {
    let stage = root.join("target/resources").join(id);
    std::fs::create_dir_all(&stage).map_err(|e| e.to_string())?;
    let mut copies = Vec::new();
    match id {
        "icon.app.windows" => {
            let dib = stage.join("solidcraft-executable.ico");
            std::fs::write(&dib, windows_ico::encode().map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            let png = stage.join("solidcraft.ico");
            cli(&png, 256)?;
            copies.push((dib, "assets/icons/app/windows/exports/solidcraft-executable.ico".into()));
            copies.push((png, "assets/icons/app/windows/exports/solidcraft.ico".into()));
        }
        "icon.app.linux" => {
            for size in [16, 24, 32, 48, 64, 128, 256, 512] {
                let png = stage.join(format!("{size}.png"));
                cli(&png, size)?;
                copies.push((png, format!("assets/icons/app/linux/exports/hicolor/{size}x{size}/apps/ai.storyteller.solidcraft.png")));
            }
        }
        "icon.app.macos" => {
            apple(root, &stage)?;
            copies.push((stage.join("Assets.car"), "assets/icons/app/macos/exports/Assets.car".into()));
            copies.push((stage.join("SolidCraft.icns"), "assets/icons/app/macos/exports/icon.icns".into()));
        }
        _ => return Err(format!("unknown generator: {id}")),
    }
    // Finish every producer before replacing any committed output.
    for (source, target) in copies {
        let target = root.join(target);
        if let Some(dir) = target.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        std::fs::copy(source, target).map_err(|e| e.to_string())?;
    }
    let path = root.join("assets/manifest.json");
    let mut v: Value = serde_json::from_slice(&std::fs::read(&path).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    let entries = v["resources"].as_array_mut().ok_or("manifest: missing resources")?;
    let r = entries.iter_mut().find(|r| r["id"] == id).ok_or_else(|| format!("manifest: missing {id}"))?;
    for f in r["files"].as_array_mut().ok_or("manifest: missing files")? {
        if f["role"] == "export" {
            f["sha256"] = crate::assets::digest(root, f["path"].as_str().ok_or("manifest: missing path")?)?.into();
        }
    }
    for f in r["inputs"].as_array_mut().ok_or("manifest: missing inputs")? {
        f["sha256"] = crate::assets::digest(root, f["path"].as_str().ok_or("manifest: missing path")?)?.into();
    }
    std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(&v).map_err(|e| e.to_string())?)).map_err(|e| e.to_string())?;
    println!("{id}: exports and digests updated; review and commit together");
    Ok(())
}

fn catalogue(path: &Path) -> Result<Vec<Value>, String> {
    let out = std::process::Command::new("xcrun").args(["assetutil", "--info"]).arg(path).output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).into_owned());
    }
    let mut entries: Vec<Value> = serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())?;
    // Compiler/storage timestamps, random rendition names and their metadata digests do not describe artwork.
    entries.retain(|e| e.get("Name").is_some());
    for e in &mut entries {
        if let Some(m) = e.as_object_mut() {
            m.remove("SHA1Digest");
            m.remove("SizeOnDisk");
            m.remove("RenditionName");
        }
    }
    entries.sort_by_key(Value::to_string);
    Ok(entries)
}
fn check_macos(root: &Path) -> Result<(), String> {
    crate::assets::run(root)?;
    let stage = root.join("target/resources/macos-check");
    std::fs::create_dir_all(&stage).map_err(|e| e.to_string())?;
    apple(root, &stage)?;
    let saved = root.join("assets/icons/app/macos/exports");
    if std::fs::read(stage.join("SolidCraft.icns")).map_err(|e| e.to_string())?
        != std::fs::read(saved.join("icon.icns")).map_err(|e| e.to_string())?
        || catalogue(&stage.join("Assets.car"))? != catalogue(&saved.join("Assets.car"))?
    {
        return Err("macOS artwork differs from the collected exports; inspect before regenerating".into());
    }
    println!("macOS: fallback images and catalogue rendition metadata match the editable project");
    Ok(())
}
