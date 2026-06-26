# snapd API Endpoints

## System Info & Warnings

- [x] `GET /v2/system-info` — Get system information (version, architecture, sandbox info, etc.)
  - [ ] Documented
  - [x] Implemented (`get_system_info`)

- [ ] `POST /v2/system-info` — System info actions (`advise-system-key-mismatch`)
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/system-info/storage-encrypted` — Get storage encryption status
  - [ ] Documented
  - [x] Implemented (`get_storage_encryption_status`)

- [x] `GET /v2/warnings` — List current warnings
  - [ ] Documented
  - [x] Implemented (`get_warnings`)

- [x] `POST /v2/warnings` — Acknowledge warnings (`okay`)
  - [ ] Documented
  - [x] Implemented (`acknowledge_warnings`)

## State / Changes

- [x] `GET /v2/changes` — List all async state changes
  - [ ] Documented
  - [x] Implemented (`list_changes`, `list_all_changes`)

- [x] `GET /v2/changes/{id}` — Get a specific change by ID
  - [ ] Documented
  - [x] Implemented (`get_change`)

- [x] `POST /v2/changes/{id}` — Abort a change (`abort`)
  - [ ] Documented
  - [x] Implemented (`abort_change`)

## Authentication & Users

- [x] `POST /v2/login` — Log in to the snap store (Macaroon auth)
  - [ ] Documented
  - [x] Implemented (`login`)

- [x] `POST /v2/logout` — Log out of the snap store
  - [ ] Documented
  - [x] Implemented (`logout`)

- [x] `POST /v2/create-user` — Create a local system user (deprecated, use `POST /v2/users`)
  - [ ] Documented
  - [x] Implemented (`create_user`)

- [x] `GET /v2/users` — List all users
  - [ ] Documented
  - [x] Implemented (`list_users`)

- [x] `POST /v2/users` — Create or remove a user (`create`, `remove`)
  - [ ] Documented
  - [x] Implemented (`create_user`, `remove_user`)

## Snaps

- [x] `GET /v2/snaps` — List all installed snaps
  - [ ] Documented
  - [x] Implemented (`list_snaps`)

- [x] `POST /v2/snaps` — Multi-snap operations (`install`, `refresh`, `revert`, `switch`, `hold`, `unhold`, `snapshot`, `remove`, `enable`, `disable`)
  - [ ] Documented
  - [x] Implemented (via single-snap operations, multi-snap not explicitly needed)

- [x] `GET /v2/snaps/{name}` — Get info for a specific snap
  - [ ] Documented
  - [x] Implemented (`get_snap`)

- [x] `POST /v2/snaps/{name}` — Single-snap operations (`install`, `refresh`, `revert`, `switch`, `hold`, `unhold`, `remove`, `enable`, `disable`)
  - [ ] Documented
  - [x] Implemented (`install_snap`, `install_snap_classic`, `sideload_snap`, `sideload_snap_classic`, `refresh_snap`, `revert_snap`, `remove_snap`, `remove_snap_purge`, `enable_snap`, `disable_snap`, `install_snap_component`, `remove_snap_component`)

- [x] `GET /v2/snaps/{name}/conf` — Get snap configuration
  - [ ] Documented
  - [x] Implemented (`get_snap_conf`)

- [x] `PUT /v2/snaps/{name}/conf` — Set snap configuration
  - [ ] Documented
  - [x] Implemented (`set_snap_conf`)

- [ ] `GET /v2/snaps/{name}/file` — Download the `.snap` file for an installed snap
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/icons/{name}/icon` — Get the icon for an installed snap
  - [ ] Documented
  - [x] Implemented (`get_snap_icon`)

## Store / Discovery

- [x] `GET /v2/find` — Search the snap store (`q`, `name`, `category`, `section`, `scope`, `select`)
  - [ ] Documented
  - [x] Implemented (`find_snaps`, `find_snap_by_name`)

