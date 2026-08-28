# Parch Linux Maintainer Guide: `parch-helper`

## 1. Overview & Purpose
`parch-helper` is a zero-overhead, native Rust utility and shell-interception engine for **Parch Linux** (an Arch-based Linux distribution).

Its primary goals:
- **Interception:** Seamlessly catch Debian/Ubuntu (`apt`, `apt-get`, `dpkg`), Fedora/RHEL (`dnf`, `yum`, `rpm`), Alpine (`apk`), openSUSE (`zypper`), and macOS (`brew`) commands.
- **Normalization:** Map foreign package names and flags to Arch equivalents (e.g., `build-essential` -> `base-devel`, `libssl-dev` -> `openssl`, `-y` -> `--noconfirm`).
- **Bilingual Guidance:** Present clean terminal boxes in English and Persian (فارسی).
- **Execution & Privilege Control:** Execute target commands with automatic root-dropping when calling AUR helpers (`paru`/`yay`) under `sudo`.

---

## 2. Architecture & Request Flow

```text
User Input (e.g., 'sudo apt update' or 'apt-get install libssl-dev')
    │
    ▼
┌─────────────────────────────────────────────────────────────┐
│ 1. Invocation Context (/src/context.rs)                     │
│    - Detect binary source name from argv[0] or subcommand   │
│    - Detect TTY/interactive state                           │
│    - Detect EUID root and SUDO_USER environment             │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 2. Translation & Normalization Engine (/src/translator/)    │
│    - Parse subcommands & sanitize typos (e.g., 'apt get')   │
│    - Map package names via embedded lookup & heuristics     │
│    - Map flags (-y -> --noconfirm)                          │
│    - Configure target helper (paru, yay, pacman)            │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 3. UI Renderer (/src/ui/)                                   │
│    - Render high-contrast ANSI box                          │
│    - Display English & Persian explanations                 │
│    - If non-interactive script: print suggestion & exit 127 │
└─────────────────────────────┬───────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│ 4. Safety & Executor (/src/safety.rs & /src/executor.rs)    │
│    - Check /var/lib/pacman/db.lck lockfile                  │
│    - Guard against destructive commands (glibc/systemd rm)  │
│    - If AUR helper + root: drop privileges via `su - user`  │
│    - Execute translated command                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 3. How the Symlink & Interception Mechanism Works

When a user runs `apt update`:
1. `/usr/bin/apt` is a symlink pointing to `/usr/bin/parch-helper`.
2. Linux kernel passes `argv[0] = "apt"` and `argv[1] = "update"`.
3. `InvocationContext::capture()` inspects `argv[0]`:
   - Detects `SourceManager::Apt`.
   - Reads `argv[1..]` as `["update"]`.
4. Translator parses `update` as database refresh:
   - Target command: `paru -Sy` (or `sudo pacman -Sy` depending on config).
5. Terminal renders bilingual explanation box and prompts `Execute: paru -Sy ? [Y/n/c]`.
6. User presses `Enter` / `Y` -> `Executor::run()` runs the native command.

### When Run Under `sudo apt ...`
1. `sudo` executes `/usr/bin/apt` with root privileges.
2. `parch-helper` detects `EUID == 0` and reads `SUDO_USER` from environment.
3. If translated command requires an AUR helper (`paru`/`yay` which block running as root), `parch-helper` automatically drops privileges using `su - $SUDO_USER -c "paru ..."`.
4. If target is pure `pacman`, redundant leading `sudo` is stripped and executed directly.

---

## 4. Package Filesystem Hierarchy

| File Path | Description |
| :--- | :--- |
| `/usr/bin/parch-helper` | Core compiled ELF binary |
| `/usr/bin/parch-translate` | Symlink to `parch-helper` |
| `/usr/bin/apt`, `apt-get`, `dnf`, `yum`, `apk`, `zypper` | Interception symlinks to `/usr/bin/parch-helper` |
| `/etc/parch/helper.toml` | Global system configuration |
| `~/.config/parch/helper.toml` | Per-user configuration override |
| `/etc/profile.d/parch-helper.sh` | Global Bash shell integration & `command_not_found_handle` |
| `/usr/share/zsh/site-functions/parch-helper.zsh` | Zsh `command_not_found_handler` |
| `/usr/share/fish/vendor_conf.d/parch-helper.fish` | Fish `fish_command_not_found` handler |

---

## 5. Configuration Reference (`/etc/parch/helper.toml`)

```toml
[general]
# Preferred Arch package manager / AUR helper: "paru" | "yay" | "pacman"
helper = "paru"

# UI language output: "both" | "en" | "fa"
language = "both"

# Auto-execute without interactive prompt (true for immediate execution)
auto_execute = false

# Enable ANSI colored box output
colored_ui = true

# Enable AUR fallback suggestion
aur_fallback = true

[package_overrides]
# Custom manual mappings (distro_pkg_name = "arch_pkg_name")
"my-custom-deb" = "my-custom-arch"
```

---

## 6. Packaging & Release for Parch Linux ISO / Repos

### PKGBUILD Build
```bash
cd ParchHelper
makepkg -sric
```

### Binary Release Build
```bash
cargo build --release --locked
# Stripped binary is located at target/release/parch-helper (~1.2MB uncompressed, 0 dependencies)
```

---

## 7. Adding New Package Mappings
To add package name translations:
1. Edit `data/pkg_mappings.json`.
2. Add entries to `debian_to_arch` or `fedora_to_arch`.
3. Recompile with `cargo build --release` (mappings are embedded at compile time via `include_str!`).
