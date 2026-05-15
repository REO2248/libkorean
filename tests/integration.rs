//! Comprehensive integration tests

use korean::char_utils::{
    호환자모인가, 가운데소리인가, 끝소리인가, 소리마디를_첫소리로_변환, 소리마디인가,
    첫소리를_호환첫소리로_변환, 첫소리인가,
};
use korean::input_context::{입력문맥, 입력사건, 입력항목};

fn 문맥생성(layout: &str) -> 입력문맥 {
    입력문맥::new(layout).expect("valid layout")
}

// ============================================================================
// test_korean_ic_process_2 (KPS9256)
// ============================================================================

#[test]
fn test_korean_ic_process_2() {
    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('A');
    assert_eq!(문맥.결속문자렬(), "가");
    assert_eq!(문맥.편집문자렬(), "ㅉ");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('q');
    문맥.처리('i');
    문맥.처리('G');
    문맥.처리('l');
    assert_eq!(문맥.결속문자렬(), "버");
    assert_eq!(문맥.편집문자렬(), "쓰");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('w');
    문맥.처리('j');
    문맥.처리('r');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "맑");
    문맥.처리('h');
    assert_eq!(문맥.결속문자렬(), "말");
    assert_eq!(문맥.편집문자렬(), "고");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('s');
    문맥.처리('g');
    assert_eq!(문맥.편집문자렬(), "ㄳ");
    문맥.처리('j');
    assert_eq!(문맥.결속문자렬(), "ㄱ");
    assert_eq!(문맥.편집문자렬(), "사");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('G');
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "가");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㄱ");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('s');
    문맥.처리('g');
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㄱ");
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('w');
    문맥.처리('j');
    문맥.처리('r');
    문맥.처리('s');
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "말");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('d');
    문맥.처리('u');
    문맥.처리('p');
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "우");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('q');
    문맥.처리('q');
    문맥.처리('u');
    문맥.처리('p');
    문맥.처리('r');
    문맥.처리('s');
    for _ in 0..6 {
        문맥.지우기();
    }
    assert_eq!(문맥.편집문자렬(), "");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('Q');
    문맥.처리('u');
    문맥.처리('p');
    문맥.처리('r');
    문맥.처리('s');
    for _ in 0..5 {
        문맥.지우기();
    }
    assert_eq!(문맥.편집문자렬(), "");
}

// ============================================================================
// test_korean_ic_process_romaja (로마자)
// ============================================================================

#[test]
fn test_korean_ic_process_romaja() {
    let mut 문맥 = 문맥생성("romaja");
    문맥.처리('h');
    문맥.처리('a');
    문맥.처리('n');
    assert_eq!(문맥.편집문자렬(), "한");
    문맥.초기화();

    문맥.처리('a');
    assert_eq!(문맥.편집문자렬(), "아");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");

    문맥.초기화();
    문맥.처리('t');
    문맥.처리('t');
    assert_eq!(문맥.편집문자렬(), "ㄸ");

    문맥.초기화();
    문맥.처리('x');
    문맥.처리('x');
    assert_eq!(문맥.결속문자렬(), "으");
    assert_eq!(문맥.편집문자렬(), "ㅇ");

    문맥.초기화();
    문맥.처리('X');
    문맥.처리('y');
    assert_eq!(문맥.편집문자렬(), "지");

    문맥.초기화();
    문맥.처리('n');
    assert_eq!(문맥.편집문자렬(), "ㄴ");
    assert_eq!(문맥.결속문자렬(), "");
    let flushed = 문맥.비우기();
    assert_eq!(flushed, "ㄴ");
    assert_eq!(문맥.편집문자렬(), "");
}

#[test]
fn test_korean_ic_flush() {
    let mut 문맥 = 입력문맥::new("romaja").unwrap();
    문맥.처리('a');
    assert_eq!(문맥.편집문자렬(), "아");
    let flushed = 문맥.비우기();
    assert_eq!(flushed, "아");
    assert_eq!(문맥.편집문자렬(), "");

    문맥.처리('n');
    assert_eq!(문맥.편집문자렬(), "ㄴ");
    let flushed = 문맥.비우기();
    assert_eq!(flushed, "ㄴ");
}

#[test]
fn test_korean_ic_romaja_annyong() {
    let mut 문맥 = 문맥생성("romaja");
    문맥.초기화();
    let mut result = String::new();
    for c in "annyeonghasibnikka".chars() {
        문맥.처리(c);
        result.push_str(문맥.결속문자렬());
        문맥.결속문자렬_비우기();
    }
    result.push_str(&문맥.편집문자렬());
    assert_eq!(result, "안녕하십니까");
}

