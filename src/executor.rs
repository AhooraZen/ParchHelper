use crate::context::{InvocationContext, UserIdentity};
use crate::safety::{DangerLevel, LockStatus, SafetyGuard};
use crate::translator::{PacmanOp, TranslationResult};
use std::ffi::CString;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

pub struct Executor;

impl Executor {
    pub fn run(ctx: &InvocationContext, res: &TranslationResult) -> Result<(), String> {
        if let Ok(true) = SafetyGuard::check_readonly_fs("/") {
            eprintln!(
                "\n\x1b[1;31m[!] CRITICAL ERROR: Root filesystem (/) is mounted READ-ONLY.\x1b[0m"
            );
            eprintln!("\x1b[31m[!] خطا: فایل‌سیستم ریشه به صورت فقط-خواندنی سوار شده است.\x1b[0m\n");
            return Err("Aborted: read-only root mount.".to_string());
        }

        match SafetyGuard::check_pacman_lock() {
            LockStatus::LockedByProcess(pid, proc_name) => {
                eprintln!(
                    "\n\x1b[1;33m[!] Warning: Pacman database is locked by {} (PID {}).\x1b[0m\n",
                    proc_name, pid
                );
            }
            LockStatus::StaleLock(pid) => {
                eprintln!(
                    "\n\x1b[1;33m[!] Notice: Found stale pacman lock file (/var/lib/pacman/db.lck, PID {} not running).\x1b[0m\n",
                    pid
                );
            }
            LockStatus::LockedUnknown => {
                eprintln!("\n\x1b[1;33m[!] Warning: Pacman lock file exists (/var/lib/pacman/db.lck).\x1b[0m\n");
            }
            LockStatus::NotLocked => {}
        }

        let conflicting = SafetyGuard::get_active_conflicting_services();
        if !conflicting.is_empty() {
            eprintln!(
                "\x1b[1;33m[!] Conflicting background services detected: {}\x1b[0m\n",
                conflicting.join(", ")
            );
        }

        if let PacmanOp::RemoveOrphans { noconfirm } = res.op {
            return Self::run_orphan_cleanup(ctx.is_sudo, noconfirm);
        }

        let mut binary = res.exec_binary.clone();
        let mut args = res.exec_args.clone();

        if ctx.is_sudo && binary == "sudo" && !args.is_empty() {
            binary = args.remove(0);
        }

        match SafetyGuard::evaluate_command_safety(&binary, &args) {
            DangerLevel::Blocked(reason) => {
                eprintln!(
                    "\n\x1b[1;31m[!] DANGEROUS OPERATION BLOCKED: {}\x1b[0m\n",
                    reason
                );
                return Err("Execution aborted for system safety.".to_string());
            }
            DangerLevel::Warning(w) => {
                eprintln!("\n\x1b[1;33m[!] WARNING: {}\x1b[0m\n", w);
            }
            DangerLevel::Safe => {}
        }

        if res.needs_aur && ctx.is_sudo {
            if let Some(target_user) = &ctx.sudo_user {
                Self::execute_as_user(target_user, &binary, &args)
            } else {
                Self::execute_direct(&binary, &args)
            }
        } else {
            Self::execute_direct(&binary, &args)
        }
    }

    fn run_orphan_cleanup(is_root: bool, noconfirm: bool) -> Result<(), String> {
        let output = Command::new("pacman")
            .args(["-Qtdq"])
            .output()
            .map_err(|e| e.to_string())?;

        let orphans_str = String::from_utf8_lossy(&output.stdout);
        let orphans: Vec<&str> = orphans_str.split_whitespace().collect();

        if orphans.is_empty() {
            println!("\x1b[1;32m✔ No orphaned packages found on system.\x1b[0m");
            println!("\x1b[36m✔ هیچ پکیج یتیمی در سیستم پیدا نشد.\x1b[0m");
            return Ok(());
        }

        println!(
            "\x1b[1;33m[!] Found {} orphaned package(s): {}\x1b[0m\n",
            orphans.len(),
            orphans.join(" ")
        );

        let mut args = vec!["-Rns".to_string()];
        if noconfirm {
            args.push("--noconfirm".to_string());
        }
        for o in orphans {
            args.push(o.to_string());
        }

        let (binary, final_args) = if is_root {
            ("pacman".to_string(), args)
        } else {
            let mut a = vec!["pacman".to_string()];
            a.extend(args);
            ("sudo".to_string(), a)
        };

        Self::execute_direct(&binary, &final_args)
    }

    fn execute_direct(binary: &str, args: &[String]) -> Result<(), String> {
        let status = Command::new(binary)
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|e| format!("Failed to execute '{}': {}", binary, e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Process exited with status {:?}", status.code()))
        }
    }

    fn execute_as_user(user: &UserIdentity, binary: &str, args: &[String]) -> Result<(), String> {
        let uid = user.uid;
        let gid = user.gid;
        let home = user.home.clone();
        let username = user.username.clone();

        let mut cmd = Command::new(binary);
        cmd.args(args)
            .env("HOME", &home)
            .env("USER", &username)
            .env("LOGNAME", &username)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        #[cfg(unix)]
        unsafe {
            cmd.pre_exec(move || {
                let c_user = CString::new(username.as_str()).map_err(|_| {
                    std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid username")
                })?;

                if libc::initgroups(c_user.as_ptr(), gid) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::setgid(gid) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                if libc::setuid(uid) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }

        let status = cmd
            .status()
            .map_err(|e| format!("Execution as user {} failed: {}", user.username, e))?;

        if status.success() {
            Ok(())
        } else {
            Err(format!("Process exited with status {:?}", status.code()))
        }
    }
}
