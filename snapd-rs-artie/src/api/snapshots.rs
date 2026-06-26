use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::SnapdClient, error::Result, types::ChangeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: u64,
    #[serde(default)]
    pub snaps: Vec<String>,
    pub time: String,
    pub size: u64,
}

impl SnapdClient {
    pub fn list_snapshots(&self) -> Result<Vec<Snapshot>> {
        self.get("/v2/snapshots")
    }

    pub fn create_snapshot(&self, snaps: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/snaps",
            &json!({ "action": "snapshot", "snaps": snaps }),
        )
    }

    pub fn restore_snapshot(&self, set_id: u64, snaps: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/snapshots",
            &json!({ "action": "restore", "set": set_id, "snaps": snaps }),
        )
    }

    pub fn forget_snapshot(&self, set_id: u64, snaps: &[&str]) -> Result<ChangeId> {
        self.post_async(
            "/v2/snapshots",
            &json!({ "action": "forget", "set": set_id, "snaps": snaps }),
        )
    }
}
