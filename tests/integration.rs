//! Comprehensive integration tests

use korean::char_utils::{
    initial_sound_to_compat_initial, is_cjamo, is_final, is_initial, is_medial, is_syllable,
    syllable_to_initial_sound,
};
use korean::input_context::{InputContext, InputEvent, InputOption};

fn create_ic(layout: &str) -> InputContext {
    InputContext::new(layout).expect("valid layout")
}

// ============================================================================
// test_korean_ic_process_2 (KPS9256)
// ============================================================================

#[test]
fn test_korean_ic_process_2() {
    let mut ic = create_ic("kps9256");
    ic.process('s');
    ic.process('j');
    ic.process('A');
    assert_eq!(ic.get_commit_string(), "가");
    assert_eq!(ic.preedit_string(), "ㅉ");

    let mut ic = create_ic("kps9256");
    ic.process('q');
    ic.process('i');
    ic.process('G');
    ic.process('l');
    assert_eq!(ic.get_commit_string(), "버");
    assert_eq!(ic.preedit_string(), "쓰");

    let mut ic = create_ic("kps9256");
    ic.process('w');
    ic.process('j');
    ic.process('r');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "맑");
    ic.process('h');
    assert_eq!(ic.get_commit_string(), "말");
    assert_eq!(ic.preedit_string(), "고");

    let mut ic = create_ic("kps9256");
    ic.process('s');
    ic.process('g');
    assert_eq!(ic.preedit_string(), "ㄳ");
    ic.process('j');
    assert_eq!(ic.get_commit_string(), "ㄱ");
    assert_eq!(ic.preedit_string(), "사");

    let mut ic = create_ic("kps9256");
    ic.process('s');
    ic.process('j');
    ic.process('G');
    ic.backspace();
    assert_eq!(ic.preedit_string(), "가");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㄱ");

    let mut ic = create_ic("kps9256");
    ic.process('s');
    ic.process('g');
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㄱ");
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.process('w');
    ic.process('j');
    ic.process('r');
    ic.process('s');
    ic.backspace();
    assert_eq!(ic.preedit_string(), "말");

    let mut ic = create_ic("kps9256");
    ic.process('d');
    ic.process('u');
    ic.process('p');
    ic.backspace();
    assert_eq!(ic.preedit_string(), "우");

    let mut ic = create_ic("kps9256");
    ic.process('q');
    ic.process('q');
    ic.process('u');
    ic.process('p');
    ic.process('r');
    ic.process('s');
    for _ in 0..6 {
        ic.backspace();
    }
    assert_eq!(ic.preedit_string(), "");

    let mut ic = create_ic("kps9256");
    ic.process('Q');
    ic.process('u');
    ic.process('p');
    ic.process('r');
    ic.process('s');
    for _ in 0..5 {
        ic.backspace();
    }
    assert_eq!(ic.preedit_string(), "");
}

// ============================================================================
// test_korean_ic_process_romaja (로마자)
// ============================================================================

#[test]
fn test_korean_ic_process_romaja() {
    let mut ic = create_ic("romaja");
    ic.process('h');
    ic.process('a');
    ic.process('n');
    assert_eq!(ic.preedit_string(), "한");
    ic.reset();

    ic.process('a');
    assert_eq!(ic.preedit_string(), "아");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");

    ic.reset();
    ic.process('t');
    ic.process('t');
    assert_eq!(ic.preedit_string(), "ㄸ");

    ic.reset();
    ic.process('x');
    ic.process('x');
    assert_eq!(ic.get_commit_string(), "으");
    assert_eq!(ic.preedit_string(), "ㅇ");

    ic.reset();
    ic.process('X');
    ic.process('y');
    assert_eq!(ic.preedit_string(), "지");

    ic.reset();
    ic.process('n');
    assert_eq!(ic.preedit_string(), "ㄴ");
    assert_eq!(ic.get_commit_string(), "");
    let flushed = ic.flush();
    assert_eq!(flushed, "ㄴ");
    assert_eq!(ic.preedit_string(), "");
}

#[test]
fn test_korean_ic_flush() {
    let mut ic = InputContext::new("romaja").unwrap();
    ic.process('a');
    assert_eq!(ic.preedit_string(), "아");
    let flushed = ic.flush();
    assert_eq!(flushed, "아");
    assert_eq!(ic.preedit_string(), "");

    ic.process('n');
    assert_eq!(ic.preedit_string(), "ㄴ");
    let flushed = ic.flush();
    assert_eq!(flushed, "ㄴ");
}

