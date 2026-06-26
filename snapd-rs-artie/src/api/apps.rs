use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    client::SnapdClient,
    error::Result,
    types::{ChangeId, DaemonScope, DaemonType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppInfo {
    pub snap: Option<String>,
    pub name: String,
    pub desktop_file: Option<String>,
    pub daemon: Option<DaemonType>,
    pub daemon_scope: Option<DaemonScope>,
    pub enabled: Option<bool>,
    pub active: Option<bool>,
    pub common_id: Option<String>,
}

impl SnapdClient {
    pub fn list_apps(&self) -> Result<Vec<AppInfo>> {
        self.get("/v2/apps")
    }

    pub fn start_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async("/v2/apps", &json!({ "action": "start", "names": services }))
    }

    pub fn stop_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async("/v2/apps", &json!({ "action": "stop", "names": services }))
    }

    pub fn restart_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/apps",
            &json!({ "action": "restart", "names": services }),
        )
    }

    pub fn enable_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/apps",
            &json!({ "action": "start", "names": services, "enable": true }),
        )
    }

    pub fn disable_service(&self, services: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/apps",
            &json!({ "action": "stop", "names": services, "disable": true }),
        )
    }

    /// List services (daemon apps) for a specific snap.
    pub fn list_snap_services(&self, snap_name: &str) -> Result<Vec<AppInfo>> {
        let apps: Vec<AppInfo> = self.get(&format!("/v2/apps?names={snap_name}"))?;
        Ok(apps.into_iter().filter(|a| a.daemon.is_some()).collect())
    }
}
