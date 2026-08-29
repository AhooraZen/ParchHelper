pub mod apk;
pub mod apt;
pub mod brew;
pub mod dnf;
pub mod flatpak;
pub mod mapper;
pub mod snap;
pub mod zypper;

use crate::config::Config;
use crate::context::{InvocationContext, SourceManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacmanOp {
    SyncInstall {
        pkgs: Vec<String>,
        noconfirm: bool,
        as_deps: bool,
        download_only: bool,
    },
    SyncSearch {
        query: String,
    },
    SyncInfo {
        pkg: String,
    },
    SyncRefresh {
        force: bool,
    },
    SyncUpgrade {
        noconfirm: bool,
        download_only: bool,
    },
    Remove {
        pkgs: Vec<String>,
        noconfirm: bool,
        cascade: bool,
        nosave: bool,
        recursive: bool,
    },
    RemoveOrphans {
        noconfirm: bool,
    },
    CleanCache {
        all: bool,
    },
    QueryList {
        explicit_only: bool,
        foreign_only: bool,
    },
    QueryFiles {
        pkg: String,
    },
    FileSearch {
        query: String,
    },
    EditMirrorlist,
    DebtapConvert {
        files: Vec<String>,
    },
    RpmExtract {
        files: Vec<String>,
    },
    DirectPacman {
        args: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResult {
    pub command: String,
    pub exec_binary: String,
    pub exec_args: Vec<String>,
    pub op: PacmanOp,
    pub needs_root: bool,
    pub needs_aur: bool,
    pub notes_en: String,
    pub notes_fa: String,
    pub warning: Option<String>,
}

pub fn translate(ctx: &InvocationContext, config: &Config) -> TranslationResult {
    match ctx.source {
        SourceManager::Apt
        | SourceManager::AptGet
        | SourceManager::AptCache
        | SourceManager::Aptitude => apt::translate_apt(&ctx.original_args, config),
        SourceManager::Dnf | SourceManager::Yum => dnf::translate_dnf(&ctx.original_args, config),
        SourceManager::Apk => apk::translate_apk(&ctx.original_args, config),
        SourceManager::Zypper => zypper::translate_zypper(&ctx.original_args, config),
        SourceManager::Brew => brew::translate_brew(&ctx.original_args, config),
        SourceManager::Flatpak => flatpak::translate_flatpak(&ctx.original_args, config),
        SourceManager::Snap => snap::translate_snap(&ctx.original_args, config),
        SourceManager::Dpkg => {
            let files: Vec<String> = ctx
                .original_args
                .iter()
                .filter(|a| a.ends_with(".deb"))
                .cloned()
                .collect();
            let file_str = if files.is_empty() {
                "<package.deb>".to_string()
            } else {
                files.join(" ")
            };
            TranslationResult {
                command: format!("debtap {}", file_str),
                exec_binary: "debtap".to_string(),
                exec_args: if files.is_empty() {
                    vec![]
                } else {
                    files.clone()
                },
                op: PacmanOp::DebtapConvert { files },
                needs_root: false,
                needs_aur: true,
                notes_en: "DPKG packages are Debian-specific. Convert to Arch packages using 'debtap'.".to_string(),
                notes_fa: "بسته‌های DPKG مخصوص دبیان هستند. با دستور debtap آن‌ها را به بسته آرچ تبدیل کنید.".to_string(),
                warning: Some("Native Arch packages use the .pkg.tar.zst format.".to_string()),
            }
        }
        SourceManager::Rpm => {
            let files: Vec<String> = ctx
                .original_args
                .iter()
                .filter(|a| a.ends_with(".rpm"))
                .cloned()
                .collect();
            let file_str = if files.is_empty() {
                "<package.rpm>".to_string()
            } else {
                files.join(" ")
            };
            TranslationResult {
                command: format!("rpmextract {}", file_str),
                exec_binary: "rpmextract".to_string(),
                exec_args: if files.is_empty() {
                    vec![]
                } else {
                    files.clone()
                },
                op: PacmanOp::RpmExtract { files },
                needs_root: false,
                needs_aur: true,
                notes_en: "RPM packages are RedHat-specific. Extract them using 'rpmextract' or find AUR PKGBUILDs.".to_string(),
                notes_fa: "پکیج‌های RPM مخصوص ردهت/فدورا هستند. با rpmextract آن‌ها را استخراج کنید یا از AUR نصب نمایید.".to_string(),
                warning: Some("Native Arch packages use the .pkg.tar.zst format.".to_string()),
            }
        }
        SourceManager::Unknown(ref s) => {
            let query = if ctx.original_args.is_empty() {
                s.clone()
            } else {
                format!("{} {}", s, ctx.original_args.join(" "))
            };
            TranslationResult {
                command: format!("pacman -Ss {}", s),
                exec_binary: "pacman".to_string(),
                exec_args: vec!["-Ss".to_string(), s.clone()],
                op: PacmanOp::SyncSearch { query: s.clone() },
                needs_root: false,
                needs_aur: false,
                notes_en: format!(
                    "Command '{}' is not recognized as a standard package manager. Searching Arch repositories for matches.",
                    s
                ),
                notes_fa: format!(
                    "دستور '{}' به عنوان پکیج‌منیجر شناخته نشد. در حال جستجوی مخازن پارچ/آرچ برای یافتن نام مشابه.",
                    s
                ),
                warning: Some(format!("Input command: '{}'", query)),
            }
        }
    }
}
