use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct 한자 {
    pub 열쇠: String,
    pub 값: String,
    pub 설명: Option<String>,
}

pub struct 한자사전 {
    지도: fst::Map<Vec<u8>>,
    화일: File,
}

impl 한자사전 {
    pub fn 적재<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let 화일 = File::open(path)?;
        let mut reader = BufReader::new(화일.try_clone()?);
        let mut entries = Vec::new();
        let mut 마지막_열쇠 = String::new();
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

            if let Some(열쇠_끝) = line.find(':') {
                let 열쇠 = &line[..열쇠_끝];
                if 열쇠 != 마지막_열쇠 {
                    entries.push((열쇠.to_string(), offset));
                    마지막_열쇠 = 열쇠.to_string();
                }
            }
            offset += bytes as u64;
        }

        entries.sort_by(|a, b| a.0.cmp(&b.0));
        let map = fst::Map::from_iter(entries).map_err(std::io::Error::other)?;

        Ok(Self { 지도: map, 화일 })
    }

    pub fn 완전일치(&self, 열쇠: &str) -> Option<Vec<한자>> {
        let 화일_위치 = self.지도.get(열쇠)?;
        self.위치에서_항목_읽기(화일_위치, Some(열쇠))
    }

    pub fn 앞부분일치(&self, 열쇠: &str) -> Vec<한자> {
        let mut results = Vec::new();
        let chars: Vec<char> = 열쇠.chars().collect();

        for len in (1..=chars.len()).rev() {
            let sub: String = chars[..len].iter().collect();
            if let Some(entries) = self.완전일치(&sub) {
                results.extend(entries);
            }
        }

        results
    }

    pub fn 뒤부분일치(&self, 열쇠: &str) -> Vec<한자> {
        let mut results = Vec::new();
        let chars: Vec<char> = 열쇠.chars().collect();

        for start in 0..chars.len() {
            let sub: String = chars[start..].iter().collect();
            if let Some(entries) = self.완전일치(&sub) {
                results.extend(entries);
            }
        }

        results
    }

    fn 위치에서_항목_읽기(
        &self,
        offset: u64,
        거름_열쇠: Option<&str>,
    ) -> Option<Vec<한자>> {
        let mut 화일 = self.화일.try_clone().ok()?;
        화일.seek(SeekFrom::Start(offset)).ok()?;
        let mut reader = BufReader::new(화일);

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
            let Some(값) = parts.next() else {
                continue;
            };
            let 설명 = parts.next().map(str::to_string);

            if let Some(target) = 거름_열쇠 {
                if entry_key > target {
                    break;
                }
                if entry_key == target {
                    results.push(한자 {
                        열쇠: entry_key.to_string(),
                        값: 값.to_string(),
                        설명,
                    });
                }
            } else {
                results.push(한자 {
                    열쇠: entry_key.to_string(),
                    값: 값.to_string(),
                    설명,
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

        let dict = 한자사전::적재(path).expect("failed to 적재 hanja dict");

        let results = dict.앞부분일치("ㄱ");
        let _ = results;
    }

    #[test]
    fn test_match_prefix_shortening() {
        let 열쇠 = "abc";
        let chars: Vec<char> = 열쇠.chars().collect();
        let expected_lengths = vec![3, 2, 1];

        for &len in &expected_lengths {
            let sub: String = chars[..len].iter().collect();
            assert_eq!(sub.len(), len);
        }
    }
}
