use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_dnf(args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if args.is_empty() {
        return TranslationResult {
            command: if helper == "pacman" {
                "sudo pacman -Syu".to_string()
            } else {
                format!("{} -Syu", helper)
            },
            exec_binary: if helper == "pacman" { "sudo".to_string() } else { helper.clone() },
            exec_args: if helper == "pacman" { vec!["pacman".to_string(), "-Syu".to_string()] } else { vec!["-Syu".to_string()] },
            op: PacmanOp::SyncUpgrade { noconfirm: false, download_only: false },
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Running pacman system upgrade. In Parch, pacman/paru replaces DNF/YUM.".to_string(),
            notes_fa: "اجرای به‌روزرسانی سیستم. در پارچ از پَک‌من یا پارو به جای DNF استفاده می‌شود.".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "check-update" => TranslationResult {
            command: "checkupdates".to_string(),
            exec_binary: "checkupdates".to_string(),
            exec_args: vec![],
            op: PacmanOp::DirectPacman { args: vec!["checkupdates".to_string()] },
            needs_root: false,
            needs_aur: false,
            notes_en: "Safely checks for available repository updates.".to_string(),
            notes_fa: "بررسی امن به‌روزرسانی‌های موجود در مخازن.".to_string(),
            warning: None,
        },
        "upgrade" | "update" => {
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--assumeyes");
            let download_only = rest.iter().any(|a| a == "--downloadonly");

            let flag = if download_only { "-Syuw" } else { "-Syu" };
            let mut exec_args = vec![flag.to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }

            let full_cmd = if helper == "pacman" {
                let mut c = format!("sudo pacman {}", flag);
                if noconfirm { c.push_str(" --noconfirm"); }
                c
            } else {
                let mut c = format!("{} {}", helper, flag);
                if noconfirm { c.push_str(" --noconfirm"); }
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
                op: PacmanOp::SyncUpgrade { noconfirm, download_only },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: "Full system upgrade (repos + AUR).".to_string(),
                notes_fa: "به‌روزرسانی کامل سیستم و تمام پکیج‌ها.".to_string(),
                warning: None,
            }
        }
        "install" | "in" => {
            let mut pkgs = Vec::new();
            let mut has_rpm = false;
            let mut noconfirm = false;
            let mut download_only = false;

            for arg in rest {
                if arg == "-y" || arg == "--assumeyes" {
                    noconfirm = true;
                } else if arg == "--downloadonly" {
                    download_only = true;
                } else if arg.ends_with(".rpm") {
                    has_rpm = true;
                    pkgs.push(arg.clone());
                } else if !arg.starts_with('-') {
                    let clean_pkg = if let Some((base, _)) = arg.split_once('.') {
                        // e.g. pkg.x86_64 -> pkg
                        if base.contains('-') || base.len() > 2 { base } else { arg.as_str() }
                    } else {
                        arg.as_str()
                    };

                    let mapped = if let Some(custom) = config.package_overrides.get(clean_pkg) {
                        custom.clone()
                    } else {
                        mappings.translate_fedora_pkg(clean_pkg)
                    };
                    pkgs.push(mapped);
                }
            }

            if has_rpm {
                return TranslationResult {
                    command: format!("rpmextract {}", pkgs.join(" ")),
                    exec_binary: "rpmextract".to_string(),
                    exec_args: pkgs.clone(),
                    op: PacmanOp::RpmExtract { files: pkgs },
                    needs_root: false,
                    needs_aur: true,
                    notes_en: ".rpm files require extraction or finding native AUR packages.".to_string(),
                    notes_fa: "فایل‌های .rpm باید با rpmextract استخراج شوند یا از مخزن AUR نصب گردند.".to_string(),
                    warning: Some("Direct .rpm installation is not supported by pacman.".to_string()),
                };
            }

            if pkgs.is_empty() {
                return TranslationResult {
                    command: if helper == "pacman" { "sudo pacman -S".to_string() } else { format!("{} -S", helper) },
                    exec_binary: if helper == "pacman" { "sudo".to_string() } else { helper.clone() },
                    exec_args: if helper == "pacman" { vec!["pacman".to_string(), "-S".to_string()] } else { vec!["-S".to_string()] },
                    op: PacmanOp::SyncInstall { pkgs: vec![], noconfirm: false, as_deps: false, download_only: false },
                    needs_root: helper == "pacman",
                    needs_aur: helper != "pacman",
                    notes_en: "Pacman install command.".to_string(),
                    notes_fa: "دستور نصب بسته با پَک‌من.".to_string(),
                    warning: None,
                };
            }

            let base_flag = if download_only { "-Sw" } else { "-S" };
            let mut exec_args = vec![base_flag.to_string()];
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
            cmd_parts.push(base_flag.to_string());
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
                    download_only,
                },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: format!("Installs package(s) via Arch repos / AUR ({}).", helper),
                notes_fa: format!("نصب بسته(ها) با استفاده از {}.", helper),
                warning: None,
            }
        }
        "remove" | "erase" => {
            let pkgs: Vec<String> = rest.iter().filter(|a| !a.starts_with('-')).cloned().collect();
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--assumeyes");

            let mut exec_args = vec!["-Rns".to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }
            exec_args.extend(pkgs.clone());

            let mut cmd_str = format!("sudo pacman -Rns {}", pkgs.join(" "));
            if noconfirm {
                cmd_str.push_str(" --noconfirm");
            }

            TranslationResult {
                command: cmd_str,
                exec_binary: "sudo".to_string(),
                exec_args: {
                    let mut a = vec!["pacman".to_string()];
                    a.extend(exec_args);
                    a
                },
                op: PacmanOp::Remove {
                    pkgs,
                    noconfirm,
                    cascade: false,
                    nosave: true,
                    recursive: true,
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes package and unneeded dependencies.".to_string(),
                notes_fa: "حذف بسته به همراه وابستگی‌های بدون مصرف.".to_string(),
                warning: None,
            }
        }
        "autoremove" => {
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--assumeyes");
            let mut cmd_str = "sudo pacman -Rns $(pacman -Qtdq)".to_string();
            if noconfirm {
                cmd_str.push_str(" --noconfirm");
            }

            TranslationResult {
                command: cmd_str,
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-Qtdq".to_string()],
                op: PacmanOp::RemoveOrphans { noconfirm },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes orphaned packages.".to_string(),
                notes_fa: "حذف پکیج‌های بی‌استفاده و یتیم.".to_string(),
                warning: None,
            }
        }
        "search" => {
            let query = rest.iter().filter(|a| !a.starts_with('-')).cloned().collect::<Vec<_>>().join(" ");
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
                notes_fa: "مشاهده جزییات و مشخصات پکیج.".to_string(),
                warning: None,
            }
        }
        "clean" => TranslationResult {
            command: "sudo pacman -Sc".to_string(),
            exec_binary: "sudo".to_string(),
            exec_args: vec!["pacman".to_string(), "-Sc".to_string()],
            op: PacmanOp::CleanCache { all: false },
            needs_root: true,
            needs_aur: false,
            notes_en: "Cleans cached package archives.".to_string(),
            notes_fa: "پاکسازی کش بسته‌ها.".to_string(),
            warning: None,
        },
        "provides" => {
            let file = rest.first().cloned().unwrap_or_default();
            TranslationResult {
                command: format!("pacman -F {}", file),
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-F".to_string(), file.clone()],
                op: PacmanOp::FileSearch { query: file },
                needs_root: false,
                needs_aur: false,
                notes_en: "Finds which package provides a specific file (requires 'pacman -Fy').".to_string(),
                notes_fa: "پیدا کردن بسته‌ای که فایل مورد نظر را ارائه می‌دهد.".to_string(),
                warning: None,
            }
        }
        other => TranslationResult {
            command: format!("pacman -S {}", other),
            exec_binary: "pacman".to_string(),
            exec_args: vec!["-S".to_string(), other.to_string()],
            op: PacmanOp::DirectPacman { args: vec!["-S".to_string(), other.to_string()] },
            needs_root: true,
            needs_aur: false,
            notes_en: format!("Attempting pacman operation for '{}'.", other),
            notes_fa: format!("تلاش برای اجرای معادل پَک‌من دستور '{}'.", other),
            warning: None,
        },
    }
}
