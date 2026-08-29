#!/usr/bin/env bash
set -e

# --- ANSI Neon Color Palette ---
C_RESET="\033[0m"
C_BOLD="\033[1m"
C_CYAN="\033[38;5;51m"
C_BLUE="\033[38;5;39m"
C_GREEN="\033[38;5;48m"
C_YELLOW="\033[38;5;220m"
C_MAGENTA="\033[38;5;201m"
C_RED="\033[38;5;196m"
C_GRAY="\033[38;5;244m"

print_banner() {
    echo -e "${C_BLUE}╭─────────────────────────────────────────────────────────────╮${C_RESET}"
    echo -e "${C_BLUE}│${C_RESET}  ${C_BOLD}${C_CYAN}❬⚡❭ Parch Linux Command Helper Installer${C_RESET}                ${C_BLUE}│${C_RESET}"
    echo -e "${C_BLUE}│${C_RESET}      ${C_MAGENTA}Multi-distro translation & safety verification${C_RESET}         ${C_BLUE}│${C_RESET}"
    echo -e "${C_BLUE}╰─────────────────────────────────────────────────────────────╯${C_RESET}"
    echo ""
}

print_banner

REPO="AhooraZen/ParchHelper"
ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64)
        TARGET_ARCH="x86_64"
        ;;
    aarch64|arm64)
        TARGET_ARCH="aarch64"
        ;;
    *)
        echo -e "${C_YELLOW}[!] Unsupported CPU architecture: ${ARCH}. Fallback to local source build.${C_RESET}"
        TARGET_ARCH=""
        ;;
esac

RELEASE_URL="https://github.com/${REPO}/releases/download/latest/parch-helper-${TARGET_ARCH}.tar.gz"
BINARY_URL="https://github.com/${REPO}/releases/download/latest/parch-helper-${TARGET_ARCH}"

TEMP_DIR=$(mktemp -d)
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

INSTALLED_SUCCESS=0
BUILD_LOCAL=false

# Check if user specifically requested a local source build via y/N prompt
if [ -t 0 ]; then
    echo -ne "${C_CYAN}╭─▶ ${C_BOLD}Do you want to compile from source locally? ${C_YELLOW}[y/N]${C_CYAN} ❯ ${C_RESET}"
    read -r user_choice
    clean_choice=$(echo "$user_choice" | tr '[:upper:]' '[:lower:]' | xargs)
    if [[ "$clean_choice" == "y" || "$clean_choice" == "yes" ]]; then
        BUILD_LOCAL=true
        echo -e "${C_GREEN}✔ Selected local compilation.${C_RESET}\n"
    else
        echo -e "${C_BLUE}ℹ Selected fast pre-built binary release.${C_RESET}\n"
    fi
fi

# Step 1: Fast download from GitHub Actions release
if [ "$BUILD_LOCAL" = false ] && [ -n "$TARGET_ARCH" ]; then
    echo -e "${C_CYAN}==>${C_RESET} ${C_BOLD}Fetching latest pre-compiled release (${TARGET_ARCH})...${C_RESET}"

    if command -v curl >/dev/null 2>&1; then
        if curl -sSfL "$RELEASE_URL" -o "$TEMP_DIR/release.tar.gz" 2>/dev/null; then
            echo -e "${C_GREEN}==>${C_RESET} ${C_BOLD}Extracting release archive...${C_RESET}"
            tar -xzf "$TEMP_DIR/release.tar.gz" -C "$TEMP_DIR"
            INSTALLED_SUCCESS=1
        elif curl -sSfL "$BINARY_URL" -o "$TEMP_DIR/parch-helper" 2>/dev/null; then
            echo -e "${C_GREEN}==>${C_RESET} ${C_BOLD}Downloaded binary artifact directly...${C_RESET}"
            chmod +x "$TEMP_DIR/parch-helper"
            mkdir -p "$TEMP_DIR/usr/bin"
            cp "$TEMP_DIR/parch-helper" "$TEMP_DIR/usr/bin/"
            INSTALLED_SUCCESS=1
        fi
    elif command -v wget >/dev/null 2>&1; then
        if wget -q "$RELEASE_URL" -O "$TEMP_DIR/release.tar.gz" 2>/dev/null; then
            echo -e "${C_GREEN}==>${C_RESET} ${C_BOLD}Extracting release archive...${C_RESET}"
            tar -xzf "$TEMP_DIR/release.tar.gz" -C "$TEMP_DIR"
            INSTALLED_SUCCESS=1
        elif wget -q "$BINARY_URL" -O "$TEMP_DIR/parch-helper" 2>/dev/null; then
            echo -e "${C_GREEN}==>${C_RESET} ${C_BOLD}Downloaded binary artifact directly...${C_RESET}"
            chmod +x "$TEMP_DIR/parch-helper"
            mkdir -p "$TEMP_DIR/usr/bin"
            cp "$TEMP_DIR/parch-helper" "$TEMP_DIR/usr/bin/"
            INSTALLED_SUCCESS=1
        fi
    fi
