//! Transient Render Browser focus: no document or saved-format state.
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;

/// Validated request and echoed result of `view.render_select`.
#[derive(Debug, Serialize)]
pub struct RenderSelect {
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl<'de> Deserialize<'de> for RenderSelect {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Request {
            kind: String,
            id: Option<String>,
        }
        let request = Request::deserialize(d)?;
        let identified = matches!(request.kind.as_str(), "light" | "shot");
        if !matches!(request.kind.as_str(), "nothing" | "environment" | "light" | "shot" | "output" | "wear")
            || identified != request.id.is_some()
            || request.id.as_ref().is_some_and(|id| id.trim().is_empty() || id.len() > 1024)
        {
            return Err(serde::de::Error::custom("expected a Browser kind and an id only for light or shot"));
        }
        Ok(Self { kind: request.kind, id: request.id })
    }
}

impl JsonSchema for RenderSelect {
    fn schema_name() -> Cow<'static, str> {
        "RenderSelect".into()
    }
    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "oneOf": [
                {
                    "type": "object",
                    "properties": {
                        "kind": {"type": "string", "enum": ["nothing", "environment", "output", "wear"]},
                        "id": {"type": "null"}
                    },
                    "required": ["kind"],
                    "additionalProperties": false
                },
                {
                    "type": "object",
                    "properties": {
                        "kind": {"type": "string", "enum": ["light", "shot"]},
                        "id": {"type": "string", "minLength": 1, "maxLength": 1024, "pattern": super::types::not_blank_pattern(),
                               "description": "A nonblank identity, at most 1024 UTF-8 bytes."}
                    },
                    "required": ["kind", "id"],
                    "additionalProperties": false
                }
            ]
        })
    }
}
