use korean::input_context::{입력문맥};

fn 문맥생성(layout: &str) -> 입력문맥 {
    입력문맥::new(layout).expect("valid layout")
}

#[test]
fn test_backspace_input_order_kps9256() {
    // Case 1: go (ㅅ + ㅐ)
    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('g'); // ㅅ
    문맥.처리('o'); // ㅐ
    assert_eq!(문맥.편집문자렬(), "새");
    
    // According to user, backspacing 'o' should remove 'ㅐ' entirely, leaving 'ㅅ'
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅅ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");

    // Case 2: gjk (ㅅ + ㅏ + ㅣ)
    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('g'); // ㅅ
    문맥.처리('j'); // ㅏ
    문맥.처리('k'); // ㅣ
    assert_eq!(문맥.편집문자렬(), "새");
    
    // Backspacing 'k' should remove 'ㅣ', leaving 'ㅅㅏ'
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "사");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅅ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");

    // Case 3: Go (ㅆ + ㅐ) - S-G is 'ㅆ'
    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('G'); // ㅆ
    문맥.처리('o'); // ㅐ
    assert_eq!(문맥.편집문자렬(), "쌔");
    
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅆ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");

    // Case 4: Gjk (ㅆ + ㅏ + ㅣ)
    let mut 문맥 = 문맥생성("kps9256");
    문맥.처리('G'); // ㅆ
    문맥.처리('j'); // ㅏ
    문맥.처리('k'); // ㅣ
    assert_eq!(문맥.편집문자렬(), "쌔");
    
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "싸");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅆ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
    
    // Case 5: ggo (ㅅ + ㅅ + ㅐ) - with 두번타건조합
    let mut 문맥 = 문맥생성("kps9256");
    문맥.항목설정(korean::input_context::입력항목::두번타건조합, true);
    문맥.처리('g'); // ㅅ
    문맥.처리('g'); // ㅅ -> ㅆ
    문맥.처리('o'); // ㅐ
    assert_eq!(문맥.편집문자렬(), "쌔");
    
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅆ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "ㅅ");
    문맥.지우기();
    assert_eq!(문맥.편집문자렬(), "");
}
