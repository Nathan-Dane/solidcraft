//! Transient Browser focus. This request never becomes saved design state.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RenderSelect {
    #[schemars(with = "RenderKind")]
    pub kind: String,
    /// Required only for a light or a shot; bounded to 1024 bytes by the command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RenderKind {
    Nothing,
    Environment,
    Lighting,
    Light,
    Shot,
    Output,
}
