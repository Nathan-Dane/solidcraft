//! Planned Render interactions, with explicit non-mutating refusals.
use super::CommandSpec;

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
    CommandSpec::new("appearance.paint", "Paint Appearance", super::inspect::appearance_gestures::paint)
        .icon("appearance").key("B").noundo()
        .params("appearance or color/name/opacity/pbr: appearance.assign look parameters; validates and returns look for ui.start paint. done?: true ends the UI tool. Programmatic calls never open UI."),
    CommandSpec::new("appearance.pick_up", "Pick Up Appearance", super::inspect::appearance_gestures::pick_up)
        .icon("select").key("I").noundo()
        .params("target?: appearance.chain target; returns its resolved look, or requests the UI sampling tool when omitted. Programmatic calls never open UI."),

    waiting!("render.lights.solo", "Solo Light", "RENDER SCENE", "LIGHTS", "render_scene", "light visibility and solo controls"),
    waiting!("render.compare", "Compare Images", "RENDER", "RESULT", "compare", "retained image comparison"),
    waiting!("render.batch", "Render All Shots", "RENDER", "BATCH", "render_image", "the saved-shot render queue"),
];

#[cfg(test)]
#[test]
fn render_workspace_planned_commands_refuse_without_mutation() {
    use serde_json::{Value, json};
    let mut session = crate::Session::default();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":800}})).unwrap();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":900}})).unwrap();
    session.execute("edit.undo", &json!({})).unwrap();
    let document = session.doc.clone();
    let before = (session.undo.len(), session.redo.len(), session.revision, session.evaluations, session.is_dirty());
    for command in COMMANDS.iter().filter(|command| command.planned.is_some()) {
        assert!(command.planned.is_some());
        assert!(!command.info(&session).enabled);
        for params in [json!({}), Value::Null, json!([]), json!({"target":1e300,"name":"hostile"})] {
            let error = session.execute(command.id, &params).unwrap_err().to_string();
            assert!(error.contains("not supported yet"), "{error}");
            assert!(std::sync::Arc::ptr_eq(&document, &session.doc));
            assert_eq!(before, (session.undo.len(), session.redo.len(), session.revision, session.evaluations, session.is_dirty()));
        }
    }
}
