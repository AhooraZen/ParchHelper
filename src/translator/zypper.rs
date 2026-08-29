use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_zypper(args: &[String], config: &Config) -> TranslationResult {
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
            notes_en: "Running system upgrade (openSUSE 'zypper' -> Arch 'pacman').".to_string(),
            notes_fa: "ارتقای سیستم (تبدیل دستور zypper به پَک‌من).".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "ref" | "refresh" => TranslationResult {
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
            notes_en: "Refreshes repository metadata.".to_string(),
            notes_fa: "به‌روزرسانی کش و متادیتای مخازن.".to_string(),
            warning: Some("Warning: -Sy alone may cause partial upgrades on Arch.".to_string()),
        },
        "dup" | "dist-upgrade" | "up" | "update" => {
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--non-interactive");
            let mut exec_args = vec!["-Syu".to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }

            let full_cmd = if helper == "pacman" {
                let mut c = "sudo pacman -Syu".to_string();
                if noconfirm {
                    c.push_str(" --noconfirm");
                }
                c
            } else {
                let mut c = format!("{} -Syu", helper);
                if noconfirm {
                    c.push_str(" --noconfirm");
                }
                c
            };

            let binary = if helper == "pacman" { "sudo" } else { helper };
            let final_args = if helper == "pacman" {
                let mut a = vec!["pacman".to_string()];
                a.extend(exec_args);
                a
            } else {
                exec_args
            };

            TranslationResult {
                command: full_cmd,
                exec_binary: binary.to_string(),
                exec_args: final_args,
                op: PacmanOp::SyncUpgrade {
                    noconfirm,
                    download_only: false,
                },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: "Full distribution upgrade.".to_string(),
                notes_fa: "ارتقای کلی سیستم.".to_string(),
                warning: None,
            }
        }
        "in" | "install" => {
            let mut pkgs = Vec::new();
            let mut noconfirm = false;

            for arg in rest {
                if arg == "-y" || arg == "--non-interactive" {
                    noconfirm = true;
                } else if !arg.starts_with('-') {
                    let mapped = if let Some(custom) = config.package_overrides.get(arg) {
                        custom.clone()
                    } else {
                        mappings.translate_zypper_pkg(arg)
                    };
                    pkgs.push(mapped);
                }
            }

            let mut exec_args = vec!["-S".to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }
            exec_args.extend(pkgs.clone());

            let mut cmd_parts = Vec::new();
            if helper == "pacman" {
                cmd_parts.push("sudo pacman".to_string());
            } else {
                cmd_parts.push(helper.clone());
            }
            cmd_parts.push("-S".to_string());
            if noconfirm {
                cmd_parts.push("--noconfirm".to_string());
            }
            cmd_parts.extend(pkgs.clone());

            let full_cmd = cmd_parts.join(" ");
            let binary = if helper == "pacman" { "sudo" } else { helper };
            let final_args = if helper == "pacman" {
                let mut a = vec!["pacman".to_string()];
                a.extend(exec_args);
                a
            } else {
                exec_args
            };

            TranslationResult {
                command: full_cmd,
                exec_binary: binary.to_string(),
                exec_args: final_args,
                op: PacmanOp::SyncInstall {
                    pkgs,
                    noconfirm,
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
        "rm" | "remove" => {
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
                notes_en: "Removes package and unneeded dependencies.".to_string(),
                notes_fa: "حذف پکیج و وابستگی‌ها.".to_string(),
                warning: None,
            }
        }
        "se" | "search" => {
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
                notes_en: "Displays package details.".to_string(),
                notes_fa: "نمایش اطلاعات پکیج.".to_string(),
                warning: None,
            }
        }
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
