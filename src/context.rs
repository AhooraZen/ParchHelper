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
            SourceManager::Brew => "brew (Homebrew)",
            SourceManager::Dpkg => "dpkg (Debian)",
            SourceManager::Rpm => "rpm (RedHat)",
            SourceManager::Unknown(s) => s.as_str(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvocationContext {
    pub source: SourceManager,
    pub original_args: Vec<String>,
    pub is_sudo: bool,
    pub sudo_user: Option<String>,
    pub is_interactive: bool,
}

impl InvocationContext {
    pub fn capture() -> Self {
        let mut raw_args: Vec<String> = env::args().collect();
        let prog_name = raw_args.first().cloned().unwrap_or_default();

        let mut source = SourceManager::from_name(&prog_name);

        // If invoked directly as "parch-helper <foreign-cmd> [args...]"
        if let SourceManager::Unknown(ref s) = source {
            if s.contains("parch-helper") || s.contains("parch-translate") {
                if raw_args.len() > 1 {
                    let sub = raw_args[1].clone();
                    source = SourceManager::from_name(&sub);
                    raw_args.remove(1);
                }
            }
        }

        let is_sudo = env::var("SUDO_USER").is_ok() || is_root();
        let sudo_user = env::var("SUDO_USER").ok();
        let is_interactive = stdin().is_terminal();

        let pass_args = if raw_args.len() > 1 {
            raw_args[1..].to_vec()
        } else {
            Vec::new()
        };

        InvocationContext {
            source,
            original_args: pass_args,
            is_sudo,
            sudo_user,
            is_interactive,
        }
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
