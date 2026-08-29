# Parch Linux Command Helper (`parch-helper` / `parch-translate`)

<p align="center">
  <b>Next-generation command translation, safety verification, and execution helper for Parch Linux & Arch-based distributions.</b><br>
  <i>راهنمای هوشمند، فوق‌سریع و ایمن برای تبدیل دستورات دبیان، اوبونتو، فدورا، آلپاین، اوپن‌سوزه، مک، فلت‌پک و اسنپ به آرچ/پارچ</i>
</p>

---

## 🚀 Features

- ⚡ **Zero-Overhead & Native**: Single static Rust binary, sub-millisecond execution, zero background daemon overhead.
- 🔄 **Multi-Distro & Container Interception**:
  - **Debian / Ubuntu**: `apt`, `apt-get`, `apt-cache`, `aptitude`, `dpkg`
  - **Fedora / RHEL / CentOS**: `dnf`, `yum`, `rpm`
  - **Alpine Linux**: `apk`
  - **openSUSE**: `zypper`
  - **macOS / Homebrew**: `brew`
  - **Containers & Sandboxes**: `flatpak`, `snap` (guides users to native Arch/AUR packages)
- 📦 **Comprehensive Package Normalization**: Extensive database mappings (`base-devel`, `openssl`, `python-*`, `docker`, `gtk3/4`, `mesa`, fonts) with smart suffix/prefix heuristics.
- 🎨 **Adaptive Modern TUI**:
  - Dynamic terminal width clamping (48–100 cols) to prevent line wrapping.
  - Unicode BiDi text isolation for RTL (Persian / FA) rendering without border breaking.
  - 5 Built-in themes: `neon`, `parch-dark`, `minimal`, `monokai`, `plain`.
- ⌨️ **Interactive Controls**:
  - `Enter` / `y`: Immediate execution.
  - `c`: Instant clipboard copy via OSC-52 escape codes (works over SSH and tmux).
  - `e`: In-place command editor.
  - `q` / `n` / `Esc`: Clean abort.
- 🛡️ **Hardened Safety & Privilege Engine**:
  - POSIX credential drop (`setuid`/`setgid`/`initgroups`) for AUR helpers (`paru`/`yay`) under `sudo`.
  - Read-only root filesystem detection (`statvfs`).
  - Active pacman lock `/var/lib/pacman/db.lck` detection with process ID mapping and stale lock handling.
  - AST-style dangerous command blocker (`rm -rf /`, cascade removals on `glibc`, `systemd`, `linux`, `base`, `pacman`).
  - Native orphan cleanup without subshell expansion failures.

---

## 📸 Preview

```text
╭─── ❬ Parch Linux Command Helper ❭ ───────────────────────────────────╮
│                                                                      │
│   Input        : apt install -y build-essential libssl-dev           │
│   Arch/Parch   : paru -S --noconfirm base-devel openssl              │
│                                                                      │
│    EN   Installs package(s) via Arch repos / AUR (paru).             │
│    FA   نصب بسته(ها) از طریق مخازن رسمی و مخزن کاربران (paru).       │
│                                                                      │
╰──────────────────────────────────────────────────────────────────────╯
╭─▶ Execute `paru -S --noconfirm base-devel openssl` ? [Enter/y: Run | c: Copy | e: Edit | q: Abort] ❯ 
✔ Executing...
```

---

## 🛠️ CLI Flags & Options

```text
Usage: parch-helper [OPTIONS] <COMMAND> [ARGS...]

Options:
  -e, --explain             Explain command without executing
  -d, --dry-run             Print translated command and exit (no execution)
  -y, --yes                 Auto-execute without interactive confirmation
  -i, --interactive         Force interactive prompt even in non-TTY sessions
  -j, --json                Output structured JSON translation metadata
  -H, --helper <NAME>       Override helper (pacman | paru | yay)
  -t, --theme <THEME>       Override theme (neon | parch-dark | minimal | monokai | plain)
  -c, --config <PATH>       Load custom TOML configuration file
  -h, --help                Show help information
```

---

## ⚙️ Configuration (`/etc/parch/helper.toml`)

```toml
[general]
# Preferred package helper: "paru" | "yay" | "pacman"
helper = "paru"

# Language mode: "both" (EN+FA) | "en" | "fa"
language = "both"

# UI Theme: "neon" | "parch-dark" | "minimal" | "monokai" | "plain"
theme = "neon"

# Auto-execute without confirmation prompt
auto_execute = false

# Show colored UI box
colored_ui = true

# Check AUR automatically
aur_fallback = true

# Enable BiDi Unicode isolation for RTL text
bidi_isolation = true

[package_overrides]
# Custom package mappings (foreign-pkg = "arch-pkg")
"custom-deb-app" = "custom-arch-app"
```

---

## 🧪 Testing & Verification

Run tests with low-memory single-CPU flag:
```bash
cargo test -j 1
```

---

## 📜 License
Released under the **GPL-3.0-or-later** license. Developed for the Parch Linux community with ❤️.
