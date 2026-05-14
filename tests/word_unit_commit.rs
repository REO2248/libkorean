use korean::input_context::{InputContext, InputOption};

fn create_ic(layout: &str) -> InputContext {
    InputContext::new(layout).expect("valid layout")
}

#[test]
fn test_word_unit_commit_backspace() {
    let mut ic = create_ic("kps9256");
    ic.set_option(InputOption::WordUnitCommit, true);
    
    // guryd -> 수령
    // g (ㅅ)
    ic.process('g');
    assert_eq!(ic.preedit_string(), "ㅅ");
    
    // u (ㅜ)
    ic.process('u');
    assert_eq!(ic.preedit_string(), "수");
    
    // r (ㄹ)
    ic.process('r');
    assert_eq!(ic.preedit_string(), "술");
    
    // y (ㅕ)
    ic.process('y');
    assert_eq!(ic.preedit_string(), "수려"); // ㄹ moved to next syllable
    
    // d (ㅇ)
    ic.process('d');
    assert_eq!(ic.preedit_string(), "수령");
    
    // Backspace once -> "수려"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "수려");
    
    // Backspace twice -> "술"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "술");
    
    // Backspace thrice -> "수"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "수");
    
    // Backspace 4 times -> "ㅅ"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅅ");
    
    // Backspace 5 times -> ""
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
}
