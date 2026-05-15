use korean::input_context::{입력문맥, 입력항목};

fn 문맥생성(layout: &str) -> 입력문맥 {
    입력문맥::new(layout).expect("valid layout")
}

#[test]
fn test_word_unit_commit_backspace() {
    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(입력항목::단어단위확정, true);
    
    // guryd -> 수령
    // g (ㅅ)
    문맥.처리('g');
    assert_eq!(문맥.편집문자렬(), "ㅅ");
    
    // u (ㅜ)
    문맥.처리('u');
    assert_eq!(문맥.편집문자렬(), "수");
    
    // r (ㄹ)
    문맥.처리('r');
    assert_eq!(문맥.편집문자렬(), "술");
    
    // y (ㅕ)
    문맥.처리('y');
    assert_eq!(문맥.편집문자렬(), "수려"); // ㄹ moved to next syllable
    
    // d (ㅇ)
    문맥.처리('d');
    assert_eq!(문맥.편집문자렬(), "수령");
    
    // Backspace once -> "수려"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "수려");
    
    // Backspace twice -> "술"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "술");
    
    // Backspace thrice -> "수"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "수");
    
    // Backspace 4 times -> "ㅅ"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅅ");
    
    // Backspace 5 times -> ""
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
}
