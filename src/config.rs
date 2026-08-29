use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    #[serde(default)]
    pub package_overrides: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_helper")]
    pub helper: String,
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default)]
    pub auto_execute: bool,
    #[serde(default = "default_true")]
    pub colored_ui: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_true")]
    pub aur_fallback: bool,
    #[serde(default = "default_true")]
    pub bidi_isolation: bool,
}

fn default_helper() -> String {
    "paru".to_string()
}

fn default_language() -> String {
    "both".to_string()
}

fn default_theme() -> String {
    "neon".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                helper: default_helper(),
                language: default_language(),
                auto_execute: false,
                colored_ui: true,
                theme: default_theme(),
                aur_fallback: true,
                bidi_isolation: true,
            },
            package_overrides: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::load_from_path(None)
    }

    pub fn load_from_path(custom: Option<&str>) -> Self {
        if let Some(custom_file) = custom {
            let path = PathBuf::from(custom_file);
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str(&content) {
                        return cfg;
                    }
                }
            }
        }

        let user_config = dirs_config_path();
        if let Some(path) = user_config {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(cfg) = toml::from_str(&content) {
                        return cfg;
                    }
                }
            }
        }

        let sys_path = PathBuf::from("/etc/parch/helper.toml");
        if sys_path.exists() {
            if let Ok(content) = fs::read_to_string(&sys_path) {
                if let Ok(cfg) = toml::from_str(&content) {
                    return cfg;
                }
            }
        }

        Config::default()
    }
}

fn dirs_config_path() -> Option<PathBuf> {
    if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".config/parch/helper.toml"))
    } else {
        None
    }
}
