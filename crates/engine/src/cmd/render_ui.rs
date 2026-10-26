//! Transient Render view requests and explicit refusals for planned interactions.
use super::CommandSpec;
use crate::schema::{TypedContract, output_schema_for, render_ui::ResultIndex};
use crate::{Result, Session, params::bad};
use serde::Deserialize;
use serde_json::{Value, json};

const RESULT_INDEX: TypedContract<ResultIndex> = TypedContract::new();

macro_rules! waiting {
    ($id:literal, $label:literal, $tab:literal, $panel:literal, $icon:literal, $reason:literal) => {
        CommandSpec::new($id, $label, |_, _| Err(crate::EngineError::Other(concat!("not supported yet: ", $reason).into())))
            .enabled(|_| Err(concat!("not supported yet: ", $reason).into()))
            .planned("5493")
            .noundo()
            .doc_only()
            .at($tab, $panel)
            .icon($icon)
            .params("No supported parameters. Always refuses without changing the design or history.")
    };
}

pub static COMMANDS: &[CommandSpec] = &[
    CommandSpec::new("render.result.select", "Select Render Result", crate::checked!("render.result.select", select_result))
        .icon("render_image").noundo().doc_only()
        .enabled(results_ready)
        .contract(&RESULT_INDEX).output(output_schema_for::<ResultIndex>)
        .params("index: integer 0..7, oldest first; must be retained. Selects a completed result as transient per-document view state; echoes index."),
    CommandSpec::new("render.result.discard", "Discard Render Result", crate::checked!("render.result.discard", discard_result))
        .icon("delete").noundo().doc_only()
        .enabled(results_ready)
        .contract(&RESULT_INDEX).output(output_schema_for::<ResultIndex>)
        .params("index: integer 0..7, oldest first; must be retained. Drops only that transient result, selecting the newest remaining result if necessary; echoes index."),
    CommandSpec::new("render.output.size", "Set Render Output Size", output_size)
        .icon("settings")
        .noundo()
        .doc_only()
        .params("width, height: 1..16384; custom?: bool. Validated Render output draft request; headless echo only, no design edits or job."),
    CommandSpec::new("render.result.view", "View Render Image", result_view)
        .icon("render_image")
        .noundo()
        .doc_only()
        .params("view: model|image|fit|actual. Validated result canvas request; headless echo only, never starts a job."),
    CommandSpec::new("render.result.close", "Close Render Image", close_result)
        .icon("close")
        .key("Escape")
        .noundo()
        .doc_only()
        .params("No parameters. Return to Model Preview; headless echo only, never cancels a job."),
    waiting!("render.lights.add", "Add Light", "RENDER SCENE", "LIGHTS", "render_scene", "named light creation"),
    waiting!("appearance.paint", "Paint Appearance", "RENDER APPEARANCE", "APPEARANCE", "appearance", "product picking and appearance painting"),
    waiting!("appearance.pick_up", "Pick Up Appearance", "RENDER APPEARANCE", "APPEARANCE", "select", "product picking and appearance sampling"),
    waiting!("render.lights.solo", "Solo Light", "RENDER SCENE", "LIGHTS", "render_scene", "light visibility and solo controls"),
    waiting!("render.compare", "Compare Images", "RENDER", "RESULT", "compare", "retained image comparison"),
    waiting!("render.batch", "Render All Shots", "RENDER", "BATCH", "render_image", "the saved-shot render queue"),
];

fn results_ready(s: &Session) -> std::result::Result<(), String> {
    if s.render_jobs.has_results() { Ok(()) } else { Err("no completed render results are retained".into()) }
}

fn indexed_result(s: &mut Session, params: &Value, command: &str, discard: bool) -> Result<Value> {
    let input: ResultIndex = serde_json::from_value(params.clone()).map_err(|e| bad(command, e.to_string()))?;
    if input.index > 7 {
        return Err(bad(command, "index must be in 0..7"));
    }
    s.render_jobs.result_index(input.index as usize, discard).map_err(crate::render_jobs::job_error)?;
    Ok(json!({"index":input.index}))
}

fn select_result(s: &mut Session, params: &Value) -> Result<Value> {
    indexed_result(s, params, "render.result.select", false)
}

fn discard_result(s: &mut Session, params: &Value) -> Result<Value> {
    indexed_result(s, params, "render.result.discard", true)
}

