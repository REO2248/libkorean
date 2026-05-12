pub mod char_utils;
pub mod engine;
pub mod hanja;
pub mod input_context;
pub mod keyboard;

#[cfg(feature = "c-api")]
pub mod ffi;

pub fn char_to_jamo(c: char) -> Option<u16> {
    match c {
        'ㄱ' => Some(0x1100),
        'ㄲ' => Some(0x1101),
        'ㄴ' => Some(0x1102),
        'ㄷ' => Some(0x1103),
        'ㄸ' => Some(0x1104),
        'ㄹ' => Some(0x1105),
        'ㅁ' => Some(0x1106),
        'ㅂ' => Some(0x1107),
        'ㅃ' => Some(0x1108),
        'ㅅ' => Some(0x1109),
        'ㅆ' => Some(0x110A),
        'ㅇ' => Some(0x110B),
        'ㅈ' => Some(0x110C),
        'ㅉ' => Some(0x110D),
        'ㅊ' => Some(0x110E),
        'ㅋ' => Some(0x110F),
        'ㅌ' => Some(0x1110),
        'ㅍ' => Some(0x1111),
        'ㅎ' => Some(0x1112),
        'ㅏ' => Some(0x1161),
        'ㅐ' => Some(0x1162),
        'ㅑ' => Some(0x1163),
        'ㅒ' => Some(0x1164),
        'ㅓ' => Some(0x1165),
        'ㅔ' => Some(0x1166),
        'ㅕ' => Some(0x1167),
        'ㅖ' => Some(0x1168),
        'ㅗ' => Some(0x1169),
        'ㅘ' => Some(0x116A),
        'ㅙ' => Some(0x116B),
        'ㅚ' => Some(0x116C),
        'ㅛ' => Some(0x116D),
        'ㅜ' => Some(0x116E),
        'ㅝ' => Some(0x116F),
        'ㅞ' => Some(0x1170),
        'ㅟ' => Some(0x1171),
        'ㅠ' => Some(0x1172),
        'ㅡ' => Some(0x1173),
        'ㅢ' => Some(0x1174),
        'ㅣ' => Some(0x1175),
        'ㄳ' => Some(0x11AA),
        'ㄵ' => Some(0x11AC),
        'ㄶ' => Some(0x11AD),
        'ㄺ' => Some(0x11B0),
        'ㄻ' => Some(0x11B1),
        'ㄼ' => Some(0x11B2),
        'ㄽ' => Some(0x11B3),
        'ㄾ' => Some(0x11B4),
        'ㄿ' => Some(0x11B5),
        'ㅀ' => Some(0x11B6),
        'ㅄ' => Some(0x11B9),
        _ => None,
    }
}

pub fn tokenize(text: &str) -> Vec<u16> {
    let mut tokens = Vec::new();

    for c in text.chars() {
        if let Some(token) = char_to_jamo(c) {
            tokens.push(token);
        }
    }

    tokens
}

pub fn tokenize_with_mapping<F>(text: &str, mapping_func: F) -> Vec<u16>
where
    F: Fn(char) -> Option<u16>,
{
    text.chars().filter_map(mapping_func).collect()
}

pub fn tokenize_from_chars(chars: Vec<char>) -> Vec<u16> {
    chars.iter().filter_map(|&c| char_to_jamo(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_jamo() {
        assert_eq!(char_to_jamo('ㄱ'), Some(0x1100));
        assert_eq!(char_to_jamo('ㅏ'), Some(0x1161));
        assert_eq!(char_to_jamo('ㅂ'), Some(0x1107));
        assert_eq!(char_to_jamo('ㄳ'), Some(0x11AA));
        assert_eq!(char_to_jamo('Z'), None);
    }

    #[test]
    fn test_tokenize() {
        let result = tokenize("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅣㅂㄴㅣㄲㅏ");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tokenize_with_mapping() {
        let result = tokenize_with_mapping("ㅇㅏㄴㄴㅕㅇㅎㅏㅅㅣㅂㄴㅣㄲㅏ", char_to_jamo);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_tokenize_from_chars() {
        let chars = vec!['ㅇ', 'ㅏ', 'ㄴ', 'ㄴ', 'ㅕ', 'ㅇ', 'ㅎ', 'ㅏ', 'ㅅ', 'ㅣ', 'ㅂ', 'ㄴ', 'ㅣ', 'ㄲ', 'ㅏ'];
        let result = tokenize_from_chars(chars);
        assert!(!result.is_empty());
    }
}