pub mod api;
pub mod client;
pub mod error;
pub mod types;

pub use client::SnapdClient;
pub use client::{SNAPD_SNAP_SOCKET, SNAPD_SOCKET, SocketAddress};
pub use error::{Error, Result};
pub use types::{
    AliasStatusKind, ChangeId, ChangeStatus, DaemonScope, DaemonType, NoticeType, PromptLifespan,
    PromptOutcome, Revision, SnapConfinement, SnapStatus, SnapType, SystemMode, ValidationSetMode,
};
