# Maintainer: Parch Linux Developers <dev@parchlinux.com>
pkgname=parch-helper
pkgver=0.1.0
pkgrel=1
pkgdesc="Distro command translation and migration helper for Parch Linux"
arch=('x86_64' 'aarch64')
url="https://github.com/AhooraZen/ParchHelper"
license=('GPL-3.0-or-later')
depends=('gcc-libs' 'pacman')
optdepends=(
    'paru: AUR package helper support'
    'yay: alternative AUR helper support'
    'debtap: install .deb packages'
)
makedepends=('cargo' 'rust')
backup=('etc/parch/helper.toml')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 "target/release/parch-helper" "$pkgdir/usr/bin/parch-helper"
    ln -sf "/usr/bin/parch-helper" "$pkgdir/usr/bin/parch-translate"

    # Default config
    install -Dm644 "config/helper.toml" "$pkgdir/etc/parch/helper.toml"

    # Shell profile integrations
    install -Dm644 "shell/parch-helper.sh" "$pkgdir/etc/profile.d/parch-helper.sh"
    install -Dm644 "shell/parch-helper.zsh" "$pkgdir/usr/share/zsh/site-functions/parch-helper.zsh"
    install -Dm644 "shell/parch-helper.fish" "$pkgdir/usr/share/fish/vendor_conf.d/parch-helper.fish"

    # Symlink aliases for seamless intercept
    for cmd in apt apt-get apt-cache dnf yum apk zypper; do
        ln -sf "/usr/bin/parch-helper" "$pkgdir/usr/bin/$cmd"
    done
}
