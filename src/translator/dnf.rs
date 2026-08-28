use super::mapper::PkgMappings;
use super::TranslationResult;
use crate::config::Config;

pub fn translate_dnf(args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if args.is_empty() {
        return TranslationResult {
            command: format!("sudo pacman -Syu"),
            args: vec!["-Syu".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Running pacman system upgrade. In Parch, pacman replaces DNF/YUM.".to_string(),
            notes_fa: "اجرای به‌روزرسانی سیستم. در پارچ از پَک‌من به جای DNF استفاده می‌شود.".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "check-update" => TranslationResult {
            command: "sudo pacman -Sy".to_string(),
            args: vec!["-Sy".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Refreshes mirror database.".to_string(),
            notes_fa: "به‌روزرسانی لیست مخازن.".to_string(),
            warning: None,
        },
        "upgrade" | "update" => TranslationResult {
            command: if helper == "pacman" { "sudo pacman -Syu".to_string() } else { format!("{} -Syu", helper) },
            args: vec!["-Syu".to_string()],
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Full system upgrade.".to_string(),
            notes_fa: "به‌روزرسانی کامل سیستم و تمام پکیج‌ها.".to_string(),
            warning: None,
        },
        "install" | "in" => {
            let mut pkgs = Vec::new();
            let mut has_rpm = false;

            for arg in rest {
                if arg.ends_with(".rpm") {
                    has_rpm = true;
                    pkgs.push(arg.clone());
                } else if !arg.starts_with('-') {
                    let mapped = if let Some(custom) = config.package_overrides.get(arg) {
                        custom.clone()
                    } else {
                        mappings.translate_fedora_pkg(arg)
                    };
                    pkgs.push(mapped);
                }
            }

            if has_rpm {
                return TranslationResult {
                    command: format!("rpmextract {}", pkgs.join(" ")),
                    args: pkgs,
                    needs_root: false,
                    needs_aur: true,
                    notes_en: ".rpm files require extraction or finding native AUR packages.".to_string(),
                    notes_fa: "فایل‌های .rpm باید اکسترکت شوند یا از مخزن AUR نصب گردند.".to_string(),
                    warning: Some("Direct .rpm installation is not supported by pacman.".to_string()),
                };
            }

            let full_cmd = if helper == "pacman" {
                format!("sudo pacman -S {}", pkgs.join(" "))
            } else {
                format!("{} -S {}", helper, pkgs.join(" "))
            };

            TranslationResult {
                command: full_cmd,
                args: {
                    let mut a = vec!["-S".to_string()];
                    a.extend(pkgs);
                    a
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
            TranslationResult {
                command: format!("sudo pacman -Rns {}", pkgs.join(" ")),
                args: {
                    let mut a = vec!["-Rns".to_string()];
                    a.extend(pkgs);
                    a
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes package and unneeded dependencies.".to_string(),
                notes_fa: "حذف بسته به همراه وابستگی‌های بدون مصرف.".to_string(),
                warning: None,
            }
        }
        "autoremove" => TranslationResult {
            command: "sudo pacman -Rns $(pacman -Qtdq)".to_string(),
            args: vec!["-Rns".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Removes orphaned packages.".to_string(),
            notes_fa: "حذف پکیج‌های بی‌استفاده و یتیم.".to_string(),
            warning: None,
        },
        "search" => {
            let query = rest.join(" ");
            let full_cmd = if helper == "pacman" {
                format!("pacman -Ss {}", query)
            } else {
                format!("{} -Ss {}", helper, query)
            };
            TranslationResult {
                command: full_cmd,
                args: vec!["-Ss".to_string(), query],
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
                args: vec!["-Si".to_string(), pkg],
                needs_root: false,
                needs_aur: false,
                notes_en: "Displays package details.".to_string(),
                notes_fa: "مشاهده جزییات و مشخصات پکیج.".to_string(),
                warning: None,
            }
        }
        "clean" => TranslationResult {
            command: "sudo pacman -Sc".to_string(),
            args: vec!["-Sc".to_string()],
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
                args: vec!["-F".to_string(), file],
                needs_root: false,
                needs_aur: false,
                notes_en: "Finds which package provides a specific file.".to_string(),
                notes_fa: "پیدا کردن بسته‌ای که فایل مورد نظر را ارائه می‌دهد.".to_string(),
                warning: None,
            }
        }
        other => TranslationResult {
            command: format!("pacman -S {}", other),
            args: vec!["-S".to_string(), other.to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: format!("Attempting pacman operation for '{}'.", other),
            notes_fa: format!("تلاش برای اجرای معادل پَک‌من دستور '{}'.", other),
            warning: None,
        },
    }
}
