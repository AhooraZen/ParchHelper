pub mod apk;
pub mod apt;
pub mod brew;
pub mod dnf;
pub mod mapper;
pub mod zypper;

use crate::config::Config;
use crate::context::{InvocationContext, SourceManager};

#[derive(Debug, Clone)]
pub struct TranslationResult {
    pub command: String,
    #[allow(dead_code)]
    pub args: Vec<String>,
    #[allow(dead_code)]
    pub needs_root: bool,
    pub needs_aur: bool,
    pub notes_en: String,
    pub notes_fa: String,
    pub warning: Option<String>,
}

pub fn translate(ctx: &InvocationContext, config: &Config) -> TranslationResult {
    match ctx.source {
        SourceManager::Apt | SourceManager::AptGet | SourceManager::AptCache | SourceManager::Aptitude => {
            apt::translate_apt(&ctx.original_args, config)
        }
        SourceManager::Dnf | SourceManager::Yum => dnf::translate_dnf(&ctx.original_args, config),
        SourceManager::Apk => apk::translate_apk(&ctx.original_args, config),
        SourceManager::Zypper => zypper::translate_zypper(&ctx.original_args, config),
        SourceManager::Brew => brew::translate_brew(&ctx.original_args, config),
        SourceManager::Dpkg => {
            let file = ctx.original_args.iter().find(|a| a.ends_with(".deb")).cloned().unwrap_or_default();
            TranslationResult {
                command: format!("debtap {}", file),
                args: vec![file],
                needs_root: false,
                needs_aur: true,
                notes_en: "DPKG package manager is Debian-specific. Convert package using 'debtap'.".to_string(),
                notes_fa: "مدیریت بسته dpkg مخصوص دبیان است. برای تبدیل فایل deb به آرچ از debtap استفاده کنید.".to_string(),
                warning: Some("Native Arch packages use .pkg.tar.zst format.".to_string()),
            }
        }
        SourceManager::Rpm => {
            let file = ctx.original_args.iter().find(|a| a.ends_with(".rpm")).cloned().unwrap_or_default();
            TranslationResult {
                command: format!("rpmextract {}", file),
                args: vec![file],
                needs_root: false,
                needs_aur: true,
                notes_en: "RPM packages are RedHat/Fedora specific. Use 'rpmextract' to unpack.".to_string(),
                notes_fa: "پکیج‌های RPM مخصوص ردهت هستند. از rpmextract برای بازکردن فایل استفاده کنید.".to_string(),
                warning: Some("Native Arch packages use .pkg.tar.zst format.".to_string()),
            }
        }
        SourceManager::Unknown(ref s) => {
            let full_input = format!("{} {}", s, ctx.original_args.join(" "));
            TranslationResult {
                command: format!("pacman -Ss {}", s),
                args: vec!["-Ss".to_string(), s.clone()],
                needs_root: false,
                needs_aur: false,
                notes_en: format!("Command '{}' not recognized as standard Arch package manager.", s),
                notes_fa: format!("دستور '{}' ابزار مدیریت بسته در پارچ/آرچ نیست.", s),
                warning: Some(format!("Input: '{}'", full_input)),
            }
        }
    }
}
