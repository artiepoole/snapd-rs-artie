use serde_json::{Value, json};

use crate::{client::SnapdClient, error::Result, types::ChangeId};

impl SnapdClient {
    pub async fn list_system_volumes(&self) -> Result<Value> {
        self.get("/v2/system-volumes").await
    }

    pub async fn system_volumes_action(&self, action: &str, params: Value) -> Result<ChangeId> {
        self.post_async(
            "/v2/system-volumes",
            &json!({
                "action": action,
                "params": params,
            }),
        )
        .await
    }
}
