# snapd-rs-artie

A type-safe, async Rust client library for the [snapd REST API](https://snapcraft.io/docs/snapd-api). This library enables Rust applications to interact with the snap daemon over Unix sockets to manage snaps, interfaces, services, and more.

## Features

- **Async/await support** - Built on tokio and hyper for high-performance async I/O
- **Type-safe API** - Strongly-typed request/response models with serde
- **Unix socket communication** - Direct communication with snapd over `/run/snapd.socket`
- **Comprehensive coverage** - Supports 70+ snapd API endpoints across all major categories
- **Snap confinement support** - Works both outside and inside snap confinement
- **Change tracking** - Monitor async operations with built-in change polling

## Supported API Categories

- ✅ **Snaps** - Install, remove, refresh, revert, enable/disable snaps
- ✅ **Store** - Search and discover snaps in the store
- ✅ **Interfaces** - Connect/disconnect plugs and slots
- ✅ **Changes** - Track and abort async operations
- ✅ **Apps & Services** - Start, stop, restart snap services
- ✅ **Aliases** - Manage snap command aliases
- ✅ **Snapshots** - Create, restore, and manage snapshots
- ✅ **Auth** - Login/logout from the snap store
- ✅ **Assertions** - Work with snap assertions
- ✅ **System Info** - Query system information and warnings
- ✅ **Validation Sets** - Apply and manage validation sets
- ✅ **Quotas** - Manage resource quota groups
- ✅ **Prompting** - Handle interface access prompts and rules
- ✅ **Notices** - Subscribe to and manage system notices
- ✅ **Recovery & FDE** - Work with recovery systems and encryption keys
- ✅ **Model** - Query device model and serial assertions
- ✅ **Debug** - Access debug endpoints

See [ENDPOINTS.md](https://github.com/artiepoole/snapd-rs-artie/blob/main/ENDPOINTS.md) for detailed endpoint coverage.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
snapd-rs-artie = "..."
```

## Quick Start

```rust
use snapd_rs_artie::{SnapdClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // List installed snaps
    let snaps = client.list_snaps().await?;
    for snap in snaps {
        println!("{}: {} ({})", snap.name, snap.version, snap.channel);
    }
    
    Ok(())
}
```

## Usage Examples

### Installing a Snap

```rust
use snapd_rs_artie::{SnapdClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // Install from a specific channel
    let change_id = client.install_snap("firefox", Some("stable")).await?;
    println!("Installation started: {}", change_id);
    
    // Wait for completion
    loop {
        let change = client.get_change(&change_id).await?;
        println!("Status: {:?}", change.status);
        
        if change.ready {
            if change.status == "Done" {
                println!("Installation complete!");
            } else {
                println!("Installation failed: {}", change.err.unwrap_or_default());
            }
            break;
        }
        
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Ok(())
}
```

### Searching the Store

```rust
use snapd_rs_artie::SnapdClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // Search for snaps
    let results = client.find_snaps("firefox").await?;
    
    for snap in results {
        println!("{}: {}", snap.name, snap.summary);
        println!("  Publisher: {}", snap.publisher.display_name);
        println!("  Channels: {:?}", snap.channels);
    }
    
    Ok(())
}
```

### Managing Services

```rust
use snapd_rs_artie::SnapdClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // List all services
    let apps = client.list_apps().await?;
    for app in apps.iter().filter(|a| a.daemon.is_some()) {
        println!("{}.{}: {:?}", app.snap, app.name, app.active);
    }
    
    // Restart a service
    let change_id = client.restart_service(&["my-snap.my-service"]).await?;
    
    Ok(())
}
```

### Working with Interfaces

```rust
use snapd_rs_artie::SnapdClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // List all connections
    let connections = client.list_connections().await?;
    
    for conn in connections {
        println!("{}:{} -> {}:{}",
            conn.plug_snap, conn.plug,
            conn.slot_snap, conn.slot
        );
    }
    
    // Connect an interface
    client.connect_interface(
        "my-snap",     // snap name
        "home",        // plug name
        None,          // slot snap (None for system)
        None,          // slot name (None = same as plug)
    ).await?;
    
    Ok(())
}
```

### Creating Snapshots

```rust
use snapd_rs_artie::SnapdClient;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // Create a snapshot of specific snaps
    let change_id = client.create_snapshot(&["firefox", "thunderbird"]).await?;
    
    // List all snapshots
    let snapshots = client.list_snapshots().await?;
    for snapshot in snapshots {
        println!("Snapshot {}: {:?}", snapshot.id, snapshot.snaps);
    }
    
    Ok(())
}
```

### Handling Snap Configuration

```rust
use snapd_rs_artie::SnapdClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // Get configuration
    let conf = client.get_snap_conf("my-snap", &["key1", "key2"]).await?;
    println!("Current config: {}", conf);
    
    // Set configuration
    let new_conf = json!({
        "key1": "value1",
        "key2": 42
    });
    client.set_snap_conf("my-snap", new_conf).await?;
    
    Ok(())
}
```

### Working with Different Sockets

```rust
use snapd_rs_artie::SnapdClient;

