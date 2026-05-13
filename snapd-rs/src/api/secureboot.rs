use serde_json::json;

use crate::{client::SnapdClient, error::Result, types::ChangeId};

impl SnapdClient {
    pub async fn secureboot_action(&self, action: &str) -> Result<ChangeId> {
        self.post_async("/v2/system-secureboot", &json!({ "action": action }))
            .await
    }
}
