use std::collections::HashMap;
use std::sync::OnceLock;

static MAPPINGS_JSON: &str = include_str!("../../data/pkg_mappings.json");

static MAPPINGS: OnceLock<PkgMappings> = OnceLock::new();

#[derive(Debug, serde::Deserialize)]
struct RawMappings {
    debian_to_arch: HashMap<String, String>,
    fedora_to_arch: HashMap<String, String>,
}

pub struct PkgMappings {
    pub debian: HashMap<String, String>,
    pub fedora: HashMap<String, String>,
}

impl PkgMappings {
    pub fn global() -> &'static PkgMappings {
        MAPPINGS.get_or_init(|| {
            let raw: RawMappings = serde_json::from_str(MAPPINGS_JSON).unwrap_or(RawMappings {
                debian_to_arch: HashMap::new(),
                fedora_to_arch: HashMap::new(),
            });
            PkgMappings {
                debian: raw.debian_to_arch,
                fedora: raw.fedora_to_arch,
            }
        })
    }

    pub fn translate_debian_pkg(&self, name: &str) -> String {
        if let Some(target) = self.debian.get(name) {
            return target.clone();
        }

        // Heuristics for Debian packages:
        // lib<name>-dev -> <name>
        if name.starts_with("lib") && name.ends_with("-dev") {
            let inner = &name[3..name.len() - 4];
            return inner.to_string();
        }
        // <name>-dev -> <name>
        if name.ends_with("-dev") {
            let inner = &name[..name.len() - 4];
            return inner.to_string();
        }
        // python3-<name> -> python-<name>
        if name.starts_with("python3-") {
            return format!("python-{}", &name[8..]);
        }
        // fonts-<name> -> ttf-<name>
        if name.starts_with("fonts-") {
            return format!("ttf-{}", &name[6..]);
        }

        name.to_string()
    }

    pub fn translate_fedora_pkg(&self, name: &str) -> String {
        if let Some(target) = self.fedora.get(name) {
            return target.clone();
        }

        // <name>-devel -> <name>
        if name.ends_with("-devel") {
            let inner = &name[..name.len() - 6];
            return inner.to_string();
        }
        if name.starts_with("python3-") {
            return format!("python-{}", &name[8..]);
        }

        name.to_string()
    }
}
