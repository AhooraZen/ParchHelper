# Parch Linux Command Helper (`parch-helper` / `parch-translate`)

<p align="center">
  <b>A lightweight, ultra-fast command translation and migration helper for Parch Linux & Arch-based distributions.</b><br>
  <i>راهنمای هوشمند، بومی‌سازی شده و فوق‌سریع برای تبدیل دستورات دبیان، اوبونتو، فدورا، آلپاین و اوپن‌سوزه به آرچ/پارچ</i>
</p>

---

## 🚀 Features

- ⚡ **Zero-Overhead & Native**: Written in Rust, single static binary, sub-2ms startup time with zero runtime dependencies.
- 🔄 **Comprehensive Distro Interception**: Translates commands from:
  - **Debian / Ubuntu**: `apt`, `apt-get`, `apt-cache`, `aptitude`, `dpkg`
  - **Fedora / RHEL / CentOS**: `dnf`, `yum`, `rpm`
  - **Alpine Linux**: `apk`
  - **openSUSE**: `zypper`
  - **macOS**: `brew`
- 📦 **Intelligent Package Normalization**: Automatically normalizes package names (e.g. `build-essential` -> `base-devel`, `libssl-dev` / `openssl-devel` -> `openssl`, `python3-pip` -> `python-pip`, `docker-ce` -> `docker`).
- 🇮🇷 **Bilingual UI (English & Persian)**: High-contrast, beautifully styled ANSI box rendering with clear explanations.
- 🛡️ **Privilege & Safety Guards**:
  - Automatically drops root privileges via `su - $SUDO_USER -c` when executing AUR helpers (`paru`/`yay`) under `sudo`.
  - Strips redundant `sudo` when already running under root.
  - Warns against `/var/lib/pacman/db.lck` lock files.
  - Blocks catastrophic deletion attempts on core packages (`glibc`, `systemd`, `linux`, `base`).
- ⚡ **1-Click Execution**: Press `Enter` or `Y` to immediately run the translated command, or cancel gracefully with `n` / `Ctrl+C`.

---

## 📸 Preview

```text
╭─── ❬ Parch Linux Command Helper ❭ ───────────────────────────────────╮
│                                                                      │
│   Input        : apt install build-essential libssl-dev              │
│   Arch/Parch   : paru -S base-devel openssl                          │
│                                                                      │
│    EN   Installs package(s) via Arch repos / AUR (paru).             │
│    FA   نصب بسته(ها) از طریق مخازن رسمی و مخزن کاربران (paru).       │
│                                                                      │
╰──────────────────────────────────────────────────────────────────────╯
╭─▶ Execute `paru -S base-devel openssl` ? [Y/n/c] ❯ 
✔ Executing...
```

---

## 🛠️ Installation & Building

### 1. Quick Installer (Recommended)
Clone the repo and run the standalone setup script:
```bash
git clone https://github.com/AhooraZen/ParchHelper.git
cd ParchHelper
sudo ./install.sh
```

### 2. Manual Cargo Build
```bash
cargo build --release
sudo install -Dm755 target/release/parch-helper /usr/bin/parch-helper
sudo ln -sf /usr/bin/parch-helper /usr/bin/parch-translate
```

### 3. Arch Linux / Parch PKGBUILD
Build and install via `makepkg`:
```bash
makepkg -si
```

---

## ⚙️ Configuration (`/etc/parch/helper.toml`)

You can customize `parch-helper` behavior globally at `/etc/parch/helper.toml` or per-user at `~/.config/parch/helper.toml`:

```toml
[general]
# Target helper for system & AUR updates: "paru" | "yay" | "pacman"
helper = "paru"

# UI language display mode: "both" (EN+FA) | "en" | "fa"
language = "both"

# Automatically execute without asking for interactive confirmation
auto_execute = false

# Enable colored ANSI borders and badges
colored_ui = true

# Allow AUR fallback suggestions
aur_fallback = true

[package_overrides]
# Custom package mappings (foreign-pkg = "arch-pkg")
"custom-deb-app" = "custom-arch-app"
```

---

## 📖 Architecture & Maintainer Guide

### Request & Execution Flow

```text
User Command (e.g. 'sudo apt update' or 'dnf install gcc-c++')
   │
   ▼
1. Invocation Context (/src/context.rs)
   ├── Inspects argv[0] to determine source manager (apt, dnf, apk, etc.)
   ├── Parses remaining command-line arguments
   ├── Detects TTY / interactive session
   └── Checks root/EUID status and extracts SUDO_USER
   │
   ▼
2. Translation Engine (/src/translator/)
   ├── Handles typos & subcommands (e.g., 'apt get upgrade' -> 'upgrade')
   ├── Applies package name dictionary (/data/pkg_mappings.json) & heuristics
   ├── Translates flags (-y -> --noconfirm)
   └── Builds TranslationResult with English & Persian notes
   │
   ▼
3. UI Renderer (/src/ui/)
   ├── Renders formatted ANSI box
   └── Prompts user for 1-click execution
   │
   ▼
4. Executor & Safety Layer (/src/executor.rs)
   ├── Checks /var/lib/pacman/db.lck lockfile
   ├── Evaluates safety guards
   ├── Drops privileges to $SUDO_USER for AUR helpers
   └── Executes translated command
```

### Command Translation Matrix

| Distro Command | Translated Parch/Arch Command | Description |
| :--- | :--- | :--- |
| `apt update` / `dnf check-update` | `paru -Sy` / `sudo pacman -Sy` | Refresh repository databases |
| `apt upgrade` / `dnf upgrade` | `paru -Syu` / `sudo pacman -Syu` | Full system & AUR upgrade |
| `apt install <pkg>` / `dnf install <pkg>` | `paru -S <pkg>` | Install package (+ auto-name mapping) |
| `apt remove <pkg>` / `apk del <pkg>` | `sudo pacman -R <pkg>` / `paru -R` | Remove package |
| `apt purge <pkg>` / `dnf erase <pkg>` | `sudo pacman -Rns <pkg>` | Remove package, unused deps & config |
| `apt autoremove` / `dnf autoremove` | `sudo pacman -Rns $(pacman -Qtdq)` | Remove orphaned dependencies |
| `apt search <query>` / `dnf search` | `paru -Ss <query>` | Search package database |
| `apt show <pkg>` / `dnf info <pkg>` | `pacman -Si <pkg>` | View package metadata & dependencies |
| `apt list --installed` | `pacman -Qe` | List explicitly installed packages |
| `apt clean` / `dnf clean all` | `sudo pacman -Sc` | Clean cached package tarballs |
| `apt-file search <f>` / `dnf provides <f>` | `pacman -F <f>` | Find which package owns a specific file |
| `dpkg -i <file.deb>` | `debtap <file.deb>` | Suggest deb package conversion |
| `rpm -i <file.rpm>` | `rpmextract <file.rpm>` | Suggest rpm archive extraction |

---

## 🧪 Testing

Run all unit tests locally:
```bash
cargo test
```

---

## 📜 License
Released under the **GPL-3.0-or-later** license. Developed for the Parch Linux community with ❤️.
