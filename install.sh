#!/usr/bin/env bash
set -e

# Run cargo build as original invoking user, not root
echo "==> Building Parch Linux Command Helper as $(whoami)..."
cargo build --release -j 1

if [ ! -f "target/release/parch-helper" ]; then
    echo "[-] Build failed: binary not found in target/release/parch-helper" >&2
    exit 1
fi

# Elevate with sudo only at the installation step
SUDO=""
if [ "$(id -u)" -ne 0 ]; then
    SUDO="sudo"
    echo "==> Elevating privileges to install to /usr/bin and /etc/parch..."
fi

echo "==> Installing binary to /usr/bin/parch-helper..."
$SUDO install -Dm755 "target/release/parch-helper" "/usr/bin/parch-helper"
$SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/parch-translate"

echo "==> Installing default configuration to /etc/parch/helper.toml..."
$SUDO mkdir -p "/etc/parch"
if [ ! -f "/etc/parch/helper.toml" ]; then
    $SUDO cp "config/helper.toml" "/etc/parch/helper.toml"
fi

echo "==> Installing shell hooks..."
$SUDO mkdir -p "/etc/profile.d" "/usr/share/zsh/site-functions" "/usr/share/fish/vendor_conf.d"
$SUDO cp "shell/parch-helper.sh" "/etc/profile.d/parch-helper.sh"
$SUDO cp "shell/parch-helper.zsh" "/usr/share/zsh/site-functions/parch-helper.zsh"
$SUDO cp "shell/parch-helper.fish" "/usr/share/fish/vendor_conf.d/parch-helper.fish"

echo "==> Creating seamless command symlinks (apt, dnf, apk, etc.)..."
for cmd in apt apt-get apt-cache aptitude dnf yum apk zypper brew dpkg rpm flatpak snap; do
    if [ ! -f "/usr/bin/$cmd" ] || [ -L "/usr/bin/$cmd" ]; then
        $SUDO ln -sf "/usr/bin/parch-helper" "/usr/bin/$cmd"
    fi
done

echo "==> Installation complete! Try running: apt update"