#[cfg(test)]
#[test]
fn render_workspace_planned_commands_refuse_without_mutation() {
    use serde_json::{Value, json};
    assert!(super::find_command("appearance.replace").is_some_and(|command| command.planned.is_none()));
    let mut session = crate::Session::default();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":800}})).unwrap();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":900}})).unwrap();
    session.execute("edit.undo", &json!({})).unwrap();
    let document = session.doc.clone();
    let before = (session.undo.len(), session.redo.len(), session.revision, session.evaluations, session.is_dirty());
    for command in COMMANDS {
        if command.planned.is_none() {
            continue;
        }
        assert!(!command.info(&session).enabled);
        for params in [json!({}), Value::Null, json!([]), json!({"target":1e300,"name":"hostile"})] {
            let error = session.execute(command.id, &params).unwrap_err().to_string();
            assert!(error.contains("not supported yet"), "{error}");
            assert!(std::sync::Arc::ptr_eq(&document, &session.doc));
            assert_eq!(before, (session.undo.len(), session.redo.len(), session.revision, session.evaluations, session.is_dirty()));
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OutputSize {
    width: u32,
    height: u32,
    #[serde(default)]
    custom: bool,
}
fn output_size(_: &mut Session, params: &Value) -> Result<Value> {
    let input: OutputSize = serde_json::from_value(params.clone()).map_err(|e| bad("render.output.size", e.to_string()))?;
    if !(1..=16384).contains(&input.width) || !(1..=16384).contains(&input.height) {
        return Err(bad("render.output.size", "dimensions must be in 1..16384"));
    }
    let mut reply = json!({"width":input.width,"height":input.height});
    if input.custom {
        reply["custom"] = json!(true);
    }
    Ok(reply)
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ResultView {
    view: String,
}
fn result_view(_: &mut Session, params: &Value) -> Result<Value> {
    let input: ResultView = serde_json::from_value(params.clone()).map_err(|e| bad("render.result.view", e.to_string()))?;
    if !matches!(input.view.as_str(), "model" | "image" | "fit" | "actual") {
        return Err(bad("render.result.view", "view must be model, image, fit or actual"));
    }
    Ok(json!({"view":input.view}))
}
fn close_result(_: &mut Session, params: &Value) -> Result<Value> {
    if !params.as_object().is_some_and(|p| p.is_empty()) {
        return Err(bad("render.result.close", "expected no parameters"));
    }
    Ok(json!({"view":"model"}))
}

#[cfg(test)]
#[test]
fn output_viewer_requests_are_validated_headless_echoes() {
    let mut s = Session::default();
    let document = s.doc.clone();
    let before = (s.revision, s.undo.len(), s.evaluations);
    for size in [json!({"width":1280,"height":720}), json!({"width":1920,"height":1080})] {
        assert_eq!(s.execute("render.output.size", &size).unwrap(), size);
    }
    let custom = json!({"width":1920,"height":1080,"custom":true});
    assert_eq!(s.execute("render.output.size", &custom).unwrap(), custom);
    assert!(s.execute("render.output.size", &json!({"width":1920,"height":1080,"custom":"yes"})).is_err());
    for view in ["model", "image", "fit", "actual"] {
        let request = json!({"view":view});
        assert_eq!(s.execute("render.result.view", &request).unwrap(), request);
    }
    assert_eq!(s.execute("render.result.close", &json!({})).unwrap(), json!({"view":"model"}));
    for request in
        [json!(null), json!({}), json!({"width":0,"height":1}), json!({"width":u64::MAX,"height":1}), json!({"width":1,"height":1,"extra":true})]
    {
        assert!(s.execute("render.output.size", &request).is_err());
    }
    for request in [json!(null), json!({"view":"unknown"}), json!({"view":"image","extra":true})] {
        assert!(s.execute("render.result.view", &request).is_err());
        // Session normalizes null parameters to an empty object for no-argument commands.
        if request.is_object() {
            assert!(s.execute("render.result.close", &request).is_err());
        }
    }
    assert!(std::sync::Arc::ptr_eq(&document, &s.doc));
    assert_eq!(before, (s.revision, s.undo.len(), s.evaluations));
}

#[cfg(test)]
fn complete_result(s: &mut Session, shot: &str) -> solidcraft_render::job::JobId {
    let reply = s.execute("render.start", &json!({"setup":shot})).unwrap();
    let job = serde_json::from_value(reply["job"].clone()).unwrap();
    let host = s.render_job_host().unwrap().clone();
    for _ in 0..1000 {
        host.pump(4096).unwrap();
        if s.execute("render.status", &json!({"job":job})).unwrap()["progress"]["phase"] == "completed" {
            return job;
        }
    }
    panic!("bounded result fixture did not complete");
}

#[cfg(test)]
#[test]
fn retained_results_cap_order_metadata_and_selection_never_edit_a_design() {
    let mut s = Session::default();
    s.set_render_job_host(Some(solidcraft_render::job::CpuHost::cooperative())).unwrap();
    s.execute("render.settings.set", &json!({"name":"Hero","set":{"width":16,"height":8}})).unwrap();
    s.execute("render.settings.set", &json!({"name":"Hero","set":{"width":32}})).unwrap();
    s.execute("edit.undo", &json!({})).unwrap();
    assert_eq!(s.redo.len(), 1, "view commands must preserve an available redo step");
    let document = s.doc.clone();
    let before = (s.undo.len(), s.redo.len(), s.revision, s.evaluations, s.is_dirty());
    let jobs: Vec<_> = (0..10).map(|_| complete_result(&mut s, "Hero")).collect();
    let results = s.render_retained_results().unwrap();
    assert_eq!(results.len(), 8);
    assert_eq!(results.iter().map(|result| result.frame.job).collect::<Vec<_>>(), jobs[2..]);
    for result in &results {
        assert_eq!(result.shot.as_deref(), Some("Hero"));
        assert_eq!((result.frame.width, result.frame.height), (16, 8));
        assert!(result.seconds.is_finite() && result.seconds >= 0.0);
        assert_eq!(result.frame.scene_generation, s.revision);
        assert!(!result.stale);
    }
    assert!(s.render_job_handle(jobs[0]).is_err());
    assert!(s.render_job_handle(jobs[1]).is_err());
    assert_eq!(s.render_selected_result(), Some(jobs[9]));
    assert_eq!(s.execute("render.result.select", &json!({"index":0})).unwrap(), json!({"index":0}));
    assert_eq!(s.render_selected_result(), Some(jobs[2]));
    assert_eq!(s.execute("render.result.copy", &json!({})).unwrap()["frame"]["job"], json!(jobs[2]));
    assert_eq!(s.execute("render.result.pin", &json!({})).unwrap()["frame"]["job"], json!(jobs[2]));
    s.execute("render.result.discard", &json!({"index":0})).unwrap();
    assert_eq!(s.render_selected_result(), Some(jobs[9]));
    assert!(s.render_job_handle(jobs[2]).is_err());
    s.execute("render.result.select", &json!({"index":2})).unwrap();
    s.execute("render.result.discard", &json!({"index":0})).unwrap();
    assert_eq!(s.render_selected_result(), Some(jobs[5]), "discarding an earlier row preserves the selected identity");
    while !s.render_retained_results().unwrap().is_empty() {
        s.execute("render.result.discard", &json!({"index":0})).unwrap();
    }
    assert!(s.render_selected_result().is_none());
    assert!(s.execute("render.result.copy", &json!({})).is_err());
    assert!(std::sync::Arc::ptr_eq(&document, &s.doc));
    assert_eq!(before, (s.undo.len(), s.redo.len(), s.revision, s.evaluations, s.is_dirty()));
}

#[cfg(test)]
#[test]
fn retained_result_requests_refuse_hostile_and_missing_indices_atomically() {
    let mut s = Session::default();
    s.set_render_job_host(Some(solidcraft_render::job::CpuHost::cooperative())).unwrap();
    s.execute("render.settings.set", &json!({"name":"Front","set":{"width":8,"height":8}})).unwrap();
    let job = complete_result(&mut s, "Front");
    let document = s.doc.clone();
    let before = (s.undo.len(), s.redo.len(), s.revision, s.evaluations, s.is_dirty());
    let retained = s.render_job_host().unwrap().admitted_bytes();
    for mode in [crate::schema::ParamCheck::Off, crate::schema::ParamCheck::Strict] {
        s.param_check = mode;
        for command in ["render.result.select", "render.result.discard"] {
            let spec = super::find_command(command).unwrap();
            assert!(spec.contract.is_some() && spec.output.is_some());
            for request in [
                json!(null),
                json!([]),
                json!({}),
                json!({"index":-1}),
                json!({"index":0.5}),
                json!({"index":1e300}),
                json!({"index":u64::MAX}),
                json!({"index":"0"}),
                json!({"index":null}),
                json!({"index":true}),
                json!({"index":1}),
                json!({"index":8}),
                json!({"index":0,"extra":true}),
            ] {
                assert!(s.execute(command, &request).is_err(), "{command}: {request}");
                assert_eq!(s.render_retained_results().unwrap().len(), 1);
                assert_eq!(s.render_selected_result(), Some(job));
                assert_eq!(s.render_job_host().unwrap().admitted_bytes(), retained);
                assert!(std::sync::Arc::ptr_eq(&document, &s.doc));
                assert_eq!(before, (s.undo.len(), s.redo.len(), s.revision, s.evaluations, s.is_dirty()));
            }
        }
    }
    s.execute("render.settings.set", &json!({"name":"Front","set":{"width":16}})).unwrap();
    assert!(s.render_retained_results().unwrap()[0].stale);
    s.execute("edit.undo", &json!({})).unwrap();
    assert!(s.render_retained_results().unwrap()[0].stale, "undo does not recapture pixels");
    assert_eq!(s.render_retained_results().unwrap()[0].frame.scene_generation, before.2);
    let mut other = Session::default();
    assert!(other.render_retained_results().unwrap().is_empty());
    assert!(other.execute("render.result.select", &json!({"index":0})).is_err());
    assert!(other.render_job_handle(job).is_err());
}
