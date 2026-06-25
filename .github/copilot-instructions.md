# Copilot Project Instructions

## Project overview

This is a Rust workspace with one crate

| Crate            | Path              | Purpose |
|------------------|-------------------|---|
| `snapd-rs-artie` | `snapd-rs-artie/` | snapd API client (Unix socket via hyper) |

---

## snapd-rs API client

- Communicates over the snapd Unix socket at `/run/snapd.socket`
- All snap operations live in `snapd-rs/src/api/snaps.rs`
- Key methods: `install_snap`, `install_snap_classic`, `remove_snap`, `remove_snap_purge`, `refresh_snap`, `revert_snap`, `enable_snap`, `disable_snap`, `list_snaps`, `find_snaps`, `list_connections`, `connect_interface`, `disconnect_interface`, `list_changes`, `get_change`, `abort_change`
- `remove_snap_purge` sends `{ "action": "remove", "purge": true }` — deletes all snap data
- Returns `ChangeId` for async operations; poll `/v2/changes/{id}` to track progress

---

## Development workflow

### Before every commit

```bash
bash /project/artie_sandbox/checks.sh
```

This runs: `cargo check`, `cargo clippy --fix`, `yamlfmt .` (non-fatal, may not be installed), `cargo fmt`.

### Commit style

Conventional commits: `feat:`, `fix:`, `refactor:`, `docs:`, `chore:`

Always include the co-author trailer:
```
Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
```

---

## Coding conventions

- `yamlfmt` may not be installed — non-fatal, ignore its errors in checks.sh
