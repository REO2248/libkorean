use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct 한자 {
    pub key: String,
    pub value: String,
    pub comment: Option<String>,
}

pub struct 한자사전 {
    map: fst::Map<Vec<u8>>,
    file: File,
}

impl 한자사전 {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file.try_clone()?);
        let mut entries = Vec::new();
        let mut last_key = String::new();
        let mut offset: u64 = 0;

        let mut line = String::new();
        loop {
            line.clear();
            let bytes = reader.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                offset += bytes as u64;
                continue;
            }

            if let Some(key_end) = line.find(':') {
                let key = &line[..key_end];
                if key != last_key {
                    entries.push((key.to_string(), offset));
                    last_key = key.to_string();
                }
            }
            offset += bytes as u64;
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let map = fst::Map::from_iter(entries).map_err(std::io::Error::other)?;

        Ok(Self { map, file })
    }

    pub fn match_exact(&self, key: &str) -> Option<Vec<한자>> {
        let file_offset = self.map.get(key)?;
        self.read_entries_from_offset(file_offset, Some(key))
    }

    pub fn match_prefix(&self, key: &str) -> Vec<한자> {
        let mut results = Vec::new();
        let chars: Vec<char> = key.chars().collect();

        for len in (1..=chars.len()).rev() {
            let sub: String = chars[..len].iter().collect();
            if let Some(entries) = self.match_exact(&sub) {
                results.extend(entries);
            }
        }

        results
    }

    pub fn match_suffix(&self, key: &str) -> Vec<한자> {
        let mut results = Vec::new();
        let chars: Vec<char> = key.chars().collect();

        for start in 0..chars.len() {
            let sub: String = chars[start..].iter().collect();
            if let Some(entries) = self.match_exact(&sub) {
                results.extend(entries);
            }
        }

        results
    }

    fn read_entries_from_offset(
        &self,
        offset: u64,
        filter_key: Option<&str>,
    ) -> Option<Vec<한자>> {
        let mut file = self.file.try_clone().ok()?;
        file.seek(SeekFrom::Start(offset)).ok()?;
        let mut reader = BufReader::new(file);

        let mut results = Vec::new();

        let mut line = String::new();
        loop {
            line.clear();
            let bytes = reader.read_line(&mut line).ok()?;
            if bytes == 0 {
                break;
            }

            let trimmed = line.trim_end();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let mut parts = trimmed.splitn(3, ':');
            let Some(entry_key) = parts.next() else {
                continue;
            };
            let Some(value) = parts.next() else {
                continue;
            };
            let comment = parts.next().map(str::to_string);

            if let Some(target) = filter_key {
                if entry_key > target {
                    break;
                }
                if entry_key == target {
                    results.push(한자 {
                        key: entry_key.to_string(),
                        value: value.to_string(),
                        comment,
                    });
                }
            } else {
                results.push(한자 {
                    key: entry_key.to_string(),
                    value: value.to_string(),
                    comment,
                });
            }
        }

        (!results.is_empty()).then_some(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_and_search() {
        let path = "data/hanja/hanja.txt";
        if !Path::new(path).exists() {
            return;
        }

        let dict = 한자사전::load(path).expect("failed to load hanja dict");

        let results = dict.match_prefix("ㄱ");
        let _ = results;
    }

    #[test]
    fn test_match_prefix_shortening() {
        let key = "abc";
        let chars: Vec<char> = key.chars().collect();
        let expected_lengths = vec![3, 2, 1];

        for &len in &expected_lengths {
            let sub: String = chars[..len].iter().collect();
            assert_eq!(sub.len(), len);
        }
    }
}