- [ ] `GET /v2/sections` — List store sections (deprecated, see `/v2/categories`)
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/categories` — List store categories
  - [ ] Documented
  - [x] Implemented (`list_categories`)

- [ ] `POST /v2/download` — Download a snap from the store with resume support (`download`)
  - [ ] Documented
  - [ ] Implemented

- [ ] `POST /v2/cohorts` — Create cohort keys for snaps (`create`)
  - [ ] Documented
  - [ ] Implemented

## Snap Purchase

- [ ] `POST /v2/buy` — Buy a snap (currently unsupported)
  - [ ] Documented
  - [ ] Implemented

- [ ] `GET /v2/buy/ready` — Check if the user is ready to buy (currently unsupported)
  - [ ] Documented
  - [ ] Implemented

## Interfaces & Connections

- [x] `GET /v2/interfaces` — List interface connections or available interfaces (`?select=`)
  - [ ] Documented
  - [x] Implemented (`list_interfaces`, `list_all_interfaces`, `list_snap_interfaces`)

- [x] `POST /v2/interfaces` — Connect or disconnect interfaces (`connect`, `disconnect`)
  - [ ] Documented
  - [x] Implemented (`connect_interface`, `disconnect_interface`)

- [x] `GET /v2/connections` — List all plug/slot connections with filtering support
  - [ ] Documented
  - [x] Implemented (`list_connections`)

## Assertions

- [x] `GET /v2/assertions` — List available assertion type names
  - [ ] Documented
  - [x] Implemented (`list_assertion_types`)

- [x] `POST /v2/assertions` — Add a new assertion to the local store
  - [ ] Documented
  - [x] Implemented (`add_assertion`)

- [x] `GET /v2/assertions/{assertType}` — Find assertions by type (with header filter query params)
  - [ ] Documented
  - [x] Implemented (`get_assertions`)

## Apps & Services

- [x] `GET /v2/apps` — List all snap apps and services with status
  - [ ] Documented
  - [x] Implemented (`list_apps`, `list_snap_services`)

- [x] `POST /v2/apps` — Start, stop, or restart snap services (`start`, `stop`, `restart`)
  - [ ] Documented
  - [x] Implemented (`start_service`, `stop_service`, `restart_service`, `enable_service`, `disable_service`)

- [ ] `GET /v2/logs` — Stream or retrieve journald logs for snap services
  - [ ] Documented
  - [ ] Implemented

## Aliases

- [x] `GET /v2/aliases` — List all snap command aliases
  - [ ] Documented
  - [x] Implemented (`list_aliases`)

- [x] `POST /v2/aliases` — Manage aliases (`alias`, `unalias`, `prefer`)
  - [ ] Documented
  - [x] Implemented (`set_alias`, `remove_alias`, `prefer_aliases`)

## Snapshots

- [x] `GET /v2/snapshots` — List saved snapshots
  - [ ] Documented
  - [x] Implemented (`list_snapshots`)

- [x] `POST /v2/snapshots` — Manage snapshots (`check`, `restore`, `forget`)
  - [ ] Documented
  - [x] Implemented (`create_snapshot`, `restore_snapshot`, `forget_snapshot`)

- [ ] `GET /v2/snapshots/{id}/export` — Export a snapshot archive
  - [ ] Documented
  - [ ] Implemented

## Model & Device

- [x] `GET /v2/model` — Get the current device model assertion
  - [ ] Documented
  - [x] Implemented (`get_model`)

- [ ] `POST /v2/model` — Remodel the device (apply a new model assertion)
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/model/serial` — Get the device serial assertion
  - [ ] Documented
  - [x] Implemented (`get_serial`)

- [ ] `POST /v2/model/serial` — Manage the serial assertion (`forget`)
  - [ ] Documented
  - [ ] Implemented

## Recovery Systems

- [x] `GET /v2/systems` — List all available recovery/seed systems
  - [ ] Documented
  - [x] Implemented (`list_systems`)

