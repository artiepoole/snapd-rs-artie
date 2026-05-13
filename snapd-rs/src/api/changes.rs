use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub id: String,
    pub kind: String,
    pub summary: String,
    pub status: String,
    #[serde(default)]
    pub tasks: Vec<Task>,
    pub err: Option<String>,
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub kind: String,
    pub summary: String,
    pub status: String,
    pub progress: TaskProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskProgress {
    pub done: i64,
    pub total: i64,
}

impl SnapdClient {
    pub async fn list_changes(&self) -> Result<Vec<Change>> {
        self.get("/v2/changes").await
    }

    pub async fn get_change(&self, id: &str) -> Result<Change> {
        self.get(&format!("/v2/changes/{id}")).await
    }

    pub async fn abort_change(&self, id: &str) -> Result<Change> {
        self.post_sync(&format!("/v2/changes/{id}"), &json!({ "action": "abort" }))
            .await
    }
}
