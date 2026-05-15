use korean::input_context::{InputContext, InputOption};

fn create_ic(layout: &str) -> InputContext {
    InputContext::new(layout).expect("valid layout")
}

#[test]
fn test_multi_syllable_backspace_noble_name() {
    let mut ic = create_ic("tubolsik");
    ic.set_option(InputOption::존함, true);

    // Type "김일" (R, L, A, D, L, F)
    let keys = vec!['r', 'l', 'a', 'd', 'l', 'f'];
    for k in keys {
        ic.process(k);
    }
    assert_eq!(ic.preedit_string(), "김일");

    // Backspace once -> "김이"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "김이");

    // Backspace again -> "김ㅇ"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "김ㅇ");

    // Backspace again -> "김"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "김");

    // Backspace again -> "기"
    ic.backspace();
    assert_eq!(ic.preedit_string(), "기");
    
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㄱ");

    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
}

#[test]
fn test_multi_syllable_backspace_standard() {
    let mut ic = create_ic("tubolsik");
    // 존함 is false by default

    ic.process('r');
    ic.process('k');
    ic.process('s');
    assert_eq!(ic.preedit_string(), "간");
    
    // Start new syllable
    ic.process('r'); 
    assert_eq!(ic.get_commit_string(), "간");
    assert_eq!(ic.preedit_string(), "ㄱ");
    
    // Backspace once -> ""
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
}
