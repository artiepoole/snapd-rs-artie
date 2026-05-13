use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub timestamp: String,
    pub snap: String,
    pub interface: String,
    pub constraints: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptRule {
    pub id: String,
    pub timestamp: String,
    pub user: String,
    pub snap: String,
    pub interface: String,
    pub constraints: Value,
    pub outcome: String,
    pub lifespan: String,
    pub expiration: Option<String>,
}

impl SnapdClient {
    pub async fn list_prompts(&self) -> Result<Vec<Prompt>> {
        self.get("/v2/interfaces/requests/prompts").await
    }

    pub async fn get_prompt(&self, id: &str) -> Result<Prompt> {
        self.get(&format!("/v2/interfaces/requests/prompts/{id}"))
            .await
    }

    pub async fn reply_to_prompt(
        &self,
        id: &str,
        outcome: &str,
        lifespan: &str,
        constraints: Value,
    ) -> Result<Vec<Prompt>> {
        self.post_sync(
            &format!("/v2/interfaces/requests/prompts/{id}"),
            &json!({
                "outcome": outcome,
                "lifespan": lifespan,
                "constraints": constraints,
            }),
        )
        .await
    }

    pub async fn list_prompt_rules(&self) -> Result<Vec<PromptRule>> {
        self.get("/v2/interfaces/requests/rules").await
    }

    pub async fn add_prompt_rule(&self, rule: Value) -> Result<PromptRule> {
        self.post_sync("/v2/interfaces/requests/rules", &rule).await
    }

    pub async fn get_prompt_rule(&self, id: &str) -> Result<PromptRule> {
        self.get(&format!("/v2/interfaces/requests/rules/{id}"))
            .await
    }

    pub async fn patch_prompt_rule(&self, id: &str, constraints: Value) -> Result<PromptRule> {
        self.post_sync(
            &format!("/v2/interfaces/requests/rules/{id}"),
            &json!({ "constraints": constraints }),
        )
        .await
    }

    pub async fn remove_prompt_rule(&self, id: &str) -> Result<()> {
        self.post_sync(
            &format!("/v2/interfaces/requests/rules/{id}"),
            &json!({ "action": "remove" }),
        )
        .await
    }
}
