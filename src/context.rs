use is_terminal::IsTerminal;
use std::env;
use std::io::stdin;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceManager {
    Apt,
    AptGet,
    AptCache,
    Aptitude,
    Dnf,
    Yum,
    Apk,
    Zypper,
    Brew,
    Flatpak,
    Snap,
    Dpkg,
    Rpm,
    Unknown(String),
}

impl SourceManager {
    pub fn from_name(name: &str) -> Self {
        let clean = Path::new(name)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(name);

        match clean {
            "apt" => SourceManager::Apt,
            "apt-get" => SourceManager::AptGet,
            "apt-cache" => SourceManager::AptCache,
            "aptitude" => SourceManager::Aptitude,
            "dnf" => SourceManager::Dnf,
            "yum" => SourceManager::Yum,
            "apk" => SourceManager::Apk,
            "zypper" => SourceManager::Zypper,
            "brew" => SourceManager::Brew,
            "flatpak" => SourceManager::Flatpak,
            "snap" => SourceManager::Snap,
            "dpkg" => SourceManager::Dpkg,
            "rpm" => SourceManager::Rpm,
            other => SourceManager::Unknown(other.to_string()),
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            SourceManager::Apt => "apt (Debian/Ubuntu)",
            SourceManager::AptGet => "apt-get (Debian/Ubuntu)",
            SourceManager::AptCache => "apt-cache (Debian/Ubuntu)",
            SourceManager::Aptitude => "aptitude (Debian)",
            SourceManager::Dnf => "dnf (Fedora/RHEL)",
            SourceManager::Yum => "yum (CentOS/RHEL)",
            SourceManager::Apk => "apk (Alpine)",
            SourceManager::Zypper => "zypper (openSUSE)",
            SourceManager::Brew => "brew (Homebrew/macOS)",
            SourceManager::Flatpak => "flatpak (Sandboxed App)",
            SourceManager::Snap => "snap (Canonical Snap)",
            SourceManager::Dpkg => "dpkg (Debian Package)",
            SourceManager::Rpm => "rpm (RedHat Package)",
            SourceManager::Unknown(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserIdentity {
    pub uid: u32,
    pub gid: u32,
    pub username: String,
    pub home: String,
}

#[derive(Debug, Clone, Default)]
pub struct CliOptions {
    pub explain: bool,
    pub dry_run: bool,
    pub yes: bool,
    pub force_interactive: bool,
    pub json: bool,
    pub helper: Option<String>,
    pub theme: Option<String>,
    pub config_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InvocationContext {
    pub source: SourceManager,
    pub original_args: Vec<String>,
    pub is_sudo: bool,
    pub sudo_user: Option<UserIdentity>,
    pub is_interactive: bool,
    pub cli_opts: CliOptions,
}

impl InvocationContext {
    pub fn capture() -> (CliOptions, Self) {
        let raw_args: Vec<String> = env::args().collect();
        let mut cli_opts = CliOptions::default();
        let mut pass_args: Vec<String> = Vec::new();
        let mut idx = 1;

        while idx < raw_args.len() {
            let arg = &raw_args[idx];
            match arg.as_str() {
                "-e" | "--explain" => cli_opts.explain = true,
                "-d" | "--dry-run" => cli_opts.dry_run = true,
                "-y" | "--yes" => cli_opts.yes = true,
                "-i" | "--interactive" => cli_opts.force_interactive = true,
                "-j" | "--json" => cli_opts.json = true,
                "-H" | "--helper" if idx + 1 < raw_args.len() => {
                    idx += 1;
                    cli_opts.helper = Some(raw_args[idx].clone());
                }
                "-t" | "--theme" if idx + 1 < raw_args.len() => {
                    idx += 1;
                    cli_opts.theme = Some(raw_args[idx].clone());
                }
                "-c" | "--config" if idx + 1 < raw_args.len() => {
                    idx += 1;
                    cli_opts.config_path = Some(raw_args[idx].clone());
                }
                _ => pass_args.push(arg.clone()),
            }
            idx += 1;
        }

        let prog_name = raw_args.first().cloned().unwrap_or_default();
        let mut source = SourceManager::from_name(&prog_name);

        if let SourceManager::Unknown(ref s) = source {
            if (s.contains("parch-helper") || s.contains("parch-translate"))
                && !pass_args.is_empty()
            {
                source = SourceManager::from_name(&pass_args[0]);
                pass_args.remove(0);
            }
        }

        let is_sudo = is_root();
        let sudo_user = if is_sudo {
            Self::resolve_sudo_target_user()
        } else {
            None
        };
        let is_interactive = if cli_opts.force_interactive {
            true
        } else {
            stdin().is_terminal()
        };

        let ctx = InvocationContext {
            source,
            original_args: pass_args,
            is_sudo,
            sudo_user,
            is_interactive,
            cli_opts: cli_opts.clone(),
        };

        (cli_opts, ctx)
    }

    fn resolve_sudo_target_user() -> Option<UserIdentity> {
        let sudo_uid = env::var("SUDO_UID")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
        if let Some(uid) = sudo_uid {
            if uid != 0 {
                #[cfg(unix)]
                unsafe {
                    let pwd = libc::getpwuid(uid);
                    if !pwd.is_null() {
                        let p = &*pwd;
                        let username = std::ffi::CStr::from_ptr(p.pw_name)
                            .to_string_lossy()
                            .into_owned();
                        let home = std::ffi::CStr::from_ptr(p.pw_dir)
                            .to_string_lossy()
                            .into_owned();
                        return Some(UserIdentity {
                            uid: p.pw_uid,
                            gid: p.pw_gid,
                            username,
                            home,
                        });
                    }
                }
            }
        }
        None
    }
}

fn is_root() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}