// Default socket for unconfined processes
let client = SnapdClient::new();

// Socket for confined snaps (uses snap-specific socket)
let client = SnapdClient::new_for_snap();

// Custom filesystem socket path
let client = SnapdClient::with_socket("/custom/path/snapd.socket");

// Abstract socket (Linux abstract namespace)
let client = SnapdClient::new_abstract("io.snapcraft.Launcher");
```

## Error Handling

The library uses a custom `Result<T>` type with a comprehensive `SnapdError` enum:

```rust
use snapd_rs_artie::{SnapdClient, SnapdError};

#[tokio::main]
async fn main() {
    let client = SnapdClient::new();
    
    match client.get_snap("nonexistent-snap").await {
        Ok(snap) => println!("Found: {}", snap.name),
        Err(SnapdError::SnapNotFound) => println!("Snap not found"),
        Err(SnapdError::PermissionDenied) => println!("Access denied"),
        Err(e) => println!("Other error: {}", e),
    }
}
```

## Requirements

- **Rust**: 1.83+ (Edition 2024)
- **snapd**: Running snapd daemon (typically `/run/snapd.socket`)
- **Platform**: Linux with snap support

## Testing

The library includes both unit tests and integration tests:

```bash
# Run all tests (requires running snapd)
cargo test

# Run only unit tests
cargo test --lib

# Run integration tests
cargo test --test integration_tests
```

## Architecture

The library is built on:

- **[tokio](https://tokio.rs/)** - Async runtime
- **[hyper](https://hyper.rs/)** - HTTP client
- **[serde](https://serde.rs/)** - Serialization/deserialization
- **[hyper-util](https://docs.rs/hyper-util/)** - Unix socket support

Communication happens over Unix domain sockets using HTTP/1.1, matching snapd's protocol.

## License

This project is licensed under the GNU General Public License v3.0 - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please:

1. Run `cargo fmt` to format code
2. Run `cargo clippy` to check for lints
3. Ensure tests pass with `cargo test`
4. Use conventional commits (`feat:`, `fix:`, `docs:`, etc.)

## Related Projects

- **[snapd](https://github.com/snapcore/snapd)** - The snap daemon
- **[snapcraft](https://snapcraft.io/)** - Official snap documentation
- **[snap-rat](https://github.com/artiepoole/snap-rat)** - TUI snap manager built with snapd-rs-artie
- **[snap-rat-vibes](https://github.com/artiepoole/snap-rat-vibes)** - Vibe Coded TUI snap manager built with snapd-rs-artie

## Disclaimer

This is an **unofficial** library and is not affiliated with or endorsed by Canonical Ltd. or the snapd project.

## Links

- **Documentation**: [docs.rs/snapd-rs-artie](https://docs.rs/snapd-rs-artie)
- **Repository**: [github.com/artiepoole/snapd-rs-artie](https://github.com/artiepoole/snapd-rs-artie)
- **Crates.io**: [crates.io/crates/snapd-rs-artie](https://crates.io/crates/snapd-rs-artie)
- **snapd API Docs**: [snapcraft.io/docs/snapd-api](https://snapcraft.io/docs/snapd-api)
