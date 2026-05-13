use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{client::SnapdClient, error::Result, types::ChangeId};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ThemeStatus {
    #[serde(default)]
    pub gtk_themes: Value,
    #[serde(default)]
    pub icon_themes: Value,
    #[serde(default)]
    pub sound_themes: Value,
}

fn build_theme_path(gtk_themes: &[&str], icon_themes: &[&str], sound_themes: &[&str]) -> String {
    let mut params = Vec::new();
    if !gtk_themes.is_empty() {
        params.push(format!("gtk-themes={}", gtk_themes.join(",")));
    }
    if !icon_themes.is_empty() {
        params.push(format!("icon-themes={}", icon_themes.join(",")));
    }
    if !sound_themes.is_empty() {
        params.push(format!("sound-themes={}", sound_themes.join(",")));
    }
    if params.is_empty() {
        "/v2/accessories/themes".to_string()
    } else {
        format!("/v2/accessories/themes?{}", params.join("&"))
    }
}

impl SnapdClient {
    pub async fn get_theme_status(
        &self,
        gtk_themes: &[&str],
        icon_themes: &[&str],
        sound_themes: &[&str],
    ) -> Result<ThemeStatus> {
        let path = build_theme_path(gtk_themes, icon_themes, sound_themes);
        self.get(&path).await
    }

    pub async fn install_themes(
        &self,
        gtk_themes: &[&str],
        icon_themes: &[&str],
        sound_themes: &[&str],
    ) -> Result<ChangeId> {
        self.post_async(
            "/v2/accessories/themes",
            &json!({
                "gtk-themes": gtk_themes,
                "icon-themes": icon_themes,
                "sound-themes": sound_themes,
            }),
        )
        .await
    }
}
