use korean::input_context::입력문맥;

fn main() {
    let mut ic = 입력문맥::new("romaja").unwrap();
    let input = "banGabseubnida";
    
    let mut output = String::new();
    for ch in input.chars() {
        ic.처리(ch);
        let commit = ic.결속문자렬();
        if !commit.is_empty() {
            output.push_str(commit);
            ic.결속문자렬_비우기();
        }
        println!("Key: '{}', Output: '{}', Preedit: '{}'", ch, output, ic.편집문자렬());
    }
}
