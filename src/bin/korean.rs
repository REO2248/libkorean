//! CLI tool for Korean input conversion.
//!
//! Usage:
//! ```bash
//! echo "gksrmfdlqfur" | cargo run --bin korean
//! ```

use korean::input_context::InputContext;
use korean::keyboard::KeyboardRegistry;
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

    let mut ic = match InputContext::new(&keyboard) {
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

fn process_string(ic: &mut InputContext, input: &str) {
    let mut output = String::new();

    for ch in input.chars() {
        if ic.process(ch) {
            let commit = ic.get_commit_string();
            if !commit.is_empty() {
                output.push_str(commit);
            }
        } else {
            output.push_str(&ic.flush());
            output.push(ch);
        }
    }

    let remaining = ic.flush();
    if !remaining.is_empty() {
        output.push_str(&remaining);
    }

    print!("{output}");
}

fn list_keyboard_layouts() {
    for layout in KeyboardRegistry::list() {
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
  echo "gksrmfdlqfur" | korean
  korean -i "gksrmfdlqfur"
  korean -k ro -i "hanugl""#
    );
}
