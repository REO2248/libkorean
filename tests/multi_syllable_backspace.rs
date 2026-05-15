use korean::input_context::{입력문맥, 입력항목};

fn 문맥생성(layout: &str) -> 입력문맥 {
    입력문맥::new(layout).expect("valid layout")
}

#[test]
fn test_multi_syllable_backspace_noble_name() {
    let mut 문맥 = 문맥생성("tubolsik");
    문맥.항목설정(입력항목::존함, true);

    // Type "김일" (R, L, A, D, L, F)
    let keys = vec!['r', 'l', 'a', 'd', 'l', 'f'];
    for k in keys {
        문맥.처리(k);
    }
    assert_eq!(문맥.편집문자렬(), "김일");

    // Backspace once -> "김이"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "김이");

    // Backspace again -> "김ㅇ"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "김ㅇ");

    // Backspace again -> "김"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "김");

    // Backspace again -> "기"
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "기");
    
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㄱ");

    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
}

#[test]
fn test_multi_syllable_backspace_standard() {
    let mut 문맥 = 문맥생성("tubolsik");
    // 존함 is false by default

    문맥.처리('r');
    문맥.처리('k');
    문맥.처리('s');
    assert_eq!(문맥.편집문자렬(), "간");
    
    // Start new syllable
    문맥.처리('r'); 
    assert_eq!(문맥.결속문자렬(), "간");
    assert_eq!(문맥.편집문자렬(), "ㄱ");
    
    // Backspace once -> ""
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
}
