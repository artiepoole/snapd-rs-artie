use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};
use url::form_urlencoded::Serializer;

use crate::{client::SnapdClient, error::Result, types::NoticeType};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ListNoticesOptions {
    pub types: Vec<NoticeType>,
    pub keys: Vec<String>,
    pub after: Option<OffsetDateTime>,
    pub timeout: Option<Duration>,
    pub user_filter: NoticeUserFilter,
}

impl ListNoticesOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_type(mut self, notice_type: NoticeType) -> Self {
        self.types.push(notice_type);
        self
    }

    pub fn with_types<I>(mut self, notice_types: I) -> Self
    where
        I: IntoIterator<Item = NoticeType>,
    {
        self.types.extend(notice_types);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.keys.push(key.into());
        self
    }

    pub fn with_keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.keys.extend(keys.into_iter().map(Into::into));
        self
    }

    pub fn after(mut self, timestamp: OffsetDateTime) -> Self {
        self.after = Some(timestamp);
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn user_id(mut self, user_id: u32) -> Self {
        self.user_filter = NoticeUserFilter::UserId(user_id);
        self
    }

    pub fn all_users(mut self) -> Self {
        self.user_filter = NoticeUserFilter::AllUsers;
        self
    }

    fn to_query_string(&self) -> Result<String> {
        let mut serializer = Serializer::new(String::new());

        if !self.types.is_empty() {
            serializer.append_pair(
                "types",
                &self
                    .types
                    .iter()
                    .map(NoticeType::as_str)
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }

        if !self.keys.is_empty() {
            serializer.append_pair("keys", &self.keys.join(","));
        }

        if let Some(after) = self.after {
            serializer.append_pair("after", &after.format(&Rfc3339)?);
        }

        if let Some(timeout) = self.timeout {
            serializer.append_pair("timeout", &format_duration(timeout));
        }

        match self.user_filter {
            NoticeUserFilter::CurrentUser => {}
            NoticeUserFilter::UserId(user_id) => {
                serializer.append_pair("user-id", &user_id.to_string());
            }
            NoticeUserFilter::AllUsers => {
                serializer.append_pair("users", "all");
            }
        }

        Ok(serializer.finish())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NoticeUserFilter {
    #[default]
    CurrentUser,
    UserId(u32),
    AllUsers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Notice {
    pub id: String,
    pub user_id: Option<u64>,
    #[serde(rename = "type")]
    pub type_: NoticeType,
    pub key: String,
    #[serde(with = "time::serde::rfc3339")]
    pub first_occurred: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub last_occurred: OffsetDateTime,
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub last_repeated: Option<OffsetDateTime>,
    pub occurrences: u64,
    #[serde(default)]
    pub last_data: HashMap<String, String>,
    pub expire_after: Option<String>,
    pub repeat_after: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AddNoticeResponse {
    pub id: String,
}

impl SnapdClient {
    pub fn list_notices(&self) -> Result<Vec<Notice>> {
        self.list_notices_with(&ListNoticesOptions::new())
    }

    pub fn list_notices_with(&self, options: &ListNoticesOptions) -> Result<Vec<Notice>> {
        let query = options.to_query_string()?;
        let path = if query.is_empty() {
            "/v2/notices".to_string()
        } else {
            format!("/v2/notices?{query}")
        };
        self.get(&path)
    }

    pub fn get_notice(&self, id: &str) -> Result<Notice> {
        self.get(&format!("/v2/notices/{id}"))
    }

    pub fn add_notice(
        &self,
        notice_type: NoticeType,
        key: &str,
        data: Option<HashMap<String, String>>,
    ) -> Result<String> {
        let response: AddNoticeResponse = self.post_sync(
            "/v2/notices",
            &json!({
                "action": "add",
                "type": notice_type,
                "key": key,
                "data": data,
            }),
        )?;
        Ok(response.id)
    }
}

fn format_duration(duration: Duration) -> String {
    if duration.is_zero() {
        return "0s".to_string();
    }

    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let nanos = duration.subsec_nanos();

    let mut formatted = String::new();
    if hours > 0 {
        formatted.push_str(&format!("{hours}h"));
    }
    if minutes > 0 {
        formatted.push_str(&format!("{minutes}m"));
    }
    if seconds > 0 || nanos > 0 {
        if nanos == 0 {
            formatted.push_str(&format!("{seconds}s"));
        } else {
            let mut fractional = format!("{nanos:09}");
            while fractional.ends_with('0') {
                fractional.pop();
            }
            formatted.push_str(&format!("{seconds}.{fractional}s"));
        }
    }

    formatted
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serde_json::json;
    use time::OffsetDateTime;

    use super::*;

    #[test]
    fn list_notices_options_build_query_string() {
        let after = OffsetDateTime::parse("2026-05-13T16:00:00.123456789Z", &Rfc3339).unwrap();
        let options = ListNoticesOptions::new()
            .with_type(NoticeType::InterfacesRequestsPrompt)
            .with_type(NoticeType::Warning)
            .with_key("123")
            .with_key("abc")
            .after(after)
            .timeout(Duration::new(90, 250_000_000))
            .user_id(1000);

        let query = options.to_query_string().unwrap();

        assert_eq!(
            query,
            "types=interfaces-requests-prompt%2Cwarning&keys=123%2Cabc&after=2026-05-13T16%3A00%3A00.123456789Z&timeout=1m30.25s&user-id=1000"
        );
    }

    #[test]
    fn notices_deserialize_rfc3339_timestamps() {
        let notice: Notice = serde_json::from_value(json!({
            "id": "1",
            "user-id": 1000,
            "type": "interfaces-requests-prompt",
            "key": "42",
            "first-occurred": "2026-05-13T16:00:00.123456789Z",
            "last-occurred": "2026-05-13T16:00:01.123456789Z",
            "last-repeated": "2026-05-13T16:00:02.123456789Z",
            "occurrences": 1,
            "last-data": {
                "resolved": "expired"
            }
        }))
        .unwrap();

        assert_eq!(notice.key, "42");
        assert_eq!(
            notice.last_repeated.unwrap(),
            OffsetDateTime::parse("2026-05-13T16:00:02.123456789Z", &Rfc3339).unwrap()
        );
    }
}
