# Parch Linux Command Helper (`parch-helper`)

<p align="center">
  <b>A smart, ultra-fast command translation and migration helper for Parch Linux & Arch-based systems.</b><br>
  <i>راهنمای هوشمند و فوق‌سریع تبدیل دستورات سایر توزیع‌ها برای پارچ لینوکس</i>
</p>

---

## 🌟 Features

- ⚡ **Zero-latency, Native Rust Core**: Sub-2ms execution time, single static binary, no bloated runtime.
- 🔄 **Multi-Distro Translation**: Translates commands from **Debian/Ubuntu** (`apt`, `apt-get`, `dpkg`), **Fedora/RHEL/CentOS** (`dnf`, `yum`, `rpm`), **Alpine** (`apk`), **openSUSE** (`zypper`), and **macOS** (`brew`) to native `pacman` and AUR helpers (`paru`/`yay`).
- 📦 **Smart Package Name Normalization**: Automatically converts distro-specific package names (e.g. `build-essential` -> `base-devel`, `libssl-dev` -> `openssl`, `python3-pip` -> `python-pip`).
- 🇮🇷 **Bilingual Terminal Output (English & Persian)**: Clear, visually distinct high-contrast boxes designed for readability.
- 🛡️ **Safety & Privilege Dropping**: Prevents dangerous deletions, detects pacman database lock (`db.lck`), and automatically drops root privileges when handing off tasks to AUR helpers.
- 🚀 **1-Click Interactive Execution**: Press `Y` or `Enter` to run the translated command immediately.

---

## 📸 Preview

```text
╭─ [ Parch Linux Command Helper ] ──────────────────────────────────────╮
│                                                                       │
│   Input    : apt install build-essential libssl-dev                   │
│   Arch     : sudo pacman -S base-devel openssl                        │
│                                                                       │
│   EN: Debian package names mapped to Arch Linux packages.             │
│   FA: .تبدیل شدند Arch Linux بسته‌های دبیان به معادل‌های              │
│                                                                       │
╰───────────────────────────────────────────────────────────────────────╯
Execute: sudo pacman -S base-devel openssl ? [Y/n/c] 
```

---

## 🛠️ How to Compile & Install

### Requirements
- Rust toolchain (`cargo`, `rustc`)
- GCC or Clang toolchain
- `git`

### Quick Build & Run
```bash
# 1. Clone the repository
git clone https://github.com/AhooraZen/ParchHelper.git
cd ParchHelper

# 2. Compile release binary
cargo build --release

# 3. Test running the binary directly
./target/release/parch-helper apt update
./target/release/parch-helper apt install build-essential
```

### System-wide Installation
```bash
sudo ./install.sh
```

Or using `make`:
```bash
make
sudo make install
```

### Arch Linux / Parch PKGBUILD
```bash
makepkg -si
```

---

## ⚙️ Configuration (`/etc/parch/helper.toml`)

```toml
[general]
# Preferred AUR helper: "paru" | "yay" | "pacman"
helper = "paru"

# Language mode: "both" (EN+FA) | "en" | "fa"
language = "both"

# Automatically execute without prompt
auto_execute = false

# Enable colorful box rendering
colored_ui = true

[package_overrides]
# Custom package aliases
"custom-debian-pkg" = "custom-arch-pkg"
```

---

## 📜 License
GPL-3.0-or-later. Created with ❤️ for Parch Linux users.
