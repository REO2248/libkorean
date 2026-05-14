use korean::input_context::{InputContext, InputOption};

#[test]
fn test_automatic_old_jamo_mode() {
    // KPS9256 (modern) should NOT have old_jamo_mode enabled by default
    let ic_modern = InputContext::new("kps9256").expect("valid layout");
    assert!(!ic_modern.get_option(InputOption::OldJamo));

    // KPS9256_yetgulja (old) SHOULD have old_jamo_mode enabled automatically
    // because it contains old jamo like ᅝ (U+115D)
    let ic_yetgulja = InputContext::new("kps9256_yetgulja").expect("valid layout");
    assert!(ic_yetgulja.get_option(InputOption::OldJamo));
}

#[test]
fn test_yetgulja_combinations() {
    let mut ic = InputContext::new("kps9256_yetgulja").expect("valid layout");

    // In KPS9256_yetgulja:
    // f: ㄴ (니은)
    // S-f: ᅝ (니은히읗)

    // Process ㄴ (f) then ㄱ (s) -> should combine to ᄓ (첫소리::니은기윽)
    ic.process('f');
    assert_eq!(ic.preedit_string(), "ㄴ");
    ic.process('s');
    assert_eq!(ic.preedit_string(), "ᄓ");
}

#[test]
fn test_modern_no_yetgulja_combinations() {
    let mut ic = InputContext::new("kps9256").expect("valid layout");

    // In KPS9256:
    // f: ㄴ
    // s: ㄱ

    ic.process('f');
    assert_eq!(ic.preedit_string(), "ㄴ");
    ic.process('s');
    // Should NOT combine. Instead, 'ㄴ' is committed and 'ㄱ' starts a new char.
    assert_eq!(ic.get_commit_string(), "ㄴ");
    assert_eq!(ic.preedit_string(), "ㄱ");
}

#[test]
fn test_yetgulja_syllable_break_marker() {
    let mut ic = InputContext::new("kps9256_yetgulja").expect("valid layout");

    // KPS9256_yetgulja: j -> ㅏ, K -> 🄵 (marker, shift-k)
    // Marker should finalize current syllable/jamo composition but should not
    // appear in commit output.
    ic.process('j');
    assert_eq!(ic.preedit_string(), "ㅏ");

    ic.process('K'); // Press marker (shift-k)
    assert_eq!(ic.get_commit_string(), "ㅏ");
    assert_eq!(ic.preedit_string(), "");
}

#[test]
fn test_old_jamo_with_final_variant_works_as_final() {
    let mut ic = InputContext::new("kps9256_yetgulja").expect("valid layout");

    // s -> ㄱ, j -> ㅏ, W (shift-w) -> ㅿ
    // ㅿ should be usable as final when composition context expects final.
    ic.process('s');
    ic.process('j');
    ic.process('W');

    // Old final composition stays as conjoining sequence in composed preedit.
    assert!(ic.preedit_string().contains('ᇫ'));
}

#[test]
fn test_old_jamo_standalone_prefers_compat_display() {
    let mut ic = InputContext::new("kps9256_yetgulja").expect("valid layout");

    // W (shift-w) -> ㅿ (compat) -> internally mapped to old initial/final-capable jamo
    // but standalone preedit should stay compat where possible.
    ic.process('W');
    assert_eq!(ic.preedit_string(), "ㅿ");
}
