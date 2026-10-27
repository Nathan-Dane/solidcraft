//! Clipboard requests for the Render workspace; headless calls only echo validated text.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Transient Browser focus; only lights and shots carry an identity.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RenderSelect {
    #[schemars(regex(pattern = "^(nothing|environment|light|shot|output|sampling|effects|check)$"))]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(length(min = 1, max = 1024))]
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CheckReport {
    /// Report text, bounded to 65536 UTF-8 bytes by the command handler.
    #[schemars(length(min = 1, max = 65536))]
    pub text: String,
}
