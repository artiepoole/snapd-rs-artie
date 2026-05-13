use serde_json::json;

use crate::{client::SnapdClient, error::Result};

impl SnapdClient {
    pub async fn get_recovery_keys(&self) -> Result<serde_json::Value> {
        self.get("/v2/system-recovery-keys").await
    }

    pub async fn remove_recovery_keys(&self) -> Result<()> {
        self.post_sync("/v2/system-recovery-keys", &json!({ "action": "remove" }))
            .await
    }
}
