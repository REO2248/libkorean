use std::ffi::CString;
use std::sync::OnceLock;

pub const SYSTEM_KEYBOARD_DIR: &str = "/usr/share/libkorean/keyboards";

#[derive(Debug, Clone)]
pub struct KeyboardLayout {
    pub id: String,
    pub name: String,
    pub id_cstr: CString,
    pub name_cstr: CString,
}

static REGISTRY: OnceLock<Vec<KeyboardLayout>> = OnceLock::new();

pub struct KeyboardRegistry;

impl KeyboardRegistry {
    pub fn list() -> impl Iterator<Item = &'static KeyboardLayout> {
        REGISTRY.get_or_init(Self::discover).iter()
    }

    pub fn get(id: &str) -> Option<KeyboardLayout> {
        Self::list().find(|l| l.id == id).cloned()
    }

    fn discover() -> Vec<KeyboardLayout> {
        let mut layouts = Vec::new();
        let mut seen = std::collections::HashSet::new();

        if let Ok(entries) = std::fs::read_dir(SYSTEM_KEYBOARD_DIR) {
            for entry in entries.filter_map(std::result::Result::ok) {
                let path = entry.path();
                if let Some(layout) = Self::layout_from_path(&path, &mut seen) {
                    layouts.push(layout);
                }
            }
        }

        let crate_data_dir = format!("{}/data/keyboards", env!("CARGO_MANIFEST_DIR"));
        if let Ok(entries) = std::fs::read_dir(&crate_data_dir) {
            for entry in entries.filter_map(std::result::Result::ok) {
                let path = entry.path();
                if let Some(layout) = Self::layout_from_path(&path, &mut seen) {
                    layouts.push(layout);
                }
            }
        }

        layouts.sort_by(|a, b| a.id.cmp(&b.id));
        layouts
    }

    fn layout_from_path(
        path: &std::path::Path,
        seen: &mut std::collections::HashSet<String>,
    ) -> Option<KeyboardLayout> {
        if path.extension().is_some_and(|ext| ext == "yaml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let id = stem.to_string();
                if seen.insert(id.clone()) {
                    let name = Self::read_display_name(path).unwrap_or_else(|| id.clone());
                    let id_cstr = CString::new(id.as_str()).unwrap_or_default();
                    let name_cstr = CString::new(name.as_str()).unwrap_or_default();
                    return Some(KeyboardLayout {
                        id,
                        name,
                        id_cstr,
                        name_cstr,
                    });
                }
            }
        }
        None
    }

    fn read_display_name(path: &std::path::Path) -> Option<String> {
        let content = std::fs::read_to_string(path).ok()?;
        for line in content.lines().take(10) {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("# name: ") {
                return Some(rest.trim().to_string());
            }
            if !line.starts_with('#') && !line.is_empty() {
                break;
            }
        }
        None
    }
}
