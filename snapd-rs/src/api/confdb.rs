use serde_json::Value;

use crate::{client::SnapdClient, error::Result, types::ChangeId};

impl SnapdClient {
    pub async fn get_confdb(&self, account: &str, schema: &str, view: &str) -> Result<Value> {
        self.get(&format!("/v2/confdb/{account}/{schema}/{view}"))
            .await
    }

    pub async fn set_confdb(
        &self,
        account: &str,
        schema: &str,
        view: &str,
        values: Value,
    ) -> Result<ChangeId> {
        self.put(&format!("/v2/confdb/{account}/{schema}/{view}"), &values)
            .await
    }
}
