use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    client::SnapdClient,
    error::Result,
    types::{AliasStatusKind, ChangeId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasStatus {
    pub command: String,
    pub status: AliasStatusKind,
    pub manual: Option<String>,
    pub auto: Option<String>,
}

impl SnapdClient {
    pub fn list_aliases(&self) -> Result<HashMap<String, HashMap<String, AliasStatus>>> {
        self.get("/v2/aliases")
    }

    pub fn set_alias(&self, snap: &str, app: &str, alias: &str) -> Result<ChangeId> {
        self.post_async(
            "/v2/aliases",
            &json!({
                "action": "alias",
                "snap": snap,
                "app": app,
                "alias": alias,
            }),
        )
    }

    pub fn remove_alias(&self, snap: &str, alias: &str) -> Result<ChangeId> {
        self.post_async(
            "/v2/aliases",
            &json!({
                "action": "unalias",
                "snap": snap,
                "alias": alias,
            }),
        )
    }

    pub fn prefer_aliases(&self, snap: &str) -> Result<ChangeId> {
        self.post_async("/v2/aliases", &json!({ "action": "prefer", "snap": snap }))
    }
}