#[test]
fn test_korean_ic_romaja_annyong() {
    let mut ic = create_ic("romaja");
    ic.reset();
    let mut result = String::new();
    for c in "annyeonghasibnikka".chars() {
        ic.process(c);
        result.push_str(ic.get_commit_string());
        ic.clear_commit_string();
    }
    result.push_str(&ic.preedit_string());
    assert_eq!(result, "안녕하십니까");
}

#[test]
fn test_korean_ic_romaja_phonetic() {
    let mut ic = create_ic("romaja");
    ic.reset();
    let mut result = String::new();
    for c in "proGram".chars() {
        ic.process(c);
        result.push_str(ic.get_commit_string());
        ic.clear_commit_string();
    }
    result.push_str(&ic.preedit_string());
    assert_eq!(result, "프로그람");
}

#[test]
fn test_korean_ic_romaja_double_consonants() {
    let mut ic = create_ic("romaja");
    let cases = [
        ("kk", "ㄲ"),
        ("gg", "ㄲ"),
        ("tt", "ㄸ"),
        ("dd", "ㄸ"),
        ("pp", "ㅃ"),
        ("bb", "ㅃ"),
        ("ss", "ㅆ"),
        ("jj", "ㅉ"),
        ("zz", "ㅉ"),
        ("kka", "까"),
        ("gga", "까"),
        ("tta", "따"),
        ("dda", "따"),
        ("ppa", "빠"),
        ("bba", "빠"),
        ("ssa", "싸"),
        ("jja", "짜"),
        ("zza", "짜"),
    ];
    for (input, expected) in cases {
        ic.reset();
        for c in input.chars() {
            ic.process(c);
        }
        assert_eq!(ic.preedit_string(), expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_korean_ic_romaja_vowel_combinations() {
    let mut ic = create_ic("romaja");
    let input = "eobeoi";
    let mut result = String::new();
    for c in input.chars() {
        ic.process(c);
        result.push_str(ic.get_commit_string());
        ic.clear_commit_string();
    }
    result.push_str(&ic.preedit_string());
    assert_eq!(result, "어버이");
}

#[test]
fn test_korean_ic_romaja_uppercase_break() {
    let mut ic = create_ic("romaja");

    ic.reset();
    let mut result = String::new();
    for ch in "anA".chars() {
        ic.process(ch);
        result.push_str(ic.get_commit_string());
    }
    result.push_str(&ic.preedit_string());
    assert_eq!(result, "안아");

    ic.reset();
    result.clear();
    for ch in "GeosEun".chars() {
        ic.process(ch);
        result.push_str(ic.get_commit_string());
    }
    result.push_str(&ic.preedit_string());
    assert_eq!(result, "것은");
}

// ============================================================================
// test_korean_ic_auto_reorder
// ============================================================================

#[test]
fn test_korean_ic_auto_reorder() {
    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::AutoReorder, true);
    ic.process('s');
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::AutoReorder, true);
    ic.process('j');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::AutoReorder, false);
    ic.process('s');
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::AutoReorder, false);
    ic.process('j');
    ic.process('s');
    assert_eq!(ic.get_commit_string(), "ㅏ");
    assert_eq!(ic.preedit_string(), "ㄱ");
}

// ============================================================================
// test_korean_ic_combi_on_double_stroke
// ============================================================================

#[test]
fn test_korean_ic_combi_on_double_stroke() {
    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    ic.process('s');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "깎");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    ic.process('s');
    ic.process('s');
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('q');
    ic.process('i');
    ic.process('g');
    ic.process('g');
    assert_eq!(ic.preedit_string(), "벘");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('q');
    ic.process('i');
    ic.process('g');
    ic.process('g');
    ic.process('l');
    assert_eq!(ic.get_commit_string(), "벗");
    assert_eq!(ic.preedit_string(), "스");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('s');
    ic.process('j');
    ic.process('g');
    ic.process('g');
    assert_eq!(ic.preedit_string(), "갔");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, true);
    ic.process('s');
    ic.process('j');
    ic.process('g');
    ic.process('g');
    ic.backspace();
    assert_eq!(ic.preedit_string(), "갓");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('s');
    ic.process('s');
    assert_eq!(ic.get_commit_string(), "ㄱ");
    assert_eq!(ic.preedit_string(), "ㄱ");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "각");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    ic.process('s');
    ic.process('s');
    assert_eq!(ic.get_commit_string(), "각");
    assert_eq!(ic.preedit_string(), "ㄱ");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('s');
    ic.process('s');
    ic.process('j');
    ic.process('s');
    ic.process('s');
    ic.process('j');
    assert_eq!(ic.preedit_string(), "가");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::CombiOnDoubleStroke, false);
    ic.process('q');
    ic.process('i');
    ic.process('g');
    ic.process('g');
    assert_eq!(ic.get_commit_string(), "벗");
    assert_eq!(ic.preedit_string(), "ㅅ");
}

