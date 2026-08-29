use parch_helper::config::Config;
use parch_helper::context::{CliOptions, InvocationContext, SourceManager};
use parch_helper::translator::translate;

fn test_ctx(source: SourceManager, args: &[&str]) -> InvocationContext {
    InvocationContext {
        source,
        original_args: args.iter().map(|s| s.to_string()).collect(),
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
        cli_opts: CliOptions::default(),
    }
}

#[test]
fn test_apt_update() {
    let ctx = test_ctx(SourceManager::Apt, &["update"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -Sy");
    assert!(res.warning.is_some());
}

#[test]
fn test_apt_get_typo_handling() {
    let ctx = test_ctx(SourceManager::Apt, &["get", "upgrade"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -Syu");
}

#[test]
fn test_apt_install_with_mapping() {
    let ctx = test_ctx(
        SourceManager::Apt,
        &["install", "build-essential", "libssl-dev"],
    );
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S base-devel openssl");
}

#[test]
fn test_apt_install_noconfirm() {
    let ctx = test_ctx(SourceManager::Apt, &["install", "-y", "htop"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S --noconfirm htop");
    assert!(res.exec_args.contains(&"--noconfirm".to_string()));
}

#[test]
fn test_dnf_upgrade() {
    let ctx = test_ctx(SourceManager::Dnf, &["upgrade"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -Syu");
}

#[test]
fn test_dnf_install_mapping() {
    let ctx = test_ctx(
        SourceManager::Dnf,
        &["install", "-y", "gcc-c++", "openssl-devel"],
    );
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S --noconfirm gcc openssl");
}

#[test]
fn test_apk_add() {
    let ctx = test_ctx(SourceManager::Apk, &["add", "htop"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S htop");
}

#[test]
fn test_zypper_install() {
    let ctx = test_ctx(SourceManager::Zypper, &["in", "-y", "libopenssl-devel"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S --noconfirm openssl");
}

#[test]
fn test_brew_install() {
    let ctx = test_ctx(SourceManager::Brew, &["install", "openssl@3"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S openssl");
}

#[test]
fn test_flatpak_install() {
    let ctx = test_ctx(SourceManager::Flatpak, &["install", "-y", "neovim"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S neovim --noconfirm");
}

#[test]
fn test_snap_install() {
    let ctx = test_ctx(SourceManager::Snap, &["install", "fastfetch"]);
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S fastfetch");
}
