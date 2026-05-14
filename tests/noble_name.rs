use korean::input_context::{InputContext, InputOption};

#[test]
fn test_noble_name_replacement() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.set_option(InputOption::NobleName, true);

    // Test 김일성 -> U+F113 U+F114 U+F115
    // 김 (s k w) -> ㄱ+ㅣ+ㅁ
    ic.process('s');
    ic.process('k');
    ic.process('w');
    assert_eq!(ic.preedit_string(), "김");

    // 일 (d k r) -> ㅇ+ㅣ+ㄹ
    ic.process('d'); // 김 is committed (delayed in noble_history)
    assert_eq!(ic.get_commit_string(), "");
    ic.process('k');
    ic.process('r');
    assert_eq!(ic.preedit_string(), "김일");

    // 성 (g i d) -> ㅅ+ㅓ+ㅇ
    ic.process('g'); // 일 is committed (delayed in noble_history)
    assert_eq!(ic.get_commit_string(), "");
    ic.process('i');
    ic.process('d');
    assert_eq!(ic.preedit_string(), "\u{F113}\u{F114}\u{F115}");

    // Finalize 성 (by typing space)
    ic.process(' ');
    assert_eq!(ic.get_commit_string(), "\u{F113}\u{F114}\u{F115} ");
}

#[test]
fn test_noble_name_replacement_split() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.set_option(InputOption::NobleName, true);

    // 김정일 -> U+F116 U+F117 U+F118
    // s k w (김) -> ㄱ+ㅣ+ㅁ
    for c in "skw".chars() { ic.process(c); }
    // a i d (정) -> ㅈ+ㅓ+ㅇ
    for c in "aid".chars() { ic.process(c); }
    // d k r (일) -> ㅇ+ㅣ+ㄹ
    for c in "dkr".chars() { ic.process(c); }

    assert_eq!(ic.preedit_string(), "\u{F116}\u{F117}\u{F118}");
    ic.process(' ');
    assert_eq!(ic.get_commit_string(), "\u{F116}\u{F117}\u{F118} ");
}

#[test]
fn test_noble_name_replacement_split_3() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.set_option(InputOption::NobleName, true);

    // 김정은 -> U+F120 U+F121 U+F122
    // s k w (김) -> ㄱ+ㅣ+ㅁ
    for c in "skw".chars() { ic.process(c); }
    // a i d (정) -> ㅈ+ㅓ+ㅇ
    for c in "aid".chars() { ic.process(c); }
    // d l f (은) -> ㅇ+ㅡ+ㄴ
    for c in "dlf".chars() { ic.process(c); }

    assert_eq!(ic.preedit_string(), "\u{F120}\u{F121}\u{F122}");
    ic.process(' ');
    assert_eq!(ic.get_commit_string(), "\u{F120}\u{F121}\u{F122} ");
}

#[test]
fn test_noble_name_non_matching() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.set_option(InputOption::NobleName, true);

    // 김밥 (s k w q j q)
    for c in "skw".chars() { ic.process(c); }
    ic.process('q'); // 김 is moved to noble_history, 'ㅂ' starts in preedit
    assert_eq!(ic.get_commit_string(), "");
    assert_eq!(ic.preedit_string(), "김ㅂ");
    ic.process('j'); // ㅂ + ㅏ -> 바
    assert_eq!(ic.preedit_string(), "김바");
    ic.process('q'); // 김바 + ㅂ -> 김밥
    assert_eq!(ic.preedit_string(), "김밥");

    // Typing space will flush 김밥
    ic.process(' ');
    assert_eq!(ic.get_commit_string(), "김밥 ");
}

#[test]
fn test_noble_name_backspace() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.set_option(InputOption::NobleName, true);

    // 김 (s k w)
    for c in "skw".chars() { ic.process(c); }
    // Typing 'd' (ㅇ) starts next syllable, 김 moves to noble_history
    ic.process('d');
    assert_eq!(ic.get_commit_string(), "");
    assert_eq!(ic.preedit_string(), "김ㅇ");

    // Backspace pops 'ㅇ'
    ic.backspace();
    assert_eq!(ic.preedit_string(), "김");

    // Backspace again pops 'w' (ㅁ) from '김', leaving 'ㄱ+ㅣ'
    ic.backspace();
    assert_eq!(ic.preedit_string(), "기");
}
