//! Contracts for transient retained-result requests.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Select or discard a retained result without editing a design.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct ResultIndex {
    /// Zero-based index, oldest first. The result must still be retained.
    #[schemars(range(min = 0, max = 7))]
    pub index: u32,
}
