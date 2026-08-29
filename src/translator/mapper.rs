use std::collections::HashMap;
use std::sync::OnceLock;

static MAPPINGS_JSON: &str = include_str!("../../data/pkg_mappings.json");

static MAPPINGS: OnceLock<PkgMappings> = OnceLock::new();

#[derive(Debug, serde::Deserialize)]
struct RawMappings {
    #[serde(default)]
    debian_to_arch: HashMap<String, String>,
    #[serde(default)]
    fedora_to_arch: HashMap<String, String>,
    #[serde(default)]
    alpine_to_arch: HashMap<String, String>,
    #[serde(default)]
    zypper_to_arch: HashMap<String, String>,
    #[serde(default)]
    brew_to_arch: HashMap<String, String>,
}

pub struct PkgMappings {
    pub debian: HashMap<String, String>,
    pub fedora: HashMap<String, String>,
    pub alpine: HashMap<String, String>,
    pub zypper: HashMap<String, String>,
    pub brew: HashMap<String, String>,
}

impl PkgMappings {
    pub fn global() -> &'static PkgMappings {
        MAPPINGS.get_or_init(|| {
            let raw: RawMappings = serde_json::from_str(MAPPINGS_JSON).unwrap_or(RawMappings {
                debian_to_arch: HashMap::new(),
                fedora_to_arch: HashMap::new(),
                alpine_to_arch: HashMap::new(),
                zypper_to_arch: HashMap::new(),
                brew_to_arch: HashMap::new(),
            });
            PkgMappings {
                debian: raw.debian_to_arch,
                fedora: raw.fedora_to_arch,
                alpine: raw.alpine_to_arch,
                zypper: raw.zypper_to_arch,
                brew: raw.brew_to_arch,
            }
        })
    }

    pub fn translate_debian_pkg(&self, name: &str) -> String {
        if let Some(target) = self.debian.get(name) {
            return target.clone();
        }

        // Heuristics for Debian packages:
        // lib<name>-dev -> <name>
        if let Some(rest) = name.strip_prefix("lib") {
            if let Some(inner) = rest.strip_suffix("-dev") {
                return inner.to_string();
            }
        }
        // <name>-dev -> <name>
        if let Some(inner) = name.strip_suffix("-dev") {
            return inner.to_string();
        }
        // python3-<name> -> python-<name>
        if let Some(stripped) = name.strip_prefix("python3-") {
            return format!("python-{}", stripped);
        }
        // fonts-<name> -> ttf-<name>
        if let Some(stripped) = name.strip_prefix("fonts-") {
            return format!("ttf-{}", stripped);
        }

        name.to_string()
    }

    pub fn translate_fedora_pkg(&self, name: &str) -> String {
        if let Some(target) = self.fedora.get(name) {
            return target.clone();
        }

        // <name>-devel -> <name>
        if let Some(inner) = name.strip_suffix("-devel") {
            return inner.to_string();
        }
        if let Some(stripped) = name.strip_prefix("python3-") {
            return format!("python-{}", stripped);
        }

        name.to_string()
    }

    pub fn translate_alpine_pkg(&self, name: &str) -> String {
        if let Some(target) = self.alpine.get(name) {
            return target.clone();
        }

        if let Some(stripped) = name.strip_prefix("py3-") {
            return format!("python-{}", stripped);
        }
        if let Some(inner) = name.strip_suffix("-dev") {
            return inner.to_string();
        }

        name.to_string()
    }

    pub fn translate_zypper_pkg(&self, name: &str) -> String {
        if let Some(target) = self.zypper.get(name) {
            return target.clone();
        }

        if let Some(inner) = name.strip_suffix("-devel") {
            return inner.to_string();
        }

        name.to_string()
    }

    pub fn translate_brew_pkg(&self, name: &str) -> String {
        if let Some(target) = self.brew.get(name) {
            return target.clone();
        }

        // Strip version suffix e.g. python@3.11 -> python
        if let Some((base, _)) = name.split_once('@') {
            if let Some(target) = self.brew.get(base) {
                return target.clone();
            }
            return base.to_string();
        }

        name.to_string()
    }
}
