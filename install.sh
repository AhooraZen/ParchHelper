#!/usr/bin/env bash
set -e

echo "==> Building Parch Linux Command Helper..."
cargo build --release

echo "==> Installing binary to /usr/bin/parch-helper..."
sudo install -Dm755 "target/release/parch-helper" "/usr/bin/parch-helper"
sudo ln -sf "/usr/bin/parch-helper" "/usr/bin/parch-translate"

echo "==> Installing default configuration to /etc/parch/helper.toml..."
sudo mkdir -p "/etc/parch"
if [ ! -f "/etc/parch/helper.toml" ]; then
    sudo cp "config/helper.toml" "/etc/parch/helper.toml"
fi

echo "==> Installing shell hooks..."
sudo mkdir -p "/etc/profile.d"
sudo cp "shell/parch-helper.sh" "/etc/profile.d/parch-helper.sh"

echo "==> Creating seamless command symlinks (apt, dnf, apk, etc.)..."
for cmd in apt apt-get apt-cache dnf yum apk zypper; do
    if [ ! -f "/usr/bin/$cmd" ] || [ -L "/usr/bin/$cmd" ]; then
        sudo ln -sf "/usr/bin/parch-helper" "/usr/bin/$cmd"
    fi
done

echo "==> Installation complete! Try running: apt update"
