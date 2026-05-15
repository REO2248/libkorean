use korean::input_context::{입력문맥, 입력항목};

#[test]
fn test_noble_name_replacement() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.항목설정(입력항목::존함, true);

    // Test 김일성 -> U+F113 U+F114 U+F115
    // 김 (s k w) -> ㄱ+ㅣ+ㅁ
    문맥.처리('s');
    문맥.처리('k');
    문맥.처리('w');
    assert_eq!(문맥.편집문자렬(), "김");

    // 일 (d k r) -> ㅇ+ㅣ+ㄹ
    문맥.처리('d'); // 김 is committed (delayed in 존함기록)
    assert_eq!(문맥.결속문자렬(), "");
    문맥.처리('k');
    문맥.처리('r');
    assert_eq!(문맥.편집문자렬(), "김일");

    // 성 (g i d) -> ㅅ+ㅓ+ㅇ
    문맥.처리('g'); // 일 is committed (delayed in 존함기록)
    assert_eq!(문맥.결속문자렬(), "");
    문맥.처리('i');
    문맥.처리('d');
    assert_eq!(문맥.편집문자렬(), "\u{F113}\u{F114}\u{F115}");

    // Finalize 성 (by typing space)
    문맥.처리(' ');
    assert_eq!(문맥.결속문자렬(), "\u{F113}\u{F114}\u{F115} ");
}

#[test]
fn test_noble_name_replacement_split() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.항목설정(입력항목::존함, true);

    // 김정일 -> U+F116 U+F117 U+F118
    // s k w (김) -> ㄱ+ㅣ+ㅁ
    for c in "skw".chars() { 문맥.처리(c); }
    // a i d (정) -> ㅈ+ㅓ+ㅇ
    for c in "aid".chars() { 문맥.처리(c); }
    // d k r (일) -> ㅇ+ㅣ+ㄹ
    for c in "dkr".chars() { 문맥.처리(c); }

    assert_eq!(문맥.편집문자렬(), "\u{F116}\u{F117}\u{F118}");
    문맥.처리(' ');
    assert_eq!(문맥.결속문자렬(), "\u{F116}\u{F117}\u{F118} ");
}

#[test]
fn test_noble_name_replacement_split_3() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.항목설정(입력항목::존함, true);

    // 김정은 -> U+F120 U+F121 U+F122
    // s k w (김) -> ㄱ+ㅣ+ㅁ
    for c in "skw".chars() { 문맥.처리(c); }
    // a i d (정) -> ㅈ+ㅓ+ㅇ
    for c in "aid".chars() { 문맥.처리(c); }
    // d l f (은) -> ㅇ+ㅡ+ㄴ
    for c in "dlf".chars() { 문맥.처리(c); }

    assert_eq!(문맥.편집문자렬(), "\u{F120}\u{F121}\u{F122}");
    문맥.처리(' ');
    assert_eq!(문맥.결속문자렬(), "\u{F120}\u{F121}\u{F122} ");
}

#[test]
fn test_noble_name_non_matching() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.항목설정(입력항목::존함, true);

    // 김밥 (s k w q j q)
    for c in "skw".chars() { 문맥.처리(c); }
    문맥.처리('q'); // 김 is moved to 존함기록, 'ㅂ' starts in preedit
    assert_eq!(문맥.결속문자렬(), "");
    assert_eq!(문맥.편집문자렬(), "김ㅂ");
    문맥.처리('j'); // ㅂ + ㅏ -> 바
    assert_eq!(문맥.편집문자렬(), "김바");
    문맥.처리('q'); // 김바 + ㅂ -> 김밥
    assert_eq!(문맥.편집문자렬(), "김밥");

    // Typing space will 비우기 김밥
    문맥.처리(' ');
    assert_eq!(문맥.결속문자렬(), "김밥 ");
}

#[test]
fn test_noble_name_backspace() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.항목설정(입력항목::존함, true);

    // 김 (s k w)
    for c in "skw".chars() { 문맥.처리(c); }
    // Typing 'd' (ㅇ) starts next syllable, 김 moves to 존함기록
    문맥.처리('d');
    assert_eq!(문맥.결속문자렬(), "");
    assert_eq!(문맥.편집문자렬(), "김ㅇ");

    // Backspace pops 'ㅇ'
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "김");

    // Backspace again pops 'w' (ㅁ) from '김', leaving 'ㄱ+ㅣ'
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "기");
}
