# CLAUDE.md

Rust library for building Zigbee nodes (coordinator / router) on ESP32-C6, on top of Espressif's
`esp-zigbee-sdk` 2.x (C). Stack: ESP-IDF 6.1.x + `esp-idf-sys/-svc/-hal` (std, FreeRTOS), Embassy async
on the application side.

Current focus: 

- revisiting the project; do things look in place, does it build etc.
	- updating to ESP-IDF 6.1.1
	- building with a restricted set of components, for faster clean builds
- `apps/demo/2` — an IAS CIE coordinator reading a Wiser CCT591011 door/window sensor.
	- This has not yet worked; aiming to make it work in reality is a priority.

## Layers (workspace: raw, api, apps, config)

- `raw/` (`ezb-node-raw`): 1:1 bindgen bindings (`wrap.h`, `build.rs`, `bindings.rs`). NO abstractions;
  only filtering — `raw/src/lib.rs` re-exports just what `api` needs. No Rust features (see DESIGN.md).
  This is the `ESP_IDF_SYS_ROOT_CRATE`, so `[package.metadata.esp-idf-sys]` lives here.
- `config/` (`ezb-node-config`): host-side library, NOT an ESP-IDF project. Turns `app.toml` into Rust
  source (`apps/demo/**/.conf.rs` — generated, git-ignored, never edit by hand). Used as a
  build-dependency.
- `api/` (`ezb-node`): Rust-native API. C types/functions must NOT leak into the public interface.
  Features: role (`coordinator` / `router`), device types (`dt_*`), remote devices (`remote_*`).
- `apps/`: demos (`demo/1/light`, `demo/1/switch`, `demo/2`) plus shared `src/`. The application owns
  `sdkconfig.defaults` and `partitions.csv`.

Dependency direction: apps → api → (raw, config). Do not use `raw` types in `apps` code.

## Cargo configuration and symlinks

- Single source of truth: `.cargo/config.espidf.toml` (target `riscv32imac-esp-espidf`,
  `ESP_IDF_VERSION`, `ESP_IDF_COMPONENTS`, runner = espflash).
- `api/.cargo/config.toml`, `apps/.cargo/config.toml` and `raw/.cargo/config.toml` are SYMLINKS to it.
  Edit only the original; never replace a symlink with a regular file. There is intentionally no
  root-level `.cargo/config.toml` (`config/` is a host crate).
- Therefore always build from a sub-project directory (`cd apps`, etc.), not from the workspace root.
- The target directory is global (in the VM: `~/target`); `cargo metadata` reports the path.

**Important advice for agent:**

- Rust target folder is common for all projects at `~/target` (not the usual `./target`).
- Avoid `cargo clean` unless really needed; clean builds take >1min, due to `esp-idf-sys`. A clean build should be needed only after ESP-IDF version changes, and/or for build debugging.
- `Justfile`s are developer helpers and the source of truth for build commands, but documentation (`README`s etc.)
  shows the full commands (not `just ...`). Keep them in sync with a marker above the command block:
  `<!-- just: {path/to/Justfile} {recipe} -->` (path from repo root); the block lists exactly what
  `just --dry-run {recipe}` prints (a leading `$ ` is fine). Dev flags like `-vv`, `--jobs 1` stay in (intentional).
  - After changing a `Justfile` or documented commands, run `tools/check-docs.sh` and fix any mismatch.
  - Markers are being introduced gradually (started with `config/README.md`); add them when touching a doc's commands.

## Commands

Allowed without asking (build / check only):
- `cd config && just build`, `cargo test` (host)
- `cd api && just build`
- `cd apps && just lb | sb | db` (light / switch / door build). A full build is slow (minutes):
  run it in the background and tell the user.
- `cargo check` / `cargo clippy` in sub-projects (same features as in `apps/Justfile`).
- `tools/check-docs.sh` (read-only; only runs `just --dry-run`).

ASK FIRST (or leave for the user to run):
- `cargo clean`, `rm -rf ~/target …`, `rm -rf ~/.espressif` — rebuilding ESP-IDF takes long and
  downloads ~5 GB.
- ANYTHING that flashes or touches the device: `espflash …`, `cargo run`, `just lr | sr | dr`
  (`cargo run` uses the runner `espflash flash --monitor`), `idf.py`. A devkit is attached, but the
  user runs flashing and monitoring manually and pastes the logs into the conversation.
- Changes to `ESP_IDF_VERSION` / `ESP_IDF_COMPONENTS` / `sdkconfig.defaults` / `partitions.csv`
  (they force a full ESP-IDF rebuild or change the flash layout).
- Commits and pushes only on request.

## Code style

- Use `Self` inside impl blocks instead of repeating the type name (DETAILS.md).
- Toolchain: nightly (`rust-toolchain.toml`), edition 2024, `rust-version = "1.99"`.
- `unsafe` FFI calls only in the `api` layer; wrap C return codes with `EzbError::wrap` / `EspError::wrap`.
- C calls from the application thread must hold `ZigbeeGuard::acquire()` (see `api/src/node/mod.rs`,
  `NodeApi`).
- Zigbee callbacks (`api/src/node/zigbee_task.rs`) must not block; they convert the event and send it
  to `CHANNEL`.
- Comment conventions: `tbd.` = open item, `#feedback` = feedback for Espressif, `//r` = removed/old,
  `#[cfg(false)]` = parked code.

## Documentation

README.md (overview), DESIGN.md, DETAILS.md, TRACK.md (upstream tracking), TODO_AI.md (open design
questions, in Finnish), `apps/README.md`, `apps/demo/2/README.md`, `DEVS/`.

- `DEVS/` has more informal (casual) notes to other developers. It shall not be referenced from the rest of the repo.

