use super::mapper::PkgMappings;
use super::{PacmanOp, TranslationResult};
use crate::config::Config;

pub fn translate_flatpak(args: &[String], config: &Config) -> TranslationResult {
    let helper = &config.general.helper;
    let mappings = PkgMappings::global();

    if args.is_empty() {
        return TranslationResult {
            command: "flatpak update".to_string(),
            exec_binary: "flatpak".to_string(),
            exec_args: vec!["update".to_string()],
            op: PacmanOp::DirectPacman {
                args: vec!["update".to_string()],
            },
            needs_root: false,
            needs_aur: false,
            notes_en: "Updating installed Flatpak runtimes and applications.".to_string(),
            notes_fa: "به‌روزرسانی ران‌تایم‌ها و برنامه‌های نصب‌شده فلت‌پک.".to_string(),
            warning: None,
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
            let noconfirm = rest.iter().any(|a| a == "-y" || a == "--assumeyes");

            if pkgs.is_empty() {
                return TranslationResult {
                    command: "flatpak install".to_string(),
                    exec_binary: "flatpak".to_string(),
                    exec_args: vec!["install".to_string()],
                    op: PacmanOp::DirectPacman {
                        args: vec!["install".to_string()],
                    },
                    needs_root: false,
                    needs_aur: false,
                    notes_en: "Flatpak install command.".to_string(),
                    notes_fa: "دستور نصب برنامه فلت‌پک.".to_string(),
                    warning: None,
                };
            }

            let mut exec_args = vec!["-S".to_string()];
            if noconfirm {
                exec_args.push("--noconfirm".to_string());
            }
            exec_args.extend(pkgs.clone());

            let full_cmd = if helper == "pacman" {
                let mut cmd = format!("sudo pacman -S {}", pkgs.join(" "));
                if noconfirm {
                    cmd.push_str(" --noconfirm");
                }
                cmd
            } else {
                let mut cmd = format!("{} -S {}", helper, pkgs.join(" "));
                if noconfirm {
                    cmd.push_str(" --noconfirm");
                }
                cmd
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
                op: PacmanOp::SyncInstall {
                    pkgs,
                    noconfirm,
                    as_deps: false,
                    download_only: false,
                },
                needs_root: helper == "pacman",
                needs_aur: helper != "pacman",
                notes_en: format!(
                    "Preferring native Arch/AUR packages over Flatpak container sandbox via {}.",
                    helper
                ),
                notes_fa: format!(
                    "پیشنهاد استفاده از بسته بومی آرچ/AUR با کارایی بالاتر از طریق {}.",
                    helper
                ),
                warning: Some("Native packages offer lower RAM overhead and seamless desktop integration.".to_string()),
            }
        }
        "update" => TranslationResult {
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
            notes_en: "Upgrading system packages.".to_string(),
            notes_fa: "ارتقا و به‌روزرسانی بسته‌های سیستم.".to_string(),
            warning: None,
        },
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
                    nosave: false,
                    recursive: true,
                },
                needs_root: true,
                needs_aur: false,
                notes_en: "Removes native packages and unneeded dependencies.".to_string(),
                notes_fa: "حذف پکیج‌ها و وابستگی‌های آنها.".to_string(),
                warning: None,
            }
        }
        other => TranslationResult {
            command: format!("flatpak {}", args.join(" ")),
            exec_binary: "flatpak".to_string(),
            exec_args: args.to_vec(),
            op: PacmanOp::DirectPacman {
                args: args.to_vec(),
            },
            needs_root: false,
            needs_aur: false,
            notes_en: format!("Passing command directly to Flatpak: '{}'.", other),
            notes_fa: format!("اجرای مستقیم دستور در Flatpak: '{}'.", other),
            warning: None,
        },
    }
}
