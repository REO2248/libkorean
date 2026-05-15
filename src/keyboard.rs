use std::ffi::CString;
use std::sync::OnceLock;

pub const 체계건반경로: &str = "/usr/share/libkorean/keyboards";

#[derive(Debug, Clone)]
pub struct 건반배렬 {
    pub id: String,
    pub name: String,
    pub id_cstr: CString,
    pub name_cstr: CString,
}

static REGISTRY: OnceLock<Vec<건반배렬>> = OnceLock::new();

pub struct 건반등록기;

impl 건반등록기 {
    pub fn 목록() -> impl Iterator<Item = &'static 건반배렬> {
        REGISTRY.get_or_init(Self::찾기).iter()
    }

    pub fn 획득(id: &str) -> Option<건반배렬> {
        Self::목록().find(|l| l.id == id).cloned()
    }

    fn 찾기() -> Vec<건반배렬> {
        let mut layouts = Vec::new();
        let mut seen = std::collections::HashSet::new();

        if let Ok(entries) = std::fs::read_dir(체계건반경로) {
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
    ) -> Option<건반배렬> {
        if path.extension().is_some_and(|ext| ext == "yaml") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                let id = stem.to_string();
                if seen.insert(id.clone()) {
                    let name = Self::read_display_name(path).unwrap_or_else(|| id.clone());
                    let id_cstr = CString::new(id.as_str()).unwrap_or_default();
                    let name_cstr = CString::new(name.as_str()).unwrap_or_default();
                    return Some(건반배렬 {
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
