use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use serde::{Serialize, de::DeserializeOwned};

use crate::{
    error::{Error, Result},
    types::{ChangeId, ErrorResult, RawSnapdResponse, ResponseType},
};

/// How the client connects to the snapd daemon.
#[derive(Debug, Clone)]
pub enum SocketAddress {
    /// A filesystem path to a Unix socket (e.g. `/run/snapd.socket`).
    Filesystem(PathBuf),
    /// An abstract Unix socket name (without the leading null byte).
    /// Used when connecting from inside snap confinement.
    Abstract(String),
}

/// The default socket used by unconfined host processes.
pub const SNAPD_SOCKET: &str = "/run/snapd.socket";

/// The socket used by snaps connecting to snapd (less-privileged).
/// This is a filesystem socket that snapd listens on for confined snap clients.
pub const SNAPD_SNAP_SOCKET: &str = "/run/snapd-snap.socket";

#[derive(Debug, Clone)]
pub struct SnapdClient {
    socket: SocketAddress,
}

impl SnapdClient {
    /// Create a client that connects via the default snapd socket
    /// (`/run/snapd.socket`) for unconfined host processes.
    pub fn new() -> Self {
        Self {
            socket: SocketAddress::Filesystem(PathBuf::from(SNAPD_SOCKET)),
        }
    }

    /// Create a client that connects via the less-privileged snap socket
    /// (`/run/snapd-snap.socket`) used by confined snaps communicating
    /// with snapd over a filesystem path.
    pub fn new_for_snap() -> Self {
        Self {
            socket: SocketAddress::Filesystem(PathBuf::from(SNAPD_SNAP_SOCKET)),
        }
    }

    /// Create a client that connects via an abstract Unix socket.
    /// The name should not include the leading null byte — it is added
    /// automatically. Use this when connecting from within snap
    /// confinement where the filesystem socket is not accessible.
    pub fn new_abstract(name: impl Into<String>) -> Self {
        Self {
            socket: SocketAddress::Abstract(name.into()),
        }
    }

    /// Create a client with an explicit filesystem socket path.
    pub fn with_socket(path: impl AsRef<Path>) -> Self {
        Self {
            socket: SocketAddress::Filesystem(path.as_ref().to_path_buf()),
        }
    }

