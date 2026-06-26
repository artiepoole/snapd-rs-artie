use serde::Deserialize;

use crate::api::model::AssertionJson;
use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Deserialize)]
struct AssertionTypesResponse {
    #[serde(default)]
    types: Vec<String>,
}

impl SnapdClient {
    pub fn list_assertion_types(&self) -> Result<Vec<String>> {
        let response: AssertionTypesResponse = self.get("/v2/assertions")?;
        Ok(response.types)
    }

    pub fn add_assertion(&self, assertion: &str) -> Result<()> {
        self.post_raw_sync("/v2/assertions", assertion.as_bytes(), "text/plain")
    }

    pub fn get_assertions(&self, assert_type: &str) -> Result<Vec<AssertionJson>> {
        self.get(&format!("/v2/assertions/{assert_type}?json=true"))
    }
}
