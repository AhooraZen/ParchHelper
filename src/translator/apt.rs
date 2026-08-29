use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_apt(raw_args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if raw_args.is_empty() {
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
            notes_en: "Running pacman system upgrade. In Parch/Arch, pacman/paru manages packages.".to_string(),
            notes_fa: "اجرای به‌روزرسانی کلی سیستم. در پارچ از پَک‌من یا پارو استفاده می‌شود.".to_string(),
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
            notes_en: "Running full system upgrade.".to_string(),
            notes_fa: "اجرای به‌روزرسانی کلی سیستم.".to_string(),
            warning: None,
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "update" | "upd" => TranslationResult {
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
            notes_en: "Updates local package database mirrors.".to_string(),
            notes_fa: "به‌روزرسانی لیست و پایگاه‌داده بسته‌ها.".to_string(),
            warning: Some("Caution: On Arch, always run full upgrade (pacman -Syu) to avoid partial-upgrade issues.".to_string()),
        },
        "upgrade" | "upg" | "dist-upgrade" | "full-upgrade" => {
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--yes" || a == "--assume-yes");
            let download_only = rest.iter().any(|a| a == "-d" || a == "--download-only");

            let pac_flag = if download_only { "-Syuw" } else { "-Syu" }.to_string();
            let mut exec_args = vec![pac_flag.clone()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }

            let full_cmd = if helper == "pacman" {
                let mut c = format!("sudo pacman {}", pac_flag);
                if noconfirm { c.push_str(" --noconfirm"); }
                c
            } else {
                let mut c = format!("{} {}", helper, pac_flag);
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
                notes_en: "Full system upgrade (Official repos + AUR).".to_string(),
                notes_fa: "ارتقا و به‌روزرسانی کامل کلیه برنامه‌ها و سیستم.".to_string(),
                warning: None,
            }
        }
        "install" | "i" | "in" | "add" => {
            let mut pkgs = Vec::new();
            let mut has_deb = false;
            let mut noconfirm = false;
            let mut download_only = false;
            let as_deps = false;

            for arg in rest {
                if arg == "-y" || arg == "--yes" || arg == "--assume-yes" {
                    noconfirm = true;
                } else if arg == "-d" || arg == "--download-only" {
                    download_only = true;
                } else if arg.ends_with(".deb") {
                    has_deb = true;
                    pkgs.push(arg.clone());
                } else if !arg.starts_with('-') {
                    // Check version pinning syntax like package=1.2.3 -> strip for arch
                    let clean_pkg = if let Some((base, _)) = arg.split_once('=') {
                        base
                    } else {
                        arg.as_str()
                    };

                    let mapped = if let Some(custom) = config.package_overrides.get(clean_pkg) {
                        custom.clone()
                    } else {
                        mappings.translate_debian_pkg(clean_pkg)
                    };
                    pkgs.push(mapped);
                }
            }

            if has_deb {
                return TranslationResult {
                    command: format!("debtap {}", pkgs.join(" ")),
                    exec_binary: "debtap".to_string(),
                    exec_args: pkgs.clone(),
                    op: PacmanOp::DebtapConvert { files: pkgs },
                    needs_root: false,
                    needs_aur: true,
                    notes_en: ".deb files require conversion via 'debtap' or finding native AUR packages.".to_string(),
                    notes_fa: "فایل‌های .deb دبیان باید با debtap تبدیل شوند یا از مخزن AUR نصب گردند.".to_string(),
                    warning: Some("Direct .deb installation is not supported natively by pacman.".to_string()),
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
                    notes_fa: "دستور نصب پکیج با پَک‌من.".to_string(),
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
                    as_deps,
                    download_only,
                },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: format!("Installs package(s) via Arch repos / AUR ({}).", helper),
                notes_fa: format!("نصب بسته(ها) از طریق مخازن رسمی و مخزن کاربران ({}).", helper),
                warning: None,
            }
        }
        "remove" | "rm" | "uninstall" => {
            let pkgs: Vec<String> = rest.iter().filter(|a| !a.starts_with('-')).cloned().collect();
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--yes");

            let mut exec_args = vec!["-R".to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }
            exec_args.extend(pkgs.clone());

            let mut cmd_str = format!("sudo pacman -R {}", pkgs.join(" "));
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
                    nosave: false,
                    recursive: false,
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
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--yes");

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
                notes_en: "Recursively removes package, unneeded dependencies, and config files.".to_string(),
                notes_fa: "حذف کامل بسته به همراه وابستگی‌های بلااستفاده و فایل‌های کانفیگ.".to_string(),
                warning: None,
            }
        }
        "autoremove" | "auto-remove" => {
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--yes");
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
                notes_en: "Removes orphaned packages (dependencies no longer required by any package).".to_string(),
                notes_fa: "حذف بسته‌های یتیم و وابستگی‌های غیرضروری باقیمانده.".to_string(),
                warning: None,
            }
        }
        "search" | "find" => {
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
                notes_en: "Searches package database for keywords.".to_string(),
                notes_fa: "جستجوی نام و توضیحات بسته‌ها در مخازن.".to_string(),
                warning: None,
            }
        }
        "show" | "info" => {
            let pkg = rest.first().cloned().unwrap_or_default();
            TranslationResult {
                command: format!("pacman -Si {}", pkg),
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-Si".to_string(), pkg.clone()],
                op: PacmanOp::SyncInfo { pkg },
                needs_root: false,
                needs_aur: false,
                notes_en: "Displays detailed remote package metadata and dependencies.".to_string(),
                notes_fa: "نمایش مشخصات، اطلاعات و وابستگی‌های بسته در مخزن.".to_string(),
                warning: None,
            }
        }
        "clean" | "autoclean" => TranslationResult {
            command: "sudo pacman -Sc".to_string(),
            exec_binary: "sudo".to_string(),
            exec_args: vec!["pacman".to_string(), "-Sc".to_string()],
            op: PacmanOp::CleanCache { all: false },
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
                    exec_binary: "pacman".to_string(),
                    exec_args: vec!["-Qe".to_string()],
                    op: PacmanOp::QueryList { explicit_only: true, foreign_only: false },
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Lists explicitly installed packages on this system.".to_string(),
                    notes_fa: "لیست بسته‌های مستقیماً نصب شده توسط کاربر.".to_string(),
                    warning: None,
                }
            } else if rest.iter().any(|a| a == "--upgradable") {
                TranslationResult {
                    command: "checkupdates".to_string(),
                    exec_binary: "checkupdates".to_string(),
                    exec_args: vec![],
                    op: PacmanOp::DirectPacman { args: vec!["checkupdates".to_string()] },
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Safely checks for available system updates without modifying sync databases.".to_string(),
                    notes_fa: "بررسی امن پکیج‌های قابل ارتقا بدون دستکاری دیتابیس مخازن.".to_string(),
                    warning: None,
                }
            } else {
                TranslationResult {
                    command: "pacman -Sl".to_string(),
                    exec_binary: "pacman".to_string(),
                    exec_args: vec!["-Sl".to_string()],
                    op: PacmanOp::QueryList { explicit_only: false, foreign_only: false },
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
            exec_binary: "sudo".to_string(),
            exec_args: vec!["micro".to_string(), "/etc/pacman.d/mirrorlist".to_string()],
            op: PacmanOp::EditMirrorlist,
            needs_root: true,
            needs_aur: false,
            notes_en: "Edits Arch/Parch mirrorlist configuration.".to_string(),
            notes_fa: "ویرایش آدرس میرورها و سرورهای مخازن پارچ.".to_string(),
            warning: None,
        },
        other => {
            let full_cmd = format!("pacman -S {}", other);
            TranslationResult {
                command: full_cmd,
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-S".to_string(), other.to_string()],
                op: PacmanOp::DirectPacman { args: vec!["-S".to_string(), other.to_string()] },
                needs_root: true,
                needs_aur: false,
                notes_en: format!("Attempting standard pacman operation for '{}'.", other),
                notes_fa: format!("تلاش برای اجرای معادل پَک‌من دستور '{}'.", other),
                warning: None,
            }
        }
    }
}
