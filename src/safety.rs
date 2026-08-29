use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockStatus {
    NotLocked,
    LockedByProcess(u32, String),
    StaleLock(u32),
    LockedUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DangerLevel {
    Safe,
    Warning(String),
    Blocked(String),
}

pub struct SafetyGuard;

impl SafetyGuard {
    const CRITICAL_PACKAGES: &'static [&'static str] = &[
        "glibc",
        "systemd",
        "base",
        "linux",
        "linux-lts",
        "linux-zen",
        "linux-hardened",
        "pacman",
        "paru",
        "yay",
        "filesystem",
        "bash",
        "coreutils",
        "shadow",
        "util-linux",
        "pam",
        "e2fsprogs",
        "sudo",
    ];

    const CONFLICTING_SERVICES: &'static [&'static str] = &[
        "packagekit.service",
        "pamac-daemon.service",
        "discover.service",
    ];

    pub fn check_readonly_fs(target_path: &str) -> Result<bool, String> {
        #[cfg(unix)]
        {
            let c_path = std::ffi::CString::new(target_path).map_err(|e| e.to_string())?;
            let mut stat: std::mem::MaybeUninit<libc::statvfs> = std::mem::MaybeUninit::uninit();

            let ret = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
            if ret != 0 {
                return Err(format!("statvfs failed on {}", target_path));
            }

            let stat = unsafe { stat.assume_init() };
            Ok((stat.f_flag & libc::ST_RDONLY) != 0)
        }
        #[cfg(not(unix))]
        {
            let _ = target_path;
            Ok(false)
        }
    }

    pub fn check_pacman_lock() -> LockStatus {
        let lock_path = Path::new("/var/lib/pacman/db.lck");
        if !lock_path.exists() {
            return LockStatus::NotLocked;
        }

        let pkg_bins = ["pacman", "paru", "yay", "pamac", "packagekitd"];
        for bin in &pkg_bins {
            if let Ok(output) = Command::new("pidof").arg(bin).output() {
                if output.status.success() {
                    let out_str = String::from_utf8_lossy(&output.stdout);
                    if let Some(pid_str) = out_str.split_whitespace().next() {
                        if let Ok(pid) = pid_str.parse::<u32>() {
                            return LockStatus::LockedByProcess(pid, bin.to_string());
                        }
                    }
                }
            }
        }

        if let Ok(content) = fs::read_to_string(lock_path) {
            if let Ok(pid) = content.trim().parse::<u32>() {
                if Path::new(&format!("/proc/{}", pid)).exists() {
                    let comm = fs::read_to_string(format!("/proc/{}/comm", pid))
                        .unwrap_or_else(|_| "unknown".to_string())
                        .trim()
                        .to_string();
                    return LockStatus::LockedByProcess(pid, comm);
                } else {
                    return LockStatus::StaleLock(pid);
                }
            }
        }

        if let Ok(metadata) = fs::metadata(lock_path) {
            let mtime = metadata.mtime();
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(mtime);

            if now - mtime > 7200 {
                return LockStatus::StaleLock(0);
            }
        }

        LockStatus::LockedUnknown
    }

    pub fn get_active_conflicting_services() -> Vec<String> {
        let mut active = Vec::new();
        for svc in Self::CONFLICTING_SERVICES {
            if let Ok(status) = Command::new("systemctl")
                .args(["is-active", "--quiet", svc])
                .status()
            {
                if status.success() {
                    active.push(svc.to_string());
                }
            }
        }
        active
    }

    pub fn evaluate_command_safety(binary: &str, args: &[String]) -> DangerLevel {
        let bin_name = Path::new(binary)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(binary);

        if bin_name == "rm" {
            let has_recursive = args
                .iter()
                .any(|a| a.contains('r') || a.contains('R') || a == "--recursive");
            let targets_root = args
                .iter()
                .any(|a| matches!(a.as_str(), "/" | "/*" | "/etc" | "/usr" | "/var" | "/boot"));
            if has_recursive && targets_root {
                return DangerLevel::Blocked(format!(
                    "Command 'rm' targets core system path: {:?}",
                    args
                ));
            }
        }

        if !matches!(bin_name, "pacman" | "paru" | "yay" | "pamac") {
            return DangerLevel::Safe;
        }

        let mut is_remove = false;
        let mut has_cascade = false;
        let mut has_nodeps = false;
        let mut targets: Vec<&str> = Vec::new();

        for arg in args {
            if arg.starts_with('-') && !arg.starts_with("--") {
                let chars: Vec<char> = arg.chars().skip(1).collect();
                if chars.contains(&'R') {
                    is_remove = true;
                }
                if chars.contains(&'c') {
                    has_cascade = true;
                }
                if chars.contains(&'d') {
                    has_nodeps = true;
                }
            } else if arg == "--remove" {
                is_remove = true;
            } else if arg == "--cascade" {
                has_cascade = true;
            } else if arg == "--nodeps" {
                has_nodeps = true;
            } else if !arg.starts_with('-') {
                targets.push(arg.as_str());
            }
        }

        if !is_remove {
            return DangerLevel::Safe;
        }

        for target in targets {
            let clean = target.split('/').last().unwrap_or(target);
            for critical in Self::CRITICAL_PACKAGES {
                if clean == *critical || clean.starts_with(&format!("{}-", critical)) {
                    return DangerLevel::Blocked(format!(
                        "Removal of protected core package '{}' is blocked.",
                        clean
                    ));
                }
            }
        }

        if has_cascade {
            return DangerLevel::Warning(
                "Cascade removal flag (-c/--cascade) removes all reverse dependencies.".to_string(),
            );
        }
        if has_nodeps {
            return DangerLevel::Warning(
                "Dependency check skip (-d/--nodeps) risks breaking shared libraries.".to_string(),
            );
        }

        DangerLevel::Safe
    }
}
