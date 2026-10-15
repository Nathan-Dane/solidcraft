//! Build information for Help › About: the git commit and the build date (UTC). Both fall back to
//! "unknown" when git is missing (a source tarball). Also one test per UI scenario file
//! (`tests/scenarios/*.json`), so adding a scenario needs no Rust edit.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string()).filter(|s| !s.is_empty())
}

/// Days since 1970-01-01 to a civil date (proleptic Gregorian).
fn civil(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

fn main() {
    let commit = git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let dirty = git(&["status", "--porcelain", "--untracked-files=no"]).is_some();
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    let (y, m, d) = civil(secs.div_euclid(86_400));
    println!("cargo:rustc-env=SOLIDCRAFT_COMMIT={commit}{}", if dirty { "+" } else { "" });
    println!("cargo:rustc-env=SOLIDCRAFT_BUILD_DATE={y:04}-{m:02}-{d:02}");
    // A new commit in this checkout (or worktree) restamps the build.
    if let Some(dir) = git(&["rev-parse", "--git-dir"]) {
        println!("cargo:rerun-if-changed={dir}/logs/HEAD");
    }
    println!("cargo:rerun-if-changed=build.rs");
    scenario_tests();
    if let Err(e) = icon_data() {
        eprintln!("icon data: {e}");
        std::process::exit(1);
    }
}

/// `$OUT_DIR/scenario_tests.rs`: a `#[test]` per scenario file, named after it; a scenario with
/// a `{"pending": "why"}` step is `#[ignore]`d with that reason (it waits for a fix).
fn scenario_tests() {
    let dir = std::path::Path::new("tests/scenarios");
    println!("cargo:rerun-if-changed=tests/scenarios");
    let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .map(|rd| rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "json")).collect())
        .unwrap_or_default();
    files.sort();
    let mut out = String::new();
    for f in files {
        let Some(stem) = f.file_stem().and_then(|s| s.to_str()) else { continue };
        let mut name: String = stem.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' }).collect();
        if !name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
            name.insert_str(0, "s_");
        }
        let text = std::fs::read_to_string(&f).unwrap_or_default();
        let pending = serde_json::from_str::<serde_json::Value>(&text)
            .ok()
            .and_then(|v| v.as_array().and_then(|a| a.iter().find_map(|s| s.get("pending").map(|p| p.as_str().unwrap_or("pending").to_string()))));
        if let Some(why) = pending {
            out += &format!("#[test]\n#[ignore = {why:?}]\nfn {name}() {{\n    run({stem:?});\n}}\n\n");
        } else {
            out += &format!("#[test]\nfn {name}() {{\n    run({stem:?});\n}}\n\n");
        }
    }
    let path = std::path::Path::new(&std::env::var("OUT_DIR").unwrap_or_else(|_| ".".into())).join("scenario_tests.rs");
    let _ = std::fs::write(path, out);
}

