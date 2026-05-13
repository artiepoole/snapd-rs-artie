use crate::{client::SnapdClient, error::Result};

impl SnapdClient {
    pub async fn get_model(&self) -> Result<serde_json::Value> {
        self.get("/v2/model").await
    }

    pub async fn get_serial(&self) -> Result<serde_json::Value> {
        self.get("/v2/model/serial").await
    }
}
