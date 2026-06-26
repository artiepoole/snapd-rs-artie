use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::SnapdClient, error::Result, types::ValidationSetMode};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ValidationSet {
    pub account_id: String,
    pub name: String,
    pub pinned_at: Option<i64>,
    pub mode: ValidationSetMode,
    pub sequence: Option<i64>,
    pub valid: bool,
}

#[derive(Debug, Serialize)]
struct ApplyValidationSetRequest {
    action: &'static str,
    mode: ValidationSetMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence: Option<i64>,
}

impl SnapdClient {
    pub fn list_validation_sets(&self) -> Result<Vec<ValidationSet>> {
        self.get("/v2/validation-sets")
    }

    pub fn get_validation_set(&self, account: &str, name: &str) -> Result<ValidationSet> {
        self.get(&format!("/v2/validation-sets/{account}/{name}"))
    }

    pub fn apply_validation_set(
        &self,
        account: &str,
        name: &str,
        mode: ValidationSetMode,
        sequence: Option<i64>,
    ) -> Result<ValidationSet> {
        self.post_sync(
            &format!("/v2/validation-sets/{account}/{name}"),
            &ApplyValidationSetRequest {
                action: "apply",
                mode,
                sequence,
            },
        )
    }

    pub fn forget_validation_set(&self, account: &str, name: &str) -> Result<()> {
        self.post_sync(
            &format!("/v2/validation-sets/{account}/{name}"),
            &json!({ "action": "forget" }),
        )
    }
}