fi

# Step 2: Compile locally from source via cargo
if [ "$INSTALLED_SUCCESS" -ne 1 ]; then
    echo -e "${C_YELLOW}==>${C_RESET} ${C_BOLD}Compiling release from source via cargo...${C_RESET}"
    if ! command -v cargo >/dev/null 2>&1; then
        echo -e "${C_RED}[-] Error: 'cargo' is required for local build but is not installed.${C_RESET}" >&2
        exit 1
    fi

    cargo build --release
    mkdir -p "$TEMP_DIR/usr/bin" "$TEMP_DIR/etc/parch" "$TEMP_DIR/etc/profile.d" \
             "$TEMP_DIR/usr/share/zsh/site-functions" "$TEMP_DIR/usr/share/fish/vendor_conf.d"

    cp "target/release/parch-helper" "$TEMP_DIR/usr/bin/"
    cp "config/helper.toml" "$TEMP_DIR/etc/parch/"
    cp "shell/parch-helper.sh" "$TEMP_DIR/etc/profile.d/"
    cp "shell/parch-helper.zsh" "$TEMP_DIR/usr/share/zsh/site-functions/"
    cp "shell/parch-helper.fish" "$TEMP_DIR/usr/share/fish/vendor_conf.d/"
fi

# Step 3: Elevation check for file deployment
SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    SUDO="sudo"
    echo -e "${C_MAGENTA}==>${C_RESET} ${C_BOLD}Elevating privileges with sudo for system installation...${C_RESET}"
fi

echo -e "${C_CYAN}==>${C_RESET} Installing binary to /usr/bin/parch-helper..."
$SUDO install -Dm755 "$TEMP_DIR/usr/bin/parch-helper" "/usr/bin/parch-helper"
$SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/parch-translate"

echo -e "${C_CYAN}==>${C_RESET} Installing configuration to /etc/parch/helper.toml..."
$SUDO mkdir -p "/etc/parch"
if [ ! -f "/etc/parch/helper.toml" ]; then
    if [ -f "$TEMP_DIR/etc/parch/helper.toml" ]; then
        $SUDO cp "$TEMP_DIR/etc/parch/helper.toml" "/etc/parch/helper.toml"
    elif [ -f "config/helper.toml" ]; then
        $SUDO cp "config/helper.toml" "/etc/parch/helper.toml"
    fi
fi

echo -e "${C_CYAN}==>${C_RESET} Installing shell hooks for Bash, Zsh, and Fish..."
$SUDO mkdir -p "/etc/profile.d" "/usr/share/zsh/site-functions" "/usr/share/fish/vendor_conf.d"

if [ -f "$TEMP_DIR/etc/profile.d/parch-helper.sh" ]; then
    $SUDO cp "$TEMP_DIR/etc/profile.d/parch-helper.sh" "/etc/profile.d/parch-helper.sh"
elif [ -f "shell/parch-helper.sh" ]; then
    $SUDO cp "shell/parch-helper.sh" "/etc/profile.d/parch-helper.sh"
fi

if [ -f "$TEMP_DIR/usr/share/zsh/site-functions/parch-helper.zsh" ]; then
    $SUDO cp "$TEMP_DIR/usr/share/zsh/site-functions/parch-helper.zsh" "/usr/share/zsh/site-functions/parch-helper.zsh"
elif [ -f "shell/parch-helper.zsh" ]; then
    $SUDO cp "shell/parch-helper.zsh" "/usr/share/zsh/site-functions/parch-helper.zsh"
fi

if [ -f "$TEMP_DIR/usr/share/fish/vendor_conf.d/parch-helper.fish" ]; then
    $SUDO cp "$TEMP_DIR/usr/share/fish/vendor_conf.d/parch-helper.fish" "/usr/share/fish/vendor_conf.d/parch-helper.fish"
elif [ -f "shell/parch-helper.fish" ]; then
    $SUDO cp "shell/parch-helper.fish" "/usr/share/fish/vendor_conf.d/parch-helper.fish"
fi

echo -e "${C_CYAN}==>${C_RESET} Generating emulation wrappers for foreign package managers..."
for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    if [ ! -f "/usr/bin/$cmd" ] || [ -L "/usr/bin/$cmd" ]; then
        $SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/$cmd"
    fi
done

echo ""
echo -e "${C_GREEN}${C_BOLD}✔ Parch Linux Command Helper installed successfully!${C_RESET}"
echo -e "${C_GRAY}Try typing:${C_RESET} ${C_YELLOW}apt update${C_RESET}, ${C_YELLOW}dnf install${C_RESET}, ${C_YELLOW}brew install${C_RESET}, or ${C_YELLOW}parch-helper --help${C_RESET}"
