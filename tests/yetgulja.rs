use korean::input_context::{입력문맥, 입력항목};

#[test]
fn test_automatic_old_jamo_mode() {
    // KPS9256 (modern) should NOT have 옛글자방식 enabled by default
    let ic_modern = 입력문맥::new("kps9256").expect("valid layout");
    assert!(!ic_modern.항목획득(입력항목::옛글자방식));

    // KPS9256_yetgulja (old) SHOULD have 옛글자방식 enabled automatically
    // because it contains old jamo like ᅝ (U+115D)
    let ic_yetgulja = 입력문맥::new("kps9256_yetgulja").expect("valid layout");
    assert!(ic_yetgulja.항목획득(입력항목::옛글자방식));
}

#[test]
fn test_yetgulja_combinations() {
    let mut 문맥 = 입력문맥::new("kps9256_yetgulja").expect("valid layout");

    // In KPS9256_yetgulja:
    // f: ㄴ (니은)
    // S-f: ᅝ (니은히읗)

    // Process ㄴ (f) then ㄱ (s) -> should combine to ᄓ (첫소리::니은기윽)
    문맥.처리('f');
    assert_eq!(문맥.편집문자렬(), "ㄴ");
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "ᄓ");
}

#[test]
fn test_modern_no_yetgulja_combinations() {
    let mut 문맥 = 입력문맥::new("kps9256").expect("valid layout");

    // In KPS9256:
    // f: ㄴ
    // s: ㄱ

    문맥.처리('f');
    assert_eq!(문맥.편집문자렬(), "ㄴ");
    문맥.처리('s');
    // Should NOT combine. Instead, 'ㄴ' is committed and 'ㄱ' starts a new char.
    assert_eq!(문맥.결속문자렬(), "ㄴ");
    assert_eq!(문맥.편집문자렬(), "ㄱ");
}

#[test]
fn test_yetgulja_syllable_break_marker() {
    let mut 문맥 = 입력문맥::new("kps9256_yetgulja").expect("valid layout");

    // KPS9256_yetgulja: j -> ㅏ, K -> 🄵 (marker, shift-k)
    // Marker should finalize current syllable/jamo composition but should not
    // appear in commit output.
    문맥.처리('j');
    assert_eq!(문맥.편집문자렬(), "ㅏ");

    문맥.처리('K'); // Press marker (shift-k)
    assert_eq!(문맥.결속문자렬(), "ㅏ");
    assert_eq!(문맥.편집문자렬(), "");
}

#[test]
fn test_old_jamo_with_final_variant_works_as_final() {
    let mut 문맥 = 입력문맥::new("kps9256_yetgulja").expect("valid layout");

    // s -> ㄱ, j -> ㅏ, W (shift-w) -> ㅿ
    // ㅿ should be usable as final when composition context expects final.
    문맥.처리('s');
    문맥.처리('j');
    문맥.처리('W');

    // Old final composition stays as conjoining sequence in composed preedit.
    assert!(문맥.편집문자렬().contains('ᇫ'));
}

#[test]
fn test_old_jamo_standalone_prefers_compat_display() {
    let mut 문맥 = 입력문맥::new("kps9256_yetgulja").expect("valid layout");

    // W (shift-w) -> ㅿ (compat) -> internally mapped to old initial/final-capable jamo
    // but standalone preedit should stay compat where possible.
    문맥.처리('W');
    assert_eq!(문맥.편집문자렬(), "ㅿ");
}
