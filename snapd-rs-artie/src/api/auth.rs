use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{client::SnapdClient, error::Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub macaroon: String,
    #[serde(default)]
    pub discharges: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    #[serde(default)]
    pub ssh_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub sudoer: bool,
    pub known: bool,
}

impl SnapdClient {
    pub async fn login(
        &self,
        email: &str,
        password: &str,
        otp: Option<&str>,
    ) -> Result<LoginResponse> {
        self.post_sync(
            "/v2/login",
            &json!({
                "email": email,
                "password": password,
                "otp": otp,
            }),
        )
        .await
    }

    pub async fn logout(&self) -> Result<()> {
        self.post_sync("/v2/logout", &json!({})).await
    }

    pub async fn list_users(&self) -> Result<Vec<UserInfo>> {
        self.get("/v2/users").await
    }

    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<UserInfo> {
        self.post_sync(
            "/v2/users",
            &json!({
                "action": "create",
                "email": request.email,
                "sudoer": request.sudoer,
                "known": request.known,
            }),
        )
        .await
    }

    pub async fn remove_user(&self, username: &str) -> Result<()> {
        self.post_sync(
            "/v2/users",
            &json!({
                "action": "remove",
                "username": username,
            }),
        )
        .await
    }
}