- [ ] `POST /v2/systems` — Perform system-level actions (`reboot`, `create`, `install`)
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/systems/{label}` — Get details of a specific recovery system
  - [ ] Documented
  - [x] Implemented (`get_system`)

- [x] `POST /v2/systems/{label}` — Actions on a labeled system (`do`, `reboot`, `install`, `create`, `remove`, `check-passphrase-quality`, `check-pin-quality`, `fix-encryption-support`)
  - [ ] Documented
  - [x] Implemented (`reboot_into_system`)

## Validation Sets

- [x] `GET /v2/validation-sets` — List all tracked validation sets
  - [ ] Documented
  - [x] Implemented (`list_validation_sets`)

- [x] `GET /v2/validation-sets/{account}/{name}` — Get a specific validation set
  - [ ] Documented
  - [x] Implemented (`get_validation_set`)

- [x] `POST /v2/validation-sets/{account}/{name}` — Apply or forget a validation set (`forget`, `apply`)
  - [ ] Documented
  - [x] Implemented (`apply_validation_set`, `forget_validation_set`)

## Themes / Accessories

- [x] `GET /v2/accessories/themes` — Check availability/status of GTK, icon, and sound themes
  - [ ] Documented
  - [x] Implemented (`get_theme_status`)

- [x] `POST /v2/accessories/themes` — Install themes from the store
  - [ ] Documented
  - [x] Implemented (`install_themes`)

- [ ] `GET /v2/accessories/changes/{id}` — Get status of an accessories (theme install) change
  - [ ] Documented
  - [ ] Implemented (use standard `get_change`)

## Quota Groups

- [x] `GET /v2/quotas` — List all resource quota groups
  - [ ] Documented
  - [x] Implemented (`list_quotas`)

- [x] `POST /v2/quotas` — Manage quota groups (`ensure`, `remove`)
  - [ ] Documented
  - [x] Implemented (`ensure_quota`, `remove_quota`)

- [x] `GET /v2/quotas/{group}` — Get details of a specific quota group
  - [ ] Documented
  - [x] Implemented (`get_quota`)

## Confdb

- [x] `GET /v2/confdb/{account}/{confdb-schema}/{view}` — Read values from a confdb view
  - [ ] Documented
  - [x] Implemented (`get_confdb`)

- [x] `PUT /v2/confdb/{account}/{confdb-schema}/{view}` — Write values to a confdb view
  - [ ] Documented
  - [x] Implemented (`set_confdb`)

- [ ] `POST /v2/confdb` — Confdb control actions (`delegate`, `undelegate`)
  - [ ] Documented
  - [ ] Implemented

## Notices

- [x] `GET /v2/notices` — List notices, with filtering and long-poll support
  - [ ] Documented
  - [x] Implemented (`list_notices`, `list_notices_with`)

- [x] `POST /v2/notices` — Add a new notice (`add`)
  - [ ] Documented
  - [x] Implemented (`add_notice`)

- [x] `GET /v2/notices/{id}` — Get a specific notice by ID
  - [ ] Documented
  - [x] Implemented (`get_notice`)

## Prompting

- [ ] `POST /v2/interfaces/requests` — Post an interface access request from within a snap (`ask`)
  - [ ] Documented
  - [ ] Implemented

- [x] `GET /v2/interfaces/requests/prompts` — List pending prompts for the current user
  - [ ] Documented
  - [x] Implemented (`list_prompts`)

- [x] `GET /v2/interfaces/requests/prompts/{id}` — Get a specific prompt by ID
  - [ ] Documented
  - [x] Implemented (`get_prompt`)

- [x] `POST /v2/interfaces/requests/prompts/{id}` — Reply to a prompt (`allow`, `deny`)
  - [ ] Documented
  - [x] Implemented (`reply_to_prompt`)

- [x] `GET /v2/interfaces/requests/rules` — List all prompting rules for the current user
  - [ ] Documented
  - [x] Implemented (`list_prompt_rules`)

- [x] `POST /v2/interfaces/requests/rules` — Add or remove prompting rules (`add`, `remove`)
  - [ ] Documented
  - [x] Implemented (`add_prompt_rule`, `remove_prompt_rules`)

- [x] `GET /v2/interfaces/requests/rules/{id}` — Get a specific prompting rule by ID
  - [ ] Documented
  - [x] Implemented (`get_prompt_rule`)

- [x] `POST /v2/interfaces/requests/rules/{id}` — Modify or remove a prompting rule (`patch`, `remove`)
  - [ ] Documented
  - [x] Implemented (`patch_prompt_rule`, `remove_prompt_rule`)

## System Recovery Keys

- [x] `GET /v2/system-recovery-keys` — Retrieve FDE recovery keys
  - [ ] Documented
  - [x] Implemented (`get_recovery_keys`)

- [x] `POST /v2/system-recovery-keys` — Remove recovery keys (`remove`)
  - [ ] Documented
  - [x] Implemented (`remove_recovery_keys`)

## System Secureboot

- [x] `POST /v2/system-secureboot` — EFI Secure Boot database actions (`efi-secureboot-update-startup`, `efi-secureboot-update-db-cleanup`, `efi-secureboot-update-db-prepare`)
  - [ ] Documented
  - [x] Implemented (`secureboot_action`)

## System Volumes

- [x] `GET /v2/system-volumes` — List encrypted volumes and key slot information
  - [ ] Documented
  - [x] Implemented (`list_system_volumes`)

- [x] `POST /v2/system-volumes` — FDE key management actions (`generate-recovery-key`, `check-recovery-key`, `add-recovery-key`, `replace-recovery-key`, `replace-platform-key`, `check-passphrase-quality`, `check-pin-quality`, `change-passphrase`, `change-pin`)
  - [ ] Documented
  - [x] Implemented (`system_volumes_action`)

## Snapctl

- [x] `POST /v2/snapctl` — Execute a snapctl command from within a snap hook
  - [ ] Documented
  - [x] Implemented (`run_snapctl`)

## Debug

- [x] `GET /v2/debug` — Get debug info (`seeding`, `raa`, `connectivity`, `base-declaration`, `timings`, `features`, `change-timings`, `state`)
  - [ ] Documented
  - [x] Implemented

- [x] `POST /v2/debug` — Debug actions (`add-warning`, `unshow-warnings`, `ensure-state-soon`, `can-manage-refreshes`, `prune`, `stacktraces`, `create-recovery-system`, `migrate-home`)
  - [ ] Documented
  - [x] Implemented

- [ ] `GET /v2/debug/pprof/` — Go pprof profiling endpoints (`cmdline`, `profile`, `symbol`, `trace`, `heap`, `allocs`, `block`, `threadcreate`, `goroutine`, `mutex`)
  - [ ] Documented
  - [ ] Implemented

## Internal

- [ ] `POST /v2/internal/console-conf-start` — Called by `console-conf` at startup to pause snap auto-refresh
  - [ ] Documented
  - [ ] Implemented
