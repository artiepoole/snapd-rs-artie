# snapd-rs-artie

A Vibe coded PoC of unofficial Rust bindings for the snapd API (for interacting with the Ubuntu/Canonical) snap store. This allows applications in Rust to talk directly to [snapd](https://snapcraft.io/docs/snapd-api) without writing their own API layer.

This library provides type-safe, async/await bindings to communicate with the snapd daemon over Unix sockets, allowing Rust applications to manage snaps, interfaces, changes, and more without writing their own API layer.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
snapd-rs-artie = { path = "snapd-rs-artie" }  # or use a version from a registry when published
tokio = { version = "1", features = ["full"] }
```

## Usage

### Basic Example

```rust
use snapd_rs_artie::{SnapdClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client connected to the default snapd socket
    let client = SnapdClient::new();
    
    // List all installed snaps
    let snaps = client.list_snaps().await?;
    for snap in snaps {
        println!("{}: {}", snap.name, snap.version);
    }
    
    // Search the store
    let results = client.find_snaps("firefox", None).await?;
    println!("Found {} results", results.len());
    
    Ok(())
}
```

### Installing a Snap

```rust
use snapd_rs_artie::{SnapdClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // Install from the stable channel
    let change_id = client.install_snap("firefox", Some("stable"), None).await?;
    println!("Installation started: change ID {:?}", change_id);
    
    // Monitor the change
    loop {
        let change = client.get_change(change_id).await?;
        println!("Status: {:?}", change.status);
        if change.ready {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    
    Ok(())
}
```

### Managing Interfaces

```rust
use snapd_rs_artie::{SnapdClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = SnapdClient::new();
    
    // List all interface connections
    let connections = client.list_connections(None, None).await?;
    for plug in connections.plugs {
        println!("Plug: {}:{}", plug.snap, plug.plug);
    }
    
    // Connect an interface
    let change_id = client
        .connect_interface("my-snap", "home", None, None)
        .await?;
    println!("Connection started: change ID {:?}", change_id);
    
    Ok(())
}
```

### Using Different Sockets

```rust
use snapd_rs_artie::SnapdClient;

// Default socket for unconfined processes
let client = SnapdClient::new();

// Socket for confined snaps
let client = SnapdClient::new_for_snap();

// Custom filesystem socket
let client = SnapdClient::with_socket("/custom/path/snapd.socket");

// Abstract socket (for snap confinement)
let client = SnapdClient::new_abstract("io.snapcraft.Launcher");
```

## Building

### Prerequisites

- Rust 1.83+ (edition 2024)
- Access to a running snapd daemon

### Build the Library

```bash
cargo build --release
```

### Run Tests

The library includes integration tests that require a running snapd daemon:

```bash
# Run all tests
cargo test

# Run only unit tests (no snapd required)
cargo test --lib

# Run integration tests (requires snapd)
cargo test --test integration_tests
```

### Check and Lint

```bash
# Check for compile errors
cargo check

# Run clippy linter
cargo clippy

# Format code
cargo fmt
```

## Project Structure

```
snapd-rs-artie/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── client.rs           # SnapdClient and HTTP layer
│   ├── error.rs            # Error types
│   ├── types.rs            # Common types and enums
│   └── api/                # API endpoint implementations
│       ├── snaps.rs        # Snap management
│       ├── interfaces.rs   # Interface connections
│       ├── changes.rs      # Change tracking
│       ├── store.rs        # Store search
│       ├── prompting.rs    # Prompting API
│       └── ...             # Other endpoints
└── tests/
    ├── integration_tests.rs
    └── mock_snapd.rs
```

## API Coverage

The library provides access to most snapd API endpoints. See [`ENDPOINTS.md`](./ENDPOINTS.md) for a detailed list of implemented endpoints.

## Development

This project uses AI-assisted development to generate endpoint implementations and type definitions from the snapd API specification.

## License

This project is open source. See LICENSE file for details.

## Contributing

Contributions are welcome! Please ensure:
- Code is formatted with `cargo fmt`
- Lints pass with `cargo clippy`
- Tests pass with `cargo test`
- Commit messages follow conventional commits format (`feat:`, `fix:`, `refactor:`, etc.)

## Related Projects

- [snapd](https://github.com/snapcore/snapd) - The snap daemon
- [snapcraft](https://snapcraft.io/) - Official snap documentation
- [snap-rat](https://github.com/artiepoole/snap-rat) My artisanal snap store and snap manager TUI using snapd-rs-artie
- [snap-rat-vibes](https://github.com/artiepoole/snap-rat-vibes) My vibe-coded snap store and snap manager TUI using snapd-rs-artie

## Credits
[Oliver Calder](https://github.com/olivercalder) for snapd expertise and manual improvements to this project
[Paul Rodriguez](https://github.com/paul-rodriguez) for contributions to the initial bringup
[Edoardo Barbieri](https://github.com/BarbieriEdoardo) for vibe-coding various tests and GitHub actions