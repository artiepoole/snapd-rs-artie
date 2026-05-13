use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result};

impl SnapdClient {
    pub async fn get_debug_info(&self, aspect: &str) -> Result<Value> {
        self.get(&format!("/v2/debug?aspect={aspect}")).await
    }

    pub async fn debug_action(&self, action: &str, params: Value) -> Result<Value> {
        self.post_sync(
            "/v2/debug",
            &json!({
                "action": action,
                "params": params,
            }),
        )
        .await
    }
}
