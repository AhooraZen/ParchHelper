use super::mapper::PkgMappings;
use super::TranslationResult;
use crate::config::Config;

pub fn translate_apt(raw_args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if raw_args.is_empty() {
        return TranslationResult {
            command: "sudo pacman -Syu".to_string(),
            args: vec!["-Syu".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Running pacman system upgrade. In Parch/Arch, pacman manages packages.".to_string(),
            notes_fa: "اجرای به‌روزرسانی کلی سیستم. در پارچ از پَک‌من استفاده می‌شود.".to_string(),
            warning: None,
        };
    }

    // Handle typo: "apt get <subcommand>" -> treat as "apt <subcommand>"
    let args: Vec<String> = if raw_args.first().map(|s| s.as_str()) == Some("get") {
        raw_args[1..].to_vec()
    } else {
        raw_args.to_vec()
    };

    if args.is_empty() {
        return TranslationResult {
            command: "sudo pacman -Syu".to_string(),
            args: vec!["-Syu".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Running pacman system upgrade.".to_string(),
            notes_fa: "اجرای به‌روزرسانی کلی سیستم.".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "update" | "upd" => TranslationResult {
            command: "sudo pacman -Sy".to_string(),
            args: vec!["-Sy".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Updates local package database mirrors.".to_string(),
            notes_fa: "به‌روزرسانی لیست و پایگاه‌داده بسته‌ها.".to_string(),
            warning: None,
        },
        "upgrade" | "upg" | "dist-upgrade" | "full-upgrade" => TranslationResult {
            command: if helper == "pacman" { "sudo pacman -Syu".to_string() } else { format!("{} -Syu", helper) },
            args: vec!["-Syu".to_string()],
            needs_root: helper == "pacman",
            needs_aur: helper != "pacman",
            notes_en: "Full system upgrade (Official repos + AUR).".to_string(),
            notes_fa: "ارتقا و به‌روزرسانی کامل کلیه برنامه‌ها و سیستم.".to_string(),
            warning: None,
        },
        "install" | "i" | "in" | "add" => {
            let mut pkgs = Vec::new();
            let mut has_deb = false;
            let mut noconfirm = false;

            for arg in rest {
                if arg == "-y" || arg == "--yes" || arg == "--assume-yes" {
                    noconfirm = true;
                } else if arg.ends_with(".deb") {
                    has_deb = true;
                    pkgs.push(arg.clone());
                } else if !arg.starts_with('-') {
                    let mapped = if let Some(custom) = config.package_overrides.get(arg) {
                        custom.clone()
                    } else {
                        mappings.translate_debian_pkg(arg)
                    };
                    pkgs.push(mapped);
                }
            }

            if has_deb {
                return TranslationResult {
                    command: format!("debtap {}", pkgs.join(" ")),
                    args: pkgs,
                    needs_root: false,
                    needs_aur: true,
                    notes_en: ".deb files require conversion via 'debtap' or finding native AUR packages.".to_string(),
                    notes_fa: "فایل‌های .deb دبیان باید با debtap تبدیل شوند یا از مخزن AUR نصب گردند.".to_string(),
                    warning: Some("Direct .deb installation is not supported by pacman.".to_string()),
                };
            }

            let mut final_args = vec!["-S".to_string()];
            if noconfirm {
                final_args.push("--noconfirm".to_string());
            }
            final_args.extend(pkgs.clone());

            let full_cmd = if helper == "pacman" {
                format!("sudo pacman -S {}", pkgs.join(" "))
            } else {
                format!("{} -S {}", helper, pkgs.join(" "))
            };

            TranslationResult {
                command: full_cmd,
                args: final_args,
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: format!("Installs package(s) via Arch repos / AUR ({}).", helper),
                notes_fa: format!("نصب بسته(ها) از طریق مخازن رسمی و مخزن کاربران ({}).", helper),
                warning: None,
            }
        }
        "remove" | "rm" | "uninstall" => {
            let pkgs: Vec<String> = rest.iter().filter(|a| !a.starts_with('-')).cloned().collect();
            TranslationResult {
                command: format!("sudo pacman -R {}", pkgs.join(" ")),
                args: {
                    let mut a = vec!["-R".to_string()];
                    a.extend(pkgs);
                    a
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes package while keeping configuration files.".to_string(),
                notes_fa: "حذف بسته با حفظ فایل‌های تنظیمات شخصی.".to_string(),
                warning: None,
            }
        }
        "purge" => {
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
                notes_en: "Recursively removes package, its unneeded dependencies, and config files.".to_string(),
                notes_fa: "حذف کامل بسته به همراه وابستگی‌های بلااستفاده و فایل‌های کانفیگ.".to_string(),
                warning: None,
            }
        }
        "autoremove" | "auto-remove" => TranslationResult {
            command: "sudo pacman -Rns $(pacman -Qtdq)".to_string(),
            args: vec!["-Rns".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Removes orphaned packages (dependencies no longer required).".to_string(),
            notes_fa: "حذف بسته‌های یتیم و وابستگی‌های غیرضروری باقیمانده.".to_string(),
            warning: None,
        },
        "search" | "find" => {
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
                notes_en: "Searches package database for keywords.".to_string(),
                notes_fa: "جستجوی نام و توضیحات بسته‌ها در مخازن.".to_string(),
                warning: None,
            }
        }
        "show" | "info" => {
            let pkg = rest.first().cloned().unwrap_or_default();
            TranslationResult {
                command: format!("pacman -Si {}", pkg),
                args: vec!["-Si".to_string(), pkg],
                needs_root: false,
                needs_aur: false,
                notes_en: "Displays detailed remote package metadata and dependencies.".to_string(),
                notes_fa: "نمایش مشخصات، اطلاعات و وابستگی‌های بسته در مخزن.".to_string(),
                warning: None,
            }
        }
        "clean" | "autoclean" => TranslationResult {
            command: "sudo pacman -Sc".to_string(),
            args: vec!["-Sc".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Cleans cached tarballs from /var/cache/pacman/pkg/.".to_string(),
            notes_fa: "پاکسازی کش بسته‌های دانلود شده.".to_string(),
            warning: None,
        },
        "list" => {
            if rest.iter().any(|a| a == "--installed") {
                TranslationResult {
                    command: "pacman -Qe".to_string(),
                    args: vec!["-Qe".to_string()],
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Lists explicitly installed packages on this system.".to_string(),
                    notes_fa: "لیست بسته‌های مستقیماً نصب شده توسط کاربر.".to_string(),
                    warning: None,
                }
            } else {
                TranslationResult {
                    command: "pacman -Sl".to_string(),
                    args: vec!["-Sl".to_string()],
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Lists all available packages in official repositories.".to_string(),
                    notes_fa: "لیست تمام بسته‌های موجود در مخازن.".to_string(),
                    warning: None,
                }
            }
        }
        "edit-sources" => TranslationResult {
            command: "sudo micro /etc/pacman.d/mirrorlist".to_string(),
            args: vec!["/etc/pacman.d/mirrorlist".to_string()],
            needs_root: true,
            needs_aur: false,
            notes_en: "Edits Arch/Parch mirrorlist configuration.".to_string(),
            notes_fa: "ویرایش آدرس میرورها و سرورهای مخازن پارچ.".to_string(),
            warning: None,
        },
        other => {
            TranslationResult {
                command: format!("pacman -S {}", other),
                args: vec!["-S".to_string(), other.to_string()],
                needs_root: true,
                needs_aur: false,
                notes_en: format!("Attempting standard pacman operation for '{}'.", other),
                notes_fa: format!("تلاش برای اجرای معادل پَک‌من دستور '{}'.", other),
                warning: None,
            }
        }
    }
}
