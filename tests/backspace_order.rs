use korean::input_context::{InputContext};

fn create_ic(layout: &str) -> InputContext {
    InputContext::new(layout).expect("valid layout")
}

#[test]
fn test_backspace_input_order_kps9256() {
    // Case 1: go (ㅅ + ㅐ)
    let mut ic = create_ic("kps9256");
    ic.process('g'); // ㅅ
    ic.process('o'); // ㅐ
    assert_eq!(ic.preedit_string(), "새");
    
    // According to user, backspacing 'o' should remove 'ㅐ' entirely, leaving 'ㅅ'
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅅ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");

    // Case 2: gjk (ㅅ + ㅏ + ㅣ)
    let mut ic = create_ic("kps9256");
    ic.process('g'); // ㅅ
    ic.process('j'); // ㅏ
    ic.process('k'); // ㅣ
    assert_eq!(ic.preedit_string(), "새");
    
    // Backspacing 'k' should remove 'ㅣ', leaving 'ㅅㅏ'
    ic.backspace();
    assert_eq!(ic.preedit_string(), "사");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅅ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");

    // Case 3: Go (ㅆ + ㅐ) - S-G is 'ㅆ'
    let mut ic = create_ic("kps9256");
    ic.process('G'); // ㅆ
    ic.process('o'); // ㅐ
    assert_eq!(ic.preedit_string(), "쌔");
    
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅆ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");

    // Case 4: Gjk (ㅆ + ㅏ + ㅣ)
    let mut ic = create_ic("kps9256");
    ic.process('G'); // ㅆ
    ic.process('j'); // ㅏ
    ic.process('k'); // ㅣ
    assert_eq!(ic.preedit_string(), "쌔");
    
    ic.backspace();
    assert_eq!(ic.preedit_string(), "싸");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅆ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
    
    // Case 5: ggo (ㅅ + ㅅ + ㅐ) - with CombiOnDoubleStroke
    let mut ic = create_ic("kps9256");
    ic.set_option(korean::input_context::InputOption::CombiOnDoubleStroke, true);
    ic.process('g'); // ㅅ
    ic.process('g'); // ㅅ -> ㅆ
    ic.process('o'); // ㅐ
    assert_eq!(ic.preedit_string(), "쌔");
    
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅆ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "ㅅ");
    ic.backspace();
    assert_eq!(ic.preedit_string(), "");
}
