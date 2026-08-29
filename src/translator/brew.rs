use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_brew(args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if args.is_empty() {
        return TranslationResult {
            command: if helper == "pacman" {
                "sudo pacman -Syu".to_string()
            } else {
                format!("{} -Syu", helper)
            },
            exec_binary: if helper == "pacman" {
                "sudo".to_string()
            } else {
                helper.clone()
            },
            exec_args: if helper == "pacman" {
                vec!["pacman".to_string(), "-Syu".to_string()]
            } else {
                vec!["-Syu".to_string()]
            },
            op: PacmanOp::SyncUpgrade {
                noconfirm: false,
                download_only: false,
            },
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Running system upgrade (Homebrew 'brew' -> Arch 'pacman').".to_string(),
            notes_fa: "ارتقای سیستم (تبدیل دستور brew به پَک‌من).".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "update" => TranslationResult {
            command: if helper == "pacman" {
                "sudo pacman -Sy".to_string()
            } else {
                format!("{} -Sy", helper)
            },
            exec_binary: if helper == "pacman" {
                "sudo".to_string()
            } else {
                helper.clone()
            },
            exec_args: if helper == "pacman" {
                vec!["pacman".to_string(), "-Sy".to_string()]
            } else {
                vec!["-Sy".to_string()]
            },
            op: PacmanOp::SyncRefresh { force: false },
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Updates package database.".to_string(),
            notes_fa: "به‌روزرسانی لیست مخازن.".to_string(),
            warning: Some(
                "Warning: Running -Sy without upgrade can lead to broken packages.".to_string(),
            ),
        },
        "upgrade" => TranslationResult {
            command: if helper == "pacman" {
                "sudo pacman -Syu".to_string()
            } else {
                format!("{} -Syu", helper)
            },
            exec_binary: if helper == "pacman" {
                "sudo".to_string()
            } else {
                helper.clone()
            },
            exec_args: if helper == "pacman" {
                vec!["pacman".to_string(), "-Syu".to_string()]
            } else {
                vec!["-Syu".to_string()]
            },
            op: PacmanOp::SyncUpgrade {
                noconfirm: false,
                download_only: false,
            },
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Full package and system upgrade.".to_string(),
            notes_fa: "ارتقای تمام بسته‌های سیستم.".to_string(),
            warning: None,
        },
        "install" => {
            let pkgs: Vec<String> = rest
                .iter()
                .filter(|a| !a.starts_with('-'))
                .map(|a| {
                    if let Some(custom) = config.package_overrides.get(a) {
                        custom.clone()
                    } else {
                        mappings.translate_brew_pkg(a)
                    }
                })
                .collect();

            let full_cmd = if helper == "pacman" {
                format!("sudo pacman -S {}", pkgs.join(" "))
            } else {
                format!("{} -S {}", helper, pkgs.join(" "))
            };

            let binary = if helper == "pacman" { "sudo" } else { helper };
            let final_args = if helper == "pacman" {
                let mut a = vec!["pacman".to_string(), "-S".to_string()];
                a.extend(pkgs.clone());
                a
            } else {
                let mut a = vec!["-S".to_string()];
                a.extend(pkgs.clone());
                a
            };

            TranslationResult {
                command: full_cmd,
                exec_binary: binary.to_string(),
                exec_args: final_args,
                op: PacmanOp::SyncInstall {
                    pkgs,
                    noconfirm: false,
                    as_deps: false,
                    download_only: false,
                },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: format!("Installs package(s) via Arch / AUR ({}).", helper),
                notes_fa: format!("نصب بسته(ها) با استفاده از {}.", helper),
                warning: None,
            }
        }
        "uninstall" | "remove" => {
            let pkgs: Vec<String> = rest
                .iter()
                .filter(|a| !a.starts_with('-'))
                .cloned()
                .collect();
            TranslationResult {
                command: format!("sudo pacman -Rns {}", pkgs.join(" ")),
                exec_binary: "sudo".to_string(),
                exec_args: {
                    let mut a = vec!["pacman".to_string(), "-Rns".to_string()];
                    a.extend(pkgs.clone());
                    a
                },
                op: PacmanOp::Remove {
                    pkgs,
                    noconfirm: false,
                    cascade: false,
                    nosave: true,
                    recursive: true,
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Uninstalls package and unneeded dependencies.".to_string(),
                notes_fa: "حذف کامل پکیج و وابستگی‌ها.".to_string(),
                warning: None,
            }
        }
        "search" => {
            let query = rest
                .iter()
                .filter(|a| !a.starts_with('-'))
                .cloned()
                .collect::<Vec<_>>()
                .join(" ");
            let full_cmd = if helper == "pacman" {
                format!("pacman -Ss {}", query)
            } else {
                format!("{} -Ss {}", helper, query)
            };
            let binary = if helper == "pacman" { "pacman" } else { helper };

            TranslationResult {
                command: full_cmd,
                exec_binary: binary.to_string(),
                exec_args: vec!["-Ss".to_string(), query.clone()],
                op: PacmanOp::SyncSearch { query },
                needs_root: false,
                needs_aur: helper != "pacman",
                notes_en: "Searches package database.".to_string(),
                notes_fa: "جستجوی بسته در مخازن.".to_string(),
                warning: None,
            }
        }
        "info" => {
            let pkg = rest.first().cloned().unwrap_or_default();
            TranslationResult {
                command: format!("pacman -Si {}", pkg),
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-Si".to_string(), pkg.clone()],
                op: PacmanOp::SyncInfo { pkg },
                needs_root: false,
                needs_aur: false,
                notes_en: "Shows package information.".to_string(),
                notes_fa: "مشاهده مشخصات پکیج.".to_string(),
                warning: None,
            }
        }
        "cleanup" => TranslationResult {
            command: "sudo pacman -Sc".to_string(),
            exec_binary: "sudo".to_string(),
            exec_args: vec!["pacman".to_string(), "-Sc".to_string()],
            op: PacmanOp::CleanCache { all: false },
            needs_root: true,
            needs_aur: false,
            notes_en: "Cleans cached download files.".to_string(),
            notes_fa: "پاکسازی فایل‌های کش.".to_string(),
            warning: None,
        },
        other => TranslationResult {
            command: format!("pacman -S {}", other),
            exec_binary: "pacman".to_string(),
            exec_args: vec!["-S".to_string(), other.to_string()],
            op: PacmanOp::DirectPacman {
                args: vec!["-S".to_string(), other.to_string()],
            },
            needs_root: true,
            needs_aur: false,
            notes_en: format!("Attempting pacman operation for '{}'.", other),
            notes_fa: format!("تلاش برای اجرای معادل پَک‌من دستور '{}'.", other),
            warning: None,
        },
    }
}
