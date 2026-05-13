use hyper::body::Bytes;

use crate::{client::SnapdClient, error::Result};

impl SnapdClient {
    pub async fn list_assertion_types(&self) -> Result<Vec<String>> {
        self.get("/v2/assertions").await
    }

    pub async fn add_assertion(&self, assertion: &str) -> Result<()> {
        self.post_raw_sync(
            "/v2/assertions",
            Bytes::from(assertion.to_owned()),
            "text/plain",
        )
        .await
    }

    pub async fn get_assertions(&self, assert_type: &str) -> Result<Vec<serde_json::Value>> {
        self.get(&format!("/v2/assertions/{assert_type}")).await
    }
}