#[test]
fn test_korean_ic_romaja_phonetic() {
    let mut 문맥 = 문맥생성("romaja");
    문맥.초기화();
    let mut result = String::new();
    for c in "proGram".chars() {
        문맥.처리(c);
        result.push_str(문맥.결속문자렬());
        문맥.결속문자렬_비우기();
    }
    result.push_str(&문맥.편집문자렬());
    assert_eq!(result, "프로그람");
}

#[test]
fn test_korean_ic_romaja_double_consonants() {
    let mut 문맥 = 문맥생성("romaja");
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
        문맥.초기화();
        for c in input.chars() {
            문맥.처리(c);
        }
        assert_eq!(문맥.편집문자렬(), expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_korean_ic_romaja_vowel_combinations() {
    let mut 문맥 = 문맥생성("romaja");
    let input = "eobeoi";
    let mut result = String::new();
    for c in input.chars() {
        문맥.처리(c);
        result.push_str(문맥.결속문자렬());
        문맥.결속문자렬_비우기();
    }
    result.push_str(&문맥.편집문자렬());
    assert_eq!(result, "어버이");
}

#[test]
fn test_korean_ic_romaja_uppercase_break() {
    let mut 문맥 = 문맥생성("romaja");

    문맥.초기화();
    let mut result = String::new();
    for ch in "anA".chars() {
        문맥.처리(ch);
        result.push_str(문맥.결속문자렬());
    }
    result.push_str(&문맥.편집문자렬());
    assert_eq!(result, "안아");

    문맥.초기화();
    result.clear();
    for ch in "GeosEun".chars() {
        문맥.처리(ch);
        result.push_str(문맥.결속문자렬());
    }
    result.push_str(&문맥.편집문자렬());
    assert_eq!(result, "것은");
}

// ============================================================================
// test_korean_ic_auto_reorder
// ============================================================================

#[test]
fn test_korean_ic_auto_reorder() {
    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::자동재배치, true);
    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::자동재배치, true);
    문맥.처리('j');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::자동재배치, false);
    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::자동재배치, false);
    문맥.처리('j');
    문맥.처리('s');
    assert_eq!(문맥.결속문자렬(), "ㅏ");
    assert_eq!(문맥.편집문자렬(), "ㄱ");
}

// ============================================================================
// test_korean_ic_combi_on_double_stroke
// ============================================================================

#[test]
fn test_korean_ic_combi_on_double_stroke() {
    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('s');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "깎");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('q');
    문맥.처리('i');
    문맥.처리('g');
    문맥.처리('g');
    assert_eq!(문맥.편집문자렬(), "벘");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('q');
    문맥.처리('i');
    문맥.처리('g');
    문맥.처리('g');
    문맥.처리('l');
    assert_eq!(문맥.결속문자렬(), "벗");
    assert_eq!(문맥.편집문자렬(), "스");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('g');
    문맥.처리('g');
    assert_eq!(문맥.편집문자렬(), "갔");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, true);
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('g');
    문맥.처리('g');
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "갓");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('s');
    문맥.처리('s');
    assert_eq!(문맥.결속문자렬(), "ㄱ");
    assert_eq!(문맥.편집문자렬(), "ㄱ");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "각");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('s');
    문맥.처리('s');
    assert_eq!(문맥.결속문자렬(), "각");
    assert_eq!(문맥.편집문자렬(), "ㄱ");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('s');
    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "가");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::두번타건조합, false);
    문맥.처리('q');
    문맥.처리('i');
    문맥.처리('g');
    문맥.처리('g');
    assert_eq!(문맥.결속문자렬(), "벗");
    assert_eq!(문맥.편집문자렬(), "ㅅ");
}

// ============================================================================
// test_korean_ic_non_choseong_combi
// ============================================================================

#[test]
fn test_korean_ic_non_choseong_combi() {
    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::첫소리밖조합, true);
    문맥.처리('s');
    문맥.처리('g');
    assert_eq!(문맥.편집문자렬(), "ㄳ");
    문맥.처리('j');
    assert_eq!(문맥.결속문자렬(), "ㄱ");
    assert_eq!(문맥.편집문자렬(), "사");

    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::첫소리밖조합, false);
    문맥.처리('s');
    문맥.처리('g');
    assert_eq!(문맥.결속문자렬(), "ㄱ");
    assert_eq!(문맥.편집문자렬(), "ㅅ");
}

// ============================================================================
// test_korean_jamo_to_cjamo
// ============================================================================

