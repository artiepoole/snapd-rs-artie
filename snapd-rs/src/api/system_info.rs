use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SystemInfo {
    pub version: String,
    pub series: String,
    pub architecture: String,
    pub build_id: Option<String>,
    pub on_classic: Option<bool>,
    pub managed: Option<bool>,
    pub kernel_version: Option<String>,
    pub system_mode: Option<String>,
    pub sandbox_features: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEncryptionStatus {
    #[serde(flatten)]
    pub details: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Warning {
    pub message: String,
    pub first_seen: String,
    pub last_seen: String,
    pub expire_after: Option<String>,
}

impl SnapdClient {
    pub async fn get_system_info(&self) -> Result<SystemInfo> {
        self.get("/v2/system-info").await
    }

    pub async fn get_storage_encryption_status(&self) -> Result<StorageEncryptionStatus> {
        self.get("/v2/system-info/storage-encrypted").await
    }

    pub async fn get_warnings(&self) -> Result<Vec<Warning>> {
        self.get("/v2/warnings").await
    }

    pub async fn acknowledge_warnings(&self, timestamp: &str) -> Result<()> {
        self.post_sync(
            "/v2/warnings",
            &json!({
                "action": "okay",
                "timestamp": timestamp,
            }),
        )
        .await
    }
}
