use korean::input_context::입력문맥;

#[test]
fn debug_unicode() {
    // Test kps9256: s, k, w (ㄱ+ㅣ+ㅁ = 김)
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");
    문맥.처리('s');
    문맥.처리('k');
    문맥.처리('w');
    let result = 문맥.편집문자렬();

    assert_eq!(result, "김", "Expected 김 (U+AE40), got {:?}", result);
}
