use super::TranslationResult;
use crate::config::Config;

pub fn translate_zypper(args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;

    if args.is_empty() {
        return TranslationResult {
            command: "sudo pacman -Syu".to_string(),
            args: vec!["-Syu".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Running system upgrade (openSUSE 'zypper' -> Arch 'pacman').".to_string(),
            notes_fa: "ارتقای سیستم (تبدیل دستور zypper به پَک‌من).".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "ref" | "refresh" => TranslationResult {
            command: "sudo pacman -Sy".to_string(),
            args: vec!["-Sy".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Refreshes repository metadata.".to_string(),
            notes_fa: "به‌روزرسانی کش و متادیتای مخازن.".to_string(),
            warning: None,
        },
        "dup" | "dist-upgrade" | "up" | "update" => TranslationResult {
            command: if helper == "pacman" { "sudo pacman -Syu".to_string() } else { format!("{} -Syu", helper) },
            args: vec!["-Syu".to_string()],
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Full distribution upgrade.".to_string(),
            notes_fa: "ارتقای کلی سیستم.".to_string(),
            warning: None,
        },
        "in" | "install" => {
            let pkgs: Vec<String> = rest.iter().filter(|a| !a.starts_with('-')).cloned().collect();
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
                notes_en: format!("Installs package(s) via Arch / AUR ({}).", helper),
                notes_fa: format!("نصب بسته(ها) با استفاده از {}.", helper),
                warning: None,
            }
        }
        "rm" | "remove" => {
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
                notes_fa: "حذف پکیج و وابستگی‌ها.".to_string(),
                warning: None,
            }
        }
        "se" | "search" => {
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
                notes_fa: "نمایش اطلاعات پکیج.".to_string(),
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
