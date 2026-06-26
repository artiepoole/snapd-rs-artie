use serde::{Deserialize, Serialize};

use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SnapctlRequest {
    pub context_id: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapctlResponse {
    pub stdout: String,
    pub stderr: String,
}

impl SnapdClient {
    pub fn run_snapctl(&self, context_id: &str, args: &[&str]) -> Result<SnapctlResponse> {
        let request = SnapctlRequest {
            context_id: context_id.to_string(),
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
        };
        self.post_sync("/v2/snapctl", &request)
    }
}
