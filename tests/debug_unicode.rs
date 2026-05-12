use korean::input_context::InputContext;

#[test]
fn debug_unicode() {
    // Test kps9256: s, k, w (ㄱ+ㅣ+ㅁ = 김)
    let mut ic = InputContext::new("kps9256").expect("valid layout");
    ic.process('s');
    ic.process('k');
    ic.process('w');
    let result = ic.preedit_string();

    assert_eq!(result, "김", "Expected 김 (U+AE40), got {:?}", result);
}
