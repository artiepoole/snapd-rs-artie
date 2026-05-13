use std::path::{Path, PathBuf};

use http_body_util::{BodyExt, Full};
use hyper::{
    body::Bytes,
    header::{CONTENT_TYPE, HOST},
    Method, Request,
};
use hyper_util::rt::TokioIo;
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::UnixStream;

use crate::{
    error::{Error, Result},
    types::{ChangeId, ErrorResult, RawSnapdResponse, ResponseType},
};

pub struct SnapdClient {
    socket_path: PathBuf,
}

impl SnapdClient {
    pub fn new() -> Self {
        Self {
            socket_path: PathBuf::from("/run/snapd.socket"),
        }
    }

    pub fn with_socket(path: impl AsRef<Path>) -> Self {
        Self {
            socket_path: path.as_ref().to_path_buf(),
        }
    }

    async fn send_request(
        &self,
        req: Request<Full<Bytes>>,
    ) -> Result<hyper::Response<hyper::body::Incoming>> {
        let stream = UnixStream::connect(&self.socket_path).await?;
        let io = TokioIo::new(stream);
        let (mut sender, connection) = hyper::client::conn::http1::handshake(io).await?;

        tokio::spawn(async move {
            let _ = connection.await;
        });

        Ok(sender.send_request(req).await?)
    }

    fn build_request(
        &self,
        method: Method,
        path: &str,
        body: Bytes,
        content_type: Option<&str>,
    ) -> Result<Request<Full<Bytes>>> {
        let mut builder = Request::builder()
            .method(method)
            .uri(path)
            .header(HOST, "localhost");

        if let Some(ct) = content_type {
            builder = builder.header(CONTENT_TYPE, ct);
        }

        Ok(builder.body(Full::new(body))?)
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let req = self.build_request(Method::GET, path, Bytes::new(), None)?;
        self.execute_sync(req).await
    }

    pub(crate) async fn post_sync<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let payload = serde_json::to_vec(body)?;
        let req = self.build_request(
            Method::POST,
            path,
            Bytes::from(payload),
            Some("application/json"),
        )?;
        self.execute_sync(req).await
    }

    pub(crate) async fn post_async<B: Serialize>(&self, path: &str, body: &B) -> Result<ChangeId> {
        let payload = serde_json::to_vec(body)?;
        let req = self.build_request(
            Method::POST,
            path,
            Bytes::from(payload),
            Some("application/json"),
        )?;
        self.execute_async(req).await
    }

    pub(crate) async fn put<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let payload = serde_json::to_vec(body)?;
        let req = self.build_request(
            Method::PUT,
            path,
            Bytes::from(payload),
            Some("application/json"),
        )?;
        self.execute_sync(req).await
    }

    pub(crate) async fn post_raw_sync<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Bytes,
        content_type: &'static str,
    ) -> Result<T> {
        let req = self.build_request(Method::POST, path, body, Some(content_type))?;
        self.execute_sync(req).await
    }

    async fn execute_sync<T: DeserializeOwned>(&self, req: Request<Full<Bytes>>) -> Result<T> {
        let response = self.send_request(req).await?;
        let body = response.into_body().collect().await?.to_bytes();
        let envelope: RawSnapdResponse = serde_json::from_slice(&body)?;

        match envelope.response_type {
            ResponseType::Sync => Ok(serde_json::from_value(
                envelope.result.unwrap_or(serde_json::Value::Null),
            )?),
            ResponseType::Error => Err(self.parse_error(&envelope)),
            ResponseType::Async => Err(Error::UnexpectedResponseType("async".to_string())),
        }
    }

    async fn execute_async(&self, req: Request<Full<Bytes>>) -> Result<ChangeId> {
        let response = self.send_request(req).await?;
        let body = response.into_body().collect().await?.to_bytes();
        let envelope: RawSnapdResponse = serde_json::from_slice(&body)?;

        match envelope.response_type {
            ResponseType::Async => match envelope.change {
                Some(change) => Ok(ChangeId(change)),
                None => Err(Error::UnexpectedResponseType(
                    "async response missing change id".to_string(),
                )),
            },
            ResponseType::Error => Err(self.parse_error(&envelope)),
            ResponseType::Sync => Err(Error::UnexpectedResponseType("sync".to_string())),
        }
    }

    fn parse_error(&self, envelope: &RawSnapdResponse) -> Error {
        let error = match &envelope.result {
            Some(value) => {
                serde_json::from_value::<ErrorResult>(value.clone()).unwrap_or_else(|_| {
                    ErrorResult {
                        message: envelope.status.clone(),
                        kind: None,
                    }
                })
            }
            None => ErrorResult {
                message: envelope.status.clone(),
                kind: None,
            },
        };
        Error::Snapd {
            kind: error.kind.unwrap_or_else(|| "unknown".to_string()),
            message: error.message,
        }
    }
}

impl Default for SnapdClient {
    fn default() -> Self {
        Self::new()
    }
}
