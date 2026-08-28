use std::path::Path;

pub struct SafetyGuard;

impl SafetyGuard {
    pub fn check_pacman_db_locked() -> bool {
        Path::new("/var/lib/pacman/db.lck").exists()
    }

    pub fn is_dangerous_command(cmd: &str) -> bool {
        let trimmed = cmd.trim();
        trimmed.contains("-Rns glibc")
            || trimmed.contains("-Rns systemd")
            || trimmed.contains("-Rns base")
            || trimmed.contains("-Rns linux")
            || trimmed.contains("rm -rf /")
    }
}
