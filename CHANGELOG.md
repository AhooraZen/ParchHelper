# Changelog

All notable changes to `parch-helper` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.0] - 2026-08-29

### 🚀 Added
- **Dynamic Release Notes & Rolling CI**: Multi-arch CI pipeline with automated changelog generation, commit metadata, and SHA256 checksums.
- **Sleek Bracketed Neon Badges**: Redesigned `❬EN❭` and `❬FA❭` badges to eliminate solid box artifacts and match installer aesthetic.
- **OSC-52 Terminal Clipboard Support**: Instant clipboard copy via `c` key during interactive prompts (works seamlessly across SSH and tmux).
- **In-Place Command Editor**: Interactive prompt editor mode via `e` key to tweak translated commands before execution.
- **Root Filesystem Safety Guard**: Instant detection of read-only root filesystems (`/`) via `statvfs` to prevent corrupt partial package operations.
- **Active Pacman Lock Inspection**: Inspects `/var/lib/pacman/db.lck` for active holding processes vs stale locks.
- **Privilege Drop Engine**: Drops root privileges to caller user (`sudo -u` / `setuid` / `setgid` / `initgroups`) when running AUR helpers (`paru`, `yay`).
- **Orphan Package Cleanup**: Native `pacman -Qtdq` resolution and `-Rns` execution without subshell expansion failures.
- **Comprehensive Distro Converters**: Translation engines for `apt`, `dnf`, `apk`, `zypper`, `brew`, `flatpak`, and `snap`.

### 🎨 Changed
- **ANSI 256-Color Palette**: Standardized terminal themes on the neon installer palette (`\x1b[38;5;51m` cyan, `\x1b[38;5;39m` blue, `\x1b[38;5;48m` green, `\x1b[38;5;220m` yellow, `\x1b[38;5;201m` magenta).
- **Cargo Release Profile**: Optimized binary size and execution speed with `opt-level = 3`, `lto = true`, `panic = "abort"`, and `strip = true`.

### 🛡️ Fixed
- **BiDi Layout Disruption**: Implemented Unicode BiDi isolation characters around Persian (FA) descriptions to protect terminal box borders.
- **Clippy & Formatting Warnings**: Cleaned up all strict clippy warnings and enforced formatting across all targets.

---

## [0.1.0] - 2026-08-29

### 🚀 Added
- Initial project release for Parch Linux.
- Basic translation from foreign package managers (`apt`, `dnf`, `brew`) to `pacman`/`paru`.
- Basic TUI box rendering with bilingual English and Persian support.
- Shell integration hooks for `command-not-found` in Bash, Zsh, and Fish.
