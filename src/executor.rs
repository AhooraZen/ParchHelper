use std::process::{Command, Stdio};
use crate::context::InvocationContext;
use crate::safety::SafetyGuard;
use crate::translator::TranslationResult;

pub struct Executor;

impl Executor {
    pub fn run(ctx: &InvocationContext, res: &TranslationResult) -> Result<(), String> {
        if SafetyGuard::check_pacman_db_locked() {
            eprintln!("\n\x1b[1;33m[!] Warning: Pacman database is locked (/var/lib/pacman/db.lck).\x1b[0m");
            eprintln!("\x1b[36m[!] هشدار: دیتابیس پَک‌من قفل است. احتمالا برنامه دیگری در حال استفاده است.\x1b[0m\n");
        }

        if SafetyGuard::is_dangerous_command(&res.command) {
            eprintln!("\n\x1b[1;31m[!] DANGEROUS OPERATION DETECTED: This command removes core system packages!\x1b[0m");
            eprintln!("\x1b[31m[!] عملیات خطرناک: این دستور بسته‌های اصلی سیستم را حذف می‌کند.\x1b[0m\n");
            return Err("Execution aborted for system safety.".to_string());
        }

        let is_root = ctx.is_sudo;
        let sudo_user = ctx.sudo_user.as_deref();

        // If target needs AUR helper (paru/yay) and we are under root/sudo, drop privileges
        if res.needs_aur && is_root && sudo_user.is_some() {
            let user = sudo_user.unwrap();
            let status = Command::new("su")
                .args(["-", user, "-c", &res.command])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(s) => {
                    if s.success() {
                        Ok(())
                    } else {
                        Err(format!("Process exited with status {:?}", s.code()))
                    }
                }
                Err(e) => Err(format!("Failed to execute command as {}: {}", user, e)),
            }
        } else {
            let parts: Vec<&str> = res.command.split_whitespace().collect();
            if parts.is_empty() {
                return Ok(());
            }

            let binary = parts[0];
            let cmd_args = &parts[1..];

            let status = Command::new(binary)
                .args(cmd_args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            match status {
                Ok(s) => {
                    if s.success() {
                        Ok(())
                    } else {
                        Err(format!("Process exited with status {:?}", s.code()))
                    }
                }
                Err(e) => Err(format!("Failed to execute '{}': {}", binary, e)),
            }
        }
    }
}
