use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result, types::ChangeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySystem {
    pub label: String,
    pub model: Value,
    pub brand: Value,
    #[serde(default)]
    pub actions: Vec<SystemAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAction {
    pub title: String,
    pub mode: String,
}

impl SnapdClient {
    pub async fn list_systems(&self) -> Result<Vec<RecoverySystem>> {
        self.get("/v2/systems").await
    }

    pub async fn get_system(&self, label: &str) -> Result<RecoverySystem> {
        self.get(&format!("/v2/systems/{label}")).await
    }

    pub async fn reboot_into_system(&self, label: &str, mode: &str) -> Result<ChangeId> {
        self.post_async(
            &format!("/v2/systems/{label}"),
            &json!({ "action": "reboot", "mode": mode }),
        )
        .await
    }
}
