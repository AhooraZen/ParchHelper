use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_snap(args: &[String], config: &Config) -> TranslationResult {
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
            notes_en: "System upgrade. Arch and Parch avoid Canonical Snap daemon overhead."
                .to_string(),
            notes_fa:
                "به‌روزرسانی سیستم. در پارچ از اسنپ به دلیل کندی و مصرف منابع بالا استفاده نمی‌شود."
                    .to_string(),
            warning: Some(
                "Snap daemon is not recommended on lightweight Arch/Parch systems.".to_string(),
            ),
        };
    }

    let subcmd = &args[0];
    let rest = &args[1..];

    match subcmd.as_str() {
        "install" => {
            let pkgs: Vec<String> = rest
                .iter()
                .filter(|a| !a.starts_with('-'))
                .map(|a| mappings.translate_debian_pkg(a))
                .collect();
            let is_classic = rest.iter().any(|a| a == "--classic");

            if pkgs.is_empty() {
                return TranslationResult {
                    command: "snap install".to_string(),
                    exec_binary: "snap".to_string(),
                    exec_args: vec!["install".to_string()],
                    op: PacmanOp::DirectPacman {
                        args: vec!["install".to_string()],
                    },
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Snap install command.".to_string(),
                    notes_fa: "دستور نصب اسنپ.".to_string(),
                    warning: None,
                };
            }

            let mut exec_args = vec!["-S".to_string()];
            exec_args.extend(pkgs.clone());

            let full_cmd = if helper == "pacman" {
                format!("sudo pacman -S {}", pkgs.join(" "))
            } else {
                format!("{} -S {}", helper, pkgs.join(" "))
            };

            let binary = if helper == "pacman" { "sudo" } else { helper };
            let final_args = if helper == "pacman" {
                let mut a = vec!["pacman".to_string()];
                a.extend(exec_args);
                a
            } else {
                exec_args
            };

            let note_extra = if is_classic {
                " (--classic tools like VSCode/Go are available as official Arch packages)"
            } else {
                ""
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
                notes_en: format!(
                    "Installing native Arch/AUR package via {}{}.",
                    helper, note_extra
                ),
                notes_fa: format!(
                    "نصب مستقیم از مخازن اصلی یا کاربران آرچ به جای Snap با استفاده از {}.",
                    helper
                ),
                warning: Some(
                    "Native packages do not mount loop devices or slow down boot time.".to_string(),
                ),
            }
        }
        "remove" => {
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
                    nosave: false,
                    recursive: true,
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes package and unneeded dependencies.".to_string(),
                notes_fa: "حذف کامل پکیج و وابستگی‌ها.".to_string(),
                warning: None,
            }
        }
        "refresh" => TranslationResult {
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
            notes_en: "Upgrading all installed system packages.".to_string(),
            notes_fa: "به‌روزرسانی تمامی بسته‌های سیستم.".to_string(),
            warning: None,
        },
        other => TranslationResult {
            command: format!("snap {}", args.join(" ")),
            exec_binary: "snap".to_string(),
            exec_args: args.to_vec(),
            op: PacmanOp::DirectPacman {
                args: args.to_vec(),
            },
            needs_root: false,
            needs_aur: false,
            notes_en: format!("Passing command to Snap: '{}'.", other),
            notes_fa: format!("ارسال دستور به Snap: '{}'.", other),
            warning: None,
        },
    }
}