#[test]
fn test_korean_jamo_to_cjamo() {
    assert_eq!(첫소리를_호환첫소리로_변환('\u{11F2}'), '\u{3183}');
    assert_eq!(첫소리를_호환첫소리로_변환('\u{A971}'), '\u{316F}');
    assert_eq!(첫소리를_호환첫소리로_변환('\u{D7F9}'), '\u{3149}');
}

// ============================================================================
// Additional edge cases & libkorean parity
// ============================================================================

#[test]
fn test_edge_cases() {
    let mut 문맥 = 문맥생성("kps9256");
    assert!(문맥.is_empty());
    assert_eq!(문맥.편집문자렬(), "");
    assert_eq!(문맥.비우기(), "");

    assert!(matches!(문맥.지우기(), 입력사건::없음));

    문맥.처리('1');
    assert_eq!(문맥.편집문자렬(), "");

    문맥.처리('s');
    문맥.처리('j');
    assert_eq!(문맥.비우기(), "가");
    assert_eq!(문맥.비우기(), "");
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
        let result = 소리마디를_첫소리로_변환(syl).unwrap();
        assert_eq!(result, expected, "Failed for {}", syl);
    }
}

#[test]
fn test_character_classification() {
    assert!(첫소리인가('\u{1100}'));
    assert!(첫소리인가('\u{1112}'));
    assert!(첫소리인가('\u{A960}'));
    assert!(!첫소리인가('\u{1161}'));

    assert!(가운데소리인가('\u{1161}'));
    assert!(가운데소리인가('\u{1175}'));
    assert!(가운데소리인가('\u{D7B0}'));
    assert!(!가운데소리인가('\u{1100}'));

    assert!(끝소리인가('\u{11A8}'));
    assert!(끝소리인가('\u{11C2}'));
    assert!(끝소리인가('\u{D7CB}'));
    assert!(!끝소리인가('\u{1100}'));

    assert!(소리마디인가('가'));
    assert!(소리마디인가('힣'));
    assert!(!소리마디인가('\u{1100}'));
    assert!(!소리마디인가('a'));

    assert!(호환자모인가('\u{3131}'));
    assert!(호환자모인가('\u{318E}'));
    assert!(!호환자모인가('\u{1100}'));
}
// ============================================================================
// Issue Reports
// ============================================================================

#[test]
fn test_repro_annyong_bug() {
    let mut 문맥 = 입력문맥::new("kps9256").unwrap();
    문맥.항목설정(입력항목::두번타건조합, true);

    문맥.처리('f');
    문맥.처리('k');
    assert_eq!(문맥.편집문자렬(), "니");

    문맥.처리('s');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "닊");

    문맥.처리('j');
    let mut output = 문맥.결속문자렬().to_string();
    output.push_str(&문맥.편집문자렬());
    assert_eq!(output, "닉가");
}

#[test]
fn test_composed_split() {
    let mut 문맥 = 입력문맥::new("kps9256").unwrap();
    문맥.항목설정(입력항목::두번타건조합, true);

    문맥.처리('f');
    문맥.처리('k');

    문맥.처리('s');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "닊");

    문맥.처리('j');
    let mut output = 문맥.결속문자렬().to_string();
    output.push_str(&문맥.편집문자렬());
    assert_eq!(output, "닉가");
}

#[test]
fn test_symbol_mapping() {
    let mut 문맥 = 입력문맥::new("kps9256").unwrap();
    문맥.처리('?');
    assert_eq!(문맥.결속문자렬(), "?");
}

#[test]
fn test_repro_ban_gab_seub_nida() {
    let mut ic = 입력문맥::new("romaja").unwrap();
    let input = "banGabseubnida";
    let mut output = String::new();
    for ch in input.chars() {
        if ic.처리(ch) {
            output.push_str(ic.결속문자렬());
        }
    }
    output.push_str(&ic.비우기());
    assert_eq!(output, "반갑습니다");
}

#[test]
fn test_preedit_noble_name_preservation() {
    let mut ic = 입력문맥::new("romaja").unwrap();
    ic.항목설정(입력항목::단어단위확정, true);
    
    // Type 'ban'
    ic.처리('b');
    ic.처리('a');
    ic.처리('n'); // Delayed
    assert_eq!(ic.편집문자렬(), "반");
    
    // Type 'b' -> 'ban' should stay in preedit
    ic.처리('b');
    // 'n' matched, '반' in state. 'b' delayed.
    assert_eq!(ic.편집문자렬(), "반ㅂ");
    
    ic.처리('a'); 
    // 'b' matched, '반' committed to history. 'ㅂ' + 'ㅏ' -> '바' in state.
    let preedit = ic.편집문자렬();
    assert_eq!(preedit, "반바");
}

