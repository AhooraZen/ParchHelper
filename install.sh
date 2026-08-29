#!/usr/bin/env bash
set -e

REPO="AhooraZen/ParchHelper"
RELEASE_URL="https://github.com/${REPO}/releases/download/latest/parch-helper-x86_64.tar.gz"
BINARY_FALLBACK_URL="https://github.com/${REPO}/releases/download/latest/parch-helper-x86_64"

echo "====================================================="
echo "   Parch Linux Command Helper - Fast Installer"
echo "====================================================="

TEMP_DIR=$(mktemp -d)
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

INSTALLED_SUCCESS=0

# Step 1: Attempt to download pre-built release artifact from GitHub Releases
echo "==> Fetching latest pre-built release from ${REPO}..."
if command -v curl >/dev/null 2>&1; then
    DOWNLOAD_CMD="curl -sSL"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOAD_CMD="wget -qO-"
else
    echo "[!] Neither curl nor wget found. Falling back to local source build..."
    DOWNLOAD_CMD=""
fi

if [ -n "$DOWNLOAD_CMD" ]; then
    if curl -sSfL "$RELEASE_URL" -o "$TEMP_DIR/release.tar.gz" 2>/dev/null; then
        echo "==> Extracting release archive..."
        tar -xzf "$TEMP_DIR/release.tar.gz" -C "$TEMP_DIR"
        INSTALLED_SUCCESS=1
    elif curl -sSfL "$BINARY_FALLBACK_URL" -o "$TEMP_DIR/parch-helper" 2>/dev/null; then
        echo "==> Downloaded binary directly..."
        chmod +x "$TEMP_DIR/parch-helper"
        mkdir -p "$TEMP_DIR/usr/bin"
        cp "$TEMP_DIR/parch-helper" "$TEMP_DIR/usr/bin/"
        INSTALLED_SUCCESS=1
    fi
fi

# Step 2: Fallback to local cargo build if download failed
if [ "$INSTALLED_SUCCESS" -ne 1 ]; then
    echo "==> Release binary not available online. Building from source with cargo..."
    if ! command -v cargo >/dev/null 2>&1; then
        echo "[-] Error: cargo is required to build from source but was not found." >&2
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

# Step 3: Auto-elevate with sudo only at installation
SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    SUDO="sudo"
    echo "==> Elevating privileges to install system components..."
fi

echo "==> Installing binary to /usr/bin/parch-helper..."
$SUDO install -Dm755 "$TEMP_DIR/usr/bin/parch-helper" "/usr/bin/parch-helper"
$SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/parch-translate"

echo "==> Installing configuration..."
$SUDO mkdir -p "/etc/parch"
if [ ! -f "/etc/parch/helper.toml" ]; then
    if [ -f "$TEMP_DIR/etc/parch/helper.toml" ]; then
        $SUDO cp "$TEMP_DIR/etc/parch/helper.toml" "/etc/parch/helper.toml"
    elif [ -f "config/helper.toml" ]; then
        $SUDO cp "config/helper.toml" "/etc/parch/helper.toml"
    fi
fi

echo "==> Installing shell integration hooks..."
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

echo "==> Creating command wrappers for seamless distro emulation..."
for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    if [ ! -f "/usr/bin/$cmd" ] || [ -L "/usr/bin/$cmd" ]; then
        $SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/$cmd"
    fi
done

echo ""
echo "✔ Parch Linux Command Helper installed successfully!"
echo "Try running: apt update, dnf install, or apk add"