    fn connect(&self) -> Result<UnixStream> {
        match &self.socket {
            SocketAddress::Filesystem(path) => Ok(UnixStream::connect(path)?),
            SocketAddress::Abstract(name) => {
                use std::os::linux::net::SocketAddrExt;
                let addr = std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes())
                    .map_err(|e| Error::Connection(format!("invalid abstract socket name: {e}")))?;
                UnixStream::connect_addr(&addr).map_err(|e| {
                    Error::Connection(format!("failed to connect to abstract socket @{name}: {e}"))
                })
            }
        }
    }

    /// Send an HTTP/1.1 request over the snapd Unix socket and return the
    /// response body bytes. A fresh connection is opened per request and the
    /// `Connection: close` header asks snapd to close it once the response has
    /// been written.
    fn send_request(
        &self,
        method: &str,
        path: &str,
        body: &[u8],
        content_type: Option<&str>,
    ) -> Result<Vec<u8>> {
        let mut stream = self.connect()?;

        let mut head = format!("{method} {path} HTTP/1.1\r\nHost: localhost\r\n");
        if let Some(ct) = content_type {
            head.push_str(&format!("Content-Type: {ct}\r\n"));
        }
        head.push_str(&format!("Content-Length: {}\r\n", body.len()));
        head.push_str("Connection: close\r\n\r\n");

        stream.write_all(head.as_bytes())?;
        if !body.is_empty() {
            stream.write_all(body)?;
        }
        stream.flush()?;

        read_response(stream)
    }

    pub(crate) fn get_bytes(&self, path: &str) -> Result<Vec<u8>> {
        self.send_request("GET", path, &[], None)
    }

    pub(crate) fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let bytes = self.send_request("GET", path, &[], None)?;
        self.parse_sync(&bytes)
    }

    pub(crate) fn get_async(&self, path: &str) -> Result<ChangeId> {
        let bytes = self.send_request("GET", path, &[], None)?;
        self.parse_async(&bytes)
    }

    pub(crate) fn post_sync<B: Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let payload = serde_json::to_vec(body)?;
        let bytes = self.send_request("POST", path, &payload, Some("application/json"))?;
        self.parse_sync(&bytes)
    }

    pub(crate) fn post_async<B: Serialize>(&self, path: &str, body: &B) -> Result<ChangeId> {
        let payload = serde_json::to_vec(body)?;
        let bytes = self.send_request("POST", path, &payload, Some("application/json"))?;
        self.parse_async(&bytes)
    }

    pub(crate) fn post_multipart_async(
        &self,
        path: &str,
        body: &[u8],
        content_type: &str,
    ) -> Result<ChangeId> {
        let bytes = self.send_request("POST", path, body, Some(content_type))?;
        self.parse_async(&bytes)
    }

    pub(crate) fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T> {
        let payload = serde_json::to_vec(body)?;
        let bytes = self.send_request("PUT", path, &payload, Some("application/json"))?;
        self.parse_sync(&bytes)
    }

    pub(crate) fn put_async<B: Serialize>(&self, path: &str, body: &B) -> Result<ChangeId> {
        let payload = serde_json::to_vec(body)?;
        let bytes = self.send_request("PUT", path, &payload, Some("application/json"))?;
        self.parse_async(&bytes)
    }

    pub(crate) fn post_raw_sync<T: DeserializeOwned>(
        &self,
        path: &str,
        body: &[u8],
        content_type: &'static str,
    ) -> Result<T> {
        let bytes = self.send_request("POST", path, body, Some(content_type))?;
        self.parse_sync(&bytes)
    }

    fn parse_sync<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T> {
        let envelope: RawSnapdResponse = serde_json::from_slice(bytes)?;

        match envelope.response_type {
            ResponseType::Sync => Ok(serde_json::from_value(
                envelope.result.unwrap_or(serde_json::Value::Null),
            )?),
            ResponseType::Error => Err(self.parse_error(&envelope)),
            ResponseType::Async => Err(Error::UnexpectedResponseType("async".to_string())),
        }
    }

    fn parse_async(&self, bytes: &[u8]) -> Result<ChangeId> {
        let envelope: RawSnapdResponse = serde_json::from_slice(bytes)?;

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

/// Read a full HTTP/1.1 response from the stream and return the body bytes.
///
/// Handles `Transfer-Encoding: chunked`, `Content-Length`, and
/// close-delimited responses.
fn read_response(stream: UnixStream) -> Result<Vec<u8>> {
    let mut reader = BufReader::new(stream);

    let mut status_line = String::new();
    if reader.read_line(&mut status_line)? == 0 {
        return Err(Error::Connection("empty HTTP response".to_string()));
    }

    let mut content_length: Option<usize> = None;
    let mut chunked = false;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            break;
        }
        let line = line.trim_end_matches(['\r', '\n']);
        if line.is_empty() {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            let name = name.trim().to_ascii_lowercase();
            let value = value.trim();
            match name.as_str() {
                "content-length" => content_length = value.parse::<usize>().ok(),
                "transfer-encoding" if value.to_ascii_lowercase().contains("chunked") => {
                    chunked = true;
                }
                _ => {}
            }
        }
    }

    if chunked {
        read_chunked_body(&mut reader)
    } else if let Some(len) = content_length {
        let mut body = vec![0u8; len];
        reader.read_exact(&mut body)?;
        Ok(body)
    } else {
        let mut body = Vec::new();
        reader.read_to_end(&mut body)?;
        Ok(body)
    }
}

/// Decode a chunked transfer-encoded body.
fn read_chunked_body<R: BufRead>(reader: &mut R) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    loop {
        let mut size_line = String::new();
        if reader.read_line(&mut size_line)? == 0 {
            break;
        }
        let size_field = size_line.trim().split(';').next().unwrap_or("").trim();
        if size_field.is_empty() {
            break;
        }
        let size = usize::from_str_radix(size_field, 16)
            .map_err(|_| Error::Connection(format!("invalid chunk size: {size_field}")))?;
        if size == 0 {
            // Consume any trailers up to the terminating blank line.
            loop {
                let mut trailer = String::new();
                if reader.read_line(&mut trailer)? == 0
                    || trailer.trim_end_matches(['\r', '\n']).is_empty()
                {
                    break;
                }
            }
            break;
        }
        let mut chunk = vec![0u8; size];
        reader.read_exact(&mut chunk)?;
        body.extend_from_slice(&chunk);
        // Consume the CRLF that terminates the chunk data.
        let mut crlf = String::new();
        reader.read_line(&mut crlf)?;
    }
    Ok(body)
}
