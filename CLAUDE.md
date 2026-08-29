# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Stack & Overview
Parch Linux command translation, safety verification, and execution helper written in Rust (2021 edition). Translates foreign package manager commands (`apt`, `dnf`, `apk`, `zypper`, `brew`, `flatpak`, `snap`, etc.) to native Arch/Parch commands (`pacman`, `paru`, `yay`).

## Commands
- Build release binary: `cargo build --release`
- Low-memory build (1-core): `CARGO_BUILD_JOBS=1 cargo build --jobs 1`
- Run checks: `cargo check`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Format check: `cargo fmt --all -- --check`
- Apply formatting: `cargo fmt --all`
- Run all tests: `cargo test --all-targets`
- Run single test: `cargo test <test_name>` (e.g., `cargo test test_apt_install_basic`)
- Makefile targets: `make`, `make check`, `make test`, `make install`, `make uninstall`

Both `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test` must pass cleanly before landing changes.

## Architecture

- `src/main.rs`: Entry point. Parses CLI flags, loads config, invokes translator, dispatches to explain/dry-run/auto-execute or runs interactive prompt loop.
- `src/config.rs`: Loads `/etc/parch/helper.toml` and CLI overrides (helper preference, language mode, themes, auto-execution, AUR fallback, package overrides).
- `src/context.rs`: Inspects runtime environment (invoked binary name, args, TTY interactive state, sudo caller identity detection).
- `src/translator/`: Distro translation engines and package normalization.
  - `mod.rs`: Main `translate()` entry point routing commands to distro modules.
  - `apt.rs`, `dnf.rs`, `apk.rs`, `zypper.rs`, `brew.rs`, `flatpak.rs`, `snap.rs`: Per-distro syntax parsers and command converters.
  - `mapper.rs`: Loads `data/pkg_mappings.json` and applies heuristic normalization (prefixes, `-devel`/`-dev`, `python-*`, `lib*`).
- `src/safety.rs`: Safety enforcement.
  - `check_readonly_fs`: Prevents execution on read-only root mount.
  - `check_pacman_lock`: Detects `/var/lib/pacman/db.lck` and reports locking PID or stale lock.
  - `get_active_conflicting_services`: Flags background package managers (e.g., `packagekit`, `discover`).
  - `evaluate_command_safety`: AST-style dangerous command blocker (protects root, `glibc`, `systemd`, `linux`, `pacman`, `base`).
- `src/executor.rs`: Command execution subsystem.
  - Direct execution via standard process spawning.
  - Privilege drop (`execute_as_user`) using POSIX `setuid`/`setgid`/`initgroups` when AUR helpers (`paru`/`yay`) are invoked under `sudo`.
  - Native orphan cleanup (`pacman -Qtdq` -> `pacman -Rns`).
- `src/ui/`: Terminal user interface.
  - `theme.rs`: 256-color ANSI palette (`Neon`, `ParchDark`, `Minimal`, `Monokai`, `Plain`) matching installer aesthetic.
  - `box_draw.rs`: Renders translated command box with width clamping (48–100 columns).
  - `layout.rs`: Terminal layout calculations and Unicode BiDi isolation for Persian (FA) text.
  - `prompt.rs`: Raw-mode keyboard listener (`Enter`/`y`, `c` for OSC-52 clipboard copy, `e` for edit, `q`/`Esc` for abort).
- `shell/`: Shell integration hooks for `command-not-found` handling in Bash (`parch-helper.sh`), Zsh (`parch-helper.zsh`), and Fish (`parch-helper.fish`).
