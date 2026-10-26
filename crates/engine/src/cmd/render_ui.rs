//! Planned Render interactions, with explicit non-mutating refusals.
use super::CommandSpec;
use crate::schema::render_lights::{MuteParams, SoloParams};

const SOLO: crate::schema::TypedContract<SoloParams> = crate::schema::TypedContract::new();
const MUTE: crate::schema::TypedContract<MuteParams> = crate::schema::TypedContract::new();

fn solo(_: &mut crate::Session, params: &serde_json::Value) -> crate::Result<serde_json::Value> {
    let command = "render.lights.solo";
    if !params.is_object() {
        return Err(crate::params::bad(command, "expected an object"));
    }
    let input: SoloParams = serde_json::from_value(params.clone()).map_err(|e| crate::params::bad(command, e.to_string()))?;
    if input.solo && input.id.is_none() {
        return Err(crate::params::bad(command, "solo requires a light id"));
    }
    if let Some(id) = &input.id {
        crate::render_lights::parse_id(id).map_err(|e| crate::params::bad(command, e))?;
    }
    serde_json::to_value(input).map_err(|e| crate::params::bad(command, e.to_string()))
}

fn mute(_: &mut crate::Session, params: &serde_json::Value) -> crate::Result<serde_json::Value> {
    let command = "render.lights.mute";
    if !params.is_object() {
        return Err(crate::params::bad(command, "expected an object"));
    }
    let input: MuteParams = serde_json::from_value(params.clone()).map_err(|e| crate::params::bad(command, e.to_string()))?;
    crate::render_lights::parse_id(&input.id).map_err(|e| crate::params::bad(command, e))?;
    serde_json::to_value(input).map_err(|e| crate::params::bad(command, e.to_string()))
}

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
    CommandSpec::new("render.lights.solo","Solo Light",crate::checked!("render.lights.solo",solo))
        .noundo().doc_only().icon("render_scene").at("RENDER SCENE","LIGHTS")
        .contract(&SOLO).output(crate::schema::output_schema_for::<SoloParams>)
        .params("id?: kind:slot light address; solo:bool. Enabling requires an id; false clears solo. Validates and echoes per-document UI view state. Never saves settings, evaluates geometry, opens an editor or changes history."),
    CommandSpec::new("render.lights.mute","Mute Light",crate::checked!("render.lights.mute",mute))
        .noundo().doc_only().icon("render_scene").at("RENDER SCENE","LIGHTS")
        .contract(&MUTE).output(crate::schema::output_schema_for::<MuteParams>)
        .params("id: kind:slot light address; mute:bool. Validates and echoes per-document UI view state, applied to live preview and captured render copies only. Never changes saved settings or history."),
    waiting!("appearance.paint", "Paint Appearance", "RENDER APPEARANCE", "APPEARANCE", "appearance", "product picking and appearance painting"),
    waiting!("appearance.pick_up", "Pick Up Appearance", "RENDER APPEARANCE", "APPEARANCE", "select", "product picking and appearance sampling"),
    waiting!("appearance.replace", "Replace Appearance", "RENDER APPEARANCE", "APPEARANCE", "appearance", "replacing all uses of an appearance"),
    waiting!("render.compare", "Compare Images", "RENDER", "RESULT", "compare", "retained image comparison"),
    waiting!("render.batch", "Render All Shots", "RENDER", "BATCH", "render_image", "the saved-shot render queue"),
];

#[cfg(test)]
#[test]
fn render_lights_remaining_planned_commands_refuse_without_mutation() {
    use serde_json::{Value, json};
    let mut session = crate::Session::default();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":800}})).unwrap();
    session.execute("render.settings.set", &json!({"name":"Studio","set":{"width":900}})).unwrap();
    session.execute("edit.undo", &json!({})).unwrap();
    let document = session.doc.clone();
    let before = (session.undo.len(), session.redo.len(), session.revision, session.evaluations, session.is_dirty());
    for command in COMMANDS.iter().filter(|command| command.planned.is_some()) {
        let Some(command) = crate::find_command(command.id).filter(|command| command.planned.is_some()) else {
            continue;
        };
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
