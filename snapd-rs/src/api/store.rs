use serde::{Deserialize, Serialize};

use crate::{
    client::SnapdClient,
    error::Result,
    types::{Revision, SnapType},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StoreSnap {
    pub id: Option<String>,
    pub name: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub publisher: Option<String>,
    pub developer: Option<String>,
    pub version: Option<String>,
    pub channel: Option<String>,
    pub revision: Option<Revision>,
    #[serde(rename = "type")]
    pub type_: Option<SnapType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
}

impl SnapdClient {
    pub async fn find_snaps(&self, query: &str) -> Result<Vec<StoreSnap>> {
        self.get(&format!("/v2/find?q={query}")).await
    }

    pub async fn list_categories(&self) -> Result<Vec<Category>> {
        self.get("/v2/categories").await
    }
}