// ============================================================================
// test_korean_ic_non_choseong_combi
// ============================================================================

#[test]
fn test_korean_ic_non_choseong_combi() {
    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::NonChoseongCombi, true);
    ic.process('s');
    ic.process('g');
    assert_eq!(ic.preedit_string(), "ㄳ");
    ic.process('j');
    assert_eq!(ic.get_commit_string(), "ㄱ");
    assert_eq!(ic.preedit_string(), "사");

    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::NonChoseongCombi, false);
    ic.process('s');
    ic.process('g');
    assert_eq!(ic.get_commit_string(), "ㄱ");
    assert_eq!(ic.preedit_string(), "ㅅ");
}

// ============================================================================
// test_korean_jamo_to_cjamo
// ============================================================================

#[test]
fn test_korean_jamo_to_cjamo() {
    assert_eq!(initial_sound_to_compat_initial('\u{11F2}'), '\u{3183}');
    assert_eq!(initial_sound_to_compat_initial('\u{A971}'), '\u{316F}');
    assert_eq!(initial_sound_to_compat_initial('\u{D7F9}'), '\u{3149}');
}

// ============================================================================
// Additional edge cases & libkorean parity
// ============================================================================

#[test]
fn test_edge_cases() {
    let mut ic = create_ic("kps9256");
    assert!(ic.is_empty());
    assert_eq!(ic.preedit_string(), "");
    assert_eq!(ic.flush(), "");

    assert!(matches!(ic.backspace(), InputEvent::None));

    ic.process('1');
    assert_eq!(ic.preedit_string(), "");

    ic.process('s');
    ic.process('j');
    assert_eq!(ic.flush(), "가");
    assert_eq!(ic.flush(), "");
}

#[test]
fn test_syllable_decomposition() {
    let tests = vec![
        ('가', ('\u{1100}', '\u{1161}', None)),
        ('각', ('\u{1100}', '\u{1161}', Some('\u{11A8}'))),
        ('핵', ('\u{1112}', '\u{1162}', Some('\u{11A8}'))),
        ('힣', ('\u{1112}', '\u{1175}', Some('\u{11C2}'))),
    ];

    for (syl, expected) in tests {
        let result = syllable_to_initial_sound(syl).unwrap();
        assert_eq!(result, expected, "Failed for {}", syl);
    }
}

#[test]
fn test_character_classification() {
    assert!(is_initial('\u{1100}'));
    assert!(is_initial('\u{1112}'));
    assert!(is_initial('\u{A960}'));
    assert!(!is_initial('\u{1161}'));

    assert!(is_medial('\u{1161}'));
    assert!(is_medial('\u{1175}'));
    assert!(is_medial('\u{D7B0}'));
    assert!(!is_medial('\u{1100}'));

    assert!(is_final('\u{11A8}'));
    assert!(is_final('\u{11C2}'));
    assert!(is_final('\u{D7CB}'));
    assert!(!is_final('\u{1100}'));

    assert!(is_syllable('가'));
    assert!(is_syllable('힣'));
    assert!(!is_syllable('\u{1100}'));
    assert!(!is_syllable('a'));

    assert!(is_cjamo('\u{3131}'));
    assert!(is_cjamo('\u{318E}'));
    assert!(!is_cjamo('\u{1100}'));
}
// ============================================================================
// Issue Reports
// ============================================================================

#[test]
fn test_repro_annyong_bug() {
    let mut ic = InputContext::new("kps9256").unwrap();
    ic.set_option(InputOption::CombiOnDoubleStroke, true);

    ic.process('f');
    ic.process('k');
    assert_eq!(ic.preedit_string(), "니");

    ic.process('s');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "닊");

    ic.process('j');
    let mut output = ic.get_commit_string().to_string();
    output.push_str(&ic.preedit_string());
    assert_eq!(output, "닉가");
}

#[test]
fn test_composed_split() {
    let mut ic = InputContext::new("kps9256").unwrap();
    ic.set_option(InputOption::CombiOnDoubleStroke, true);

    ic.process('f');
    ic.process('k');

    ic.process('s');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "닊");

    ic.process('j');
    let mut output = ic.get_commit_string().to_string();
    output.push_str(&ic.preedit_string());
    assert_eq!(output, "닉가");
}

#[test]
fn test_symbol_mapping() {
    let mut ic = InputContext::new("kps9256").unwrap();
    ic.process('?');
    assert_eq!(ic.get_commit_string(), "?");
}
