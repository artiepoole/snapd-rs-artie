use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result, types::ChangeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QuotaGroup {
    pub group_name: String,
    pub parent: Option<String>,
    #[serde(default)]
    pub subgroups: Vec<String>,
    #[serde(default)]
    pub snaps: Vec<String>,
    pub constraints: Value,
}

impl SnapdClient {
    pub async fn list_quotas(&self) -> Result<Vec<QuotaGroup>> {
        self.get("/v2/quotas").await
    }

    pub async fn get_quota(&self, group: &str) -> Result<QuotaGroup> {
        self.get(&format!("/v2/quotas/{group}")).await
    }

    pub async fn ensure_quota(&self, group: &str, constraints: Value) -> Result<ChangeId> {
        self.post_async(
            "/v2/quotas",
            &json!({
                "action": "ensure",
                "group-name": group,
                "constraints": constraints,
            }),
        )
        .await
    }

    pub async fn remove_quota(&self, group: &str) -> Result<ChangeId> {
        self.post_async(
            "/v2/quotas",
            &json!({
                "action": "remove",
                "group-name": group,
            }),
        )
        .await
    }
}
