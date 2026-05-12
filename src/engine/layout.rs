use std::collections::BTreeMap;

use crate::engine::KeyValue;

const KEYBOARD_SEARCH_PATHS: &[&str] = &[
    "/usr/share/libkorean/keyboards",
];

#[derive(Clone)]
pub struct Layout {
    map: BTreeMap<String, KeyValue>,
}

impl Layout {
    pub fn new(id: &str) -> Result<Self, crate::input_context::LayoutError> {
        let manifest_path = format!("{}/data/keyboards/{id}.yaml", env!("CARGO_MANIFEST_DIR"));
        if let Ok(content) = std::fs::read_to_string(&manifest_path) {
            return Self::from_yaml(&content);
        }

        let crate_path = format!("data/keyboards/{id}.yaml");
        if let Ok(content) = std::fs::read_to_string(&crate_path) {
            return Self::from_yaml(&content);
        }

        let sys_path = format!("{}/{id}.yaml", crate::keyboard::SYSTEM_KEYBOARD_DIR);
        if let Ok(content) = std::fs::read_to_string(&sys_path) {
            return Self::from_yaml(&content);
        }

        Err(crate::input_context::LayoutError::Unknown(id.into()))
    }

    pub fn from_file(path: &str) -> Result<Self, crate::input_context::LayoutError> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| crate::input_context::LayoutError::Unknown(path.into()))?;
        Self::from_yaml(&content)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, crate::input_context::LayoutError> {
        let mut map = BTreeMap::new();

        for line in yaml.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            if let Some((key_part, value_part)) = line.split_once(':') {
                let key_part = key_part.trim().trim_matches('\'').trim_matches('"');
                let value_part = value_part.trim().trim_matches('\'').trim_matches('"');

                let (key_str, is_shift) = key_part
                    .strip_prefix("S-")
                    .map_or((key_part, false), |stripped| (stripped, true));

                let final_key: String = match key_str {
                    "1" if is_shift => "!".to_string(),
                    "2" if is_shift => "@".to_string(),
                    "3" if is_shift => "#".to_string(),
                    "4" if is_shift => "$".to_string(),
                    "5" if is_shift => "%".to_string(),
                    "6" if is_shift => "^".to_string(),
                    "7" if is_shift => "&".to_string(),
                    "8" if is_shift => "*".to_string(),
                    "9" if is_shift => "(".to_string(),
                    "0" if is_shift => ")".to_string(),
                    "Minus" => (if is_shift { "_" } else { "-" }).to_string(),
                    "Equal" => (if is_shift { "+" } else { "=" }).to_string(),
                    "Backslash" => (if is_shift { "|" } else { "\\" }).to_string(),
                    "OpenBracket" => (if is_shift { "{" } else { "[" }).to_string(),
                    "CloseBracket" => (if is_shift { "}" } else { "]" }).to_string(),
                    "SemiColon" => (if is_shift { ":" } else { ";" }).to_string(),
                    "Quote" => (if is_shift { "\"" } else { "'" }).to_string(),
                    "Comma" => (if is_shift { "<" } else { "," }).to_string(),
                    "Period" => (if is_shift { ">" } else { "." }).to_string(),
                    "Slash" => (if is_shift { "?" } else { "/" }).to_string(),
                    "Grave" => (if is_shift { "~" } else { "`" }).to_string(),
                    _ if key_str.len() == 1 => {
                        let c = key_str.chars().next().unwrap();
                        if is_shift {
                            c.to_uppercase().to_string()
                        } else {
                            c.to_lowercase().to_string()
                        }
                    }
                    _ => key_str.to_string(),
                };

                if let Ok(kv) = value_part.parse::<KeyValue>() {
                    map.insert(final_key, kv);
                }
            }
        }

        Ok(Self { map })
    }

    pub fn lookup(&self, input: &str, case_insensitive: bool) -> Option<KeyValue> {
        if let Some(kv) = self.map.get(input).copied() {
            return Some(kv);
        }
        if case_insensitive {
            let lower = input.to_lowercase();
            for (k, v) in &self.map {
                if k.to_lowercase() == lower {
                    return Some(*v);
                }
            }
        }
        None
    }

    pub fn is_prefix(&self, input: &str, case_insensitive: bool) -> bool {
        if self.map
            .range(input.to_string()..)
            .any(|(k, _)| k.len() > input.len() && k.starts_with(input))
        {
            return true;
        }

        if case_insensitive {
            let lower = input.to_lowercase();
            for k in self.map.keys() {
                if k.len() > lower.len() && k.to_lowercase().starts_with(&lower) {
                    return true;
                }
            }
        }
        false
    }

    pub fn has_old_jamo(&self) -> bool {
        self.map.values().any(super::compose::KeyValue::has_old_jamo)
    }

    pub fn has_multi_char_keys(&self) -> bool {
        self.map.keys().any(|k| k.chars().count() > 1)
    }
}

pub fn discover_layouts() -> Vec<String> {
    let mut layouts = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for dir in KEYBOARD_SEARCH_PATHS {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.filter_map(std::result::Result::ok) {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "yaml") {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        if seen.insert(stem.to_string()) {
                            layouts.push(stem.to_string());
                        }
                    }
                }
            }
        }
    }

    let crate_data_dir = format!("{}/data/keyboards", env!("CARGO_MANIFEST_DIR"));
    if let Ok(entries) = std::fs::read_dir(&crate_data_dir) {
        for entry in entries.filter_map(std::result::Result::ok) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "yaml") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    if seen.insert(stem.to_string()) {
                        layouts.push(stem.to_string());
                    }
                }
            }
        }
    }

    layouts.sort();
    layouts
}
