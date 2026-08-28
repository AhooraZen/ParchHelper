use parch_helper::config::Config;
use parch_helper::context::{InvocationContext, SourceManager};
use parch_helper::translator::translate;

#[test]
fn test_apt_update() {
    let ctx = InvocationContext {
        source: SourceManager::Apt,
        original_args: vec!["update".to_string()],
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
    };
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "sudo pacman -Sy");
}

#[test]
fn test_apt_get_typo_handling() {
    let ctx = InvocationContext {
        source: SourceManager::Apt,
        original_args: vec!["get".to_string(), "upgrade".to_string()],
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
    };
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -Syu");
}

#[test]
fn test_apt_install_with_mapping() {
    let ctx = InvocationContext {
        source: SourceManager::Apt,
        original_args: vec!["install".to_string(), "build-essential".to_string(), "libssl-dev".to_string()],
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
    };
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S base-devel openssl");
}

#[test]
fn test_dnf_upgrade() {
    let ctx = InvocationContext {
        source: SourceManager::Dnf,
        original_args: vec!["upgrade".to_string()],
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
    };
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -Syu");
}

#[test]
fn test_apk_add() {
    let ctx = InvocationContext {
        source: SourceManager::Apk,
        original_args: vec!["add".to_string(), "htop".to_string()],
        is_sudo: false,
        sudo_user: None,
        is_interactive: true,
    };
    let cfg = Config::default();
    let res = translate(&ctx, &cfg);
    assert_eq!(res.command, "paru -S htop");
}
