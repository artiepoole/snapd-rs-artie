use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::SnapdClient, error::Result, types::ChangeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppInfo {
    pub snap: Option<String>,
    pub name: String,
    pub desktop_file: Option<String>,
    pub daemon: Option<String>,
    pub daemon_scope: Option<String>,
    pub enabled: Option<bool>,
    pub active: Option<bool>,
    pub common_id: Option<String>,
}

impl SnapdClient {
    pub async fn list_apps(&self) -> Result<Vec<AppInfo>> {
        self.get("/v2/apps").await
    }

    pub async fn start_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async("/v2/apps", &json!({ "action": "start", "names": services }))
            .await
    }

    pub async fn stop_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async("/v2/apps", &json!({ "action": "stop", "names": services }))
            .await
    }

    pub async fn restart_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/apps",
            &json!({ "action": "restart", "names": services }),
        )
        .await
    }
}
