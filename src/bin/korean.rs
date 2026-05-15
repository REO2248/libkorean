//! CLI tool for Korean input conversion.
//!
//! Usage:
//! ```bash
//! echo "djffydtjgkqfkSJ" | cargo run --bin korean
//! ```

use korean::input_context::입력문맥;
use korean::keyboard::건반등록기;
use std::io::{self, Read};
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut keyboard = "kps9256".to_string();
    let mut list_keyboards = false;
    let mut input_string: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-k" | "--keyboard" => {
                i += 1;
                if i < args.len() {
                    keyboard = args[i].clone();
                }
            }
            "-l" | "--list" => {
                list_keyboards = true;
            }
            "-i" | "--input" => {
                i += 1;
                if i < args.len() {
                    input_string = Some(args[i].clone());
                }
            }
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-v" | "--version" => {
                println!("korean {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            other => {
                eprintln!("Unknown option: {other}");
                process::exit(1);
            }
        }
        i += 1;
    }

    if list_keyboards {
        list_keyboard_layouts();
        process::exit(0);
    }

    let mut ic = match 입력문맥::new(&keyboard) {
        Ok(ic) => ic,
        Err(e) => {
            eprintln!("Error: {e}");
            process::exit(1);
        }
    };

    if let Some(input) = input_string {
        process_string(&mut ic, &input);
    } else {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap_or_default();
        process_string(&mut ic, &input);
    }
}

fn process_string(ic: &mut 입력문맥, input: &str) {
    let mut output = String::new();

    for ch in input.chars() {
        if ic.처리(ch) {
            let commit = ic.결속문자렬();
            if !commit.is_empty() {
                output.push_str(commit);
            }
        } else {
            output.push_str(&ic.비우기());
            output.push(ch);
        }
    }

    let remaining = ic.비우기();
    if !remaining.is_empty() {
        output.push_str(&remaining);
    }

    print!("{output}");
}

fn list_keyboard_layouts() {
    for layout in 건반등록기::목록() {
        println!("{:<12} {}", layout.id, layout.name);
    }
}

fn print_usage() {
    println!(
        r#"Usage: korean [OPTION]...

Convert string into Korean characters according to keyboard layout.

Options:
  -k, --keyboard=KEYBOARD   Select keyboard layout (default: "kps9256")
  -l, --list                List available keyboard layouts
  -i, --input=STRING        Use STRING as input instead of stdin
  -h, --help                Display this help
  -v, --version             Output version information

Examples:
  echo "djffydtjgkqfkSJ" | korean
  korean -i "ajryssodgod"
  korean -k romaja -i "banGabseubnida"
"#
    );
}