/// `assets/icons/ui/design`: the SVG masters, `palette.toml` (every colour role in light and dark,
/// and the accents) and `aliases.toml`, compiled into `$OUT_DIR/icon_data.rs` for `icon_svg.rs`.
/// A missing hex, a role an accent does not know or an alias to a missing master fails the build.
fn icon_data() -> Result<(), String> {
    use std::fmt::Write as _;
    let manifest = std::env::var("CARGO_MANIFEST_DIR").map_err(|e| e.to_string())?;
    let root = std::path::Path::new(&manifest).join("../../assets/icons/ui/design");
    let read = |name: &str| std::fs::read_to_string(root.join(name)).map_err(|e| format!("{name}: {e}"));
    let table = |name: &str| read(name)?.parse::<toml::Table>().map_err(|e| format!("{name}: {e}"));
    let hex = |v: Option<&toml::Value>, what: &str| -> Result<String, String> {
        let s = v.and_then(toml::Value::as_str).ok_or_else(|| format!("{what}: missing hex"))?;
        let ok = s.len() == 7 && s.starts_with('#') && s[1..].bytes().all(|b| b.is_ascii_hexdigit());
        if ok { Ok(s.to_ascii_lowercase()) } else { Err(format!("{what}: `{s}` is not #rrggbb")) }
    };
    let rgb = |h: &str| format!("[0x{}, 0x{}, 0x{}]", &h[1..3], &h[3..5], &h[5..7]);
    let palette = table("palette.toml")?;
    let roles = palette.get("roles").and_then(toml::Value::as_table).ok_or("palette.toml: no [roles]")?;
    let names: Vec<&String> = roles.keys().collect();
    let mut out = String::new();
    let _ = writeln!(out, "pub(crate) const ROLE_NAMES: &[&str] = &{:?};", names);
    let (mut lights, mut darks, mut light_hex) = (Vec::new(), Vec::new(), Vec::new());
    for (name, role) in roles {
        let l = hex(role.get("light"), &format!("role {name} light"))?;
        let d = hex(role.get("dark"), &format!("role {name} dark"))?;
        lights.push(rgb(&l));
        darks.push(rgb(&d));
        light_hex.push(l);
    }
    let _ = writeln!(out, "pub(crate) const ROLE_LIGHT_HEX: &[&str] = &{:?};", light_hex);
    let _ = writeln!(out, "pub(crate) const PALETTE: [&[[u8; 3]]; 2] = [&[{}], &[{}]];", lights.join(", "), darks.join(", "));
    let mut accent_names = Vec::new();
    let mut accents = Vec::new();
    if let Some(table) = palette.get("accents").and_then(toml::Value::as_table) {
        for (acc, roles_of) in table {
            let roles_of = roles_of.as_table().ok_or_else(|| format!("accent {acc}: not a table"))?;
            let mut items = Vec::new();
            for (role, v) in roles_of {
                let idx = names.iter().position(|n| *n == role).ok_or_else(|| format!("accent {acc}: unknown role {role}"))?;
                let l = hex(v.get("light"), &format!("accent {acc} {role} light"))?;
                let d = hex(v.get("dark"), &format!("accent {acc} {role} dark"))?;
                items.push(format!("({idx}, {}, {})", rgb(&l), rgb(&d)));
            }
            accent_names.push(acc.clone());
            accents.push(format!("&[{}]", items.join(", ")));
        }
    }
    let _ = writeln!(out, "pub(crate) const ACCENT_NAMES: &[&str] = &{:?};", accent_names);
    let _ = writeln!(out, "pub(crate) const ACCENTS: &[&[(usize, [u8; 3], [u8; 3])]] = &[{}];", accents.join(", "));
    // The masters, by family.
    let mut svgs = Vec::new();
    let svg_dir = root.join("svg");
    println!("cargo:rerun-if-changed={}", root.display());
    for fam in ["model", "small", "flat", "workspace"] {
        let dir = svg_dir.join(fam);
        let mut files: Vec<_> =
            std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?.filter_map(Result::ok).map(|e| e.path()).collect();
        files.sort();
        for f in files.iter().filter(|f| f.extension().is_some_and(|e| e == "svg")) {
            let id = f.file_stem().and_then(|s| s.to_str()).ok_or("bad file name")?;
            let abs = f.canonicalize().map_err(|e| e.to_string())?;
            println!("cargo:rerun-if-changed={}", abs.display());
            svgs.push(format!("({id:?}, {fam:?}, include_str!({:?}))", abs.display().to_string()));
        }
    }
    let _ = writeln!(out, "pub(crate) static SVGS: &[(&str, &str, &str)] = &[{}];", svgs.join(", "));
    let aliases = table("aliases.toml")?;
    let mut al = Vec::new();
    if let Some(t) = aliases.get("alias").and_then(toml::Value::as_table) {
        for (alias, v) in t {
            let (master, accent) = match v {
                toml::Value::String(m) => (m.clone(), String::new()),
                toml::Value::Table(t) => (
                    t.get("master").and_then(toml::Value::as_str).ok_or_else(|| format!("alias {alias}: no master"))?.to_string(),
                    t.get("accent").and_then(toml::Value::as_str).unwrap_or("").to_string(),
                ),
                _ => return Err(format!("alias {alias}: expected a string or a table")),
            };
            if !accent.is_empty() && !accent_names.contains(&accent) {
                return Err(format!("alias {alias}: unknown accent {accent}"));
            }
            al.push(format!("({alias:?}, {master:?}, {accent:?})"));
        }
    }
    let _ = writeln!(out, "pub(crate) const ALIASES: &[(&str, &str, &str)] = &[{}];", al.join(", "));
    let dest = std::path::Path::new(&std::env::var("OUT_DIR").map_err(|e| e.to_string())?).join("icon_data.rs");
    std::fs::write(dest, out).map_err(|e| e.to_string())
}
