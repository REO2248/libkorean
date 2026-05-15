//! CLI tool for Hanja dictionary search.
//!
//! Usage:
//! ```bash
//! echo "인민" | cargo run --bin hanja-search
//! ```

use korean::hanja::한자사전;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut dict_path: Option<PathBuf> = None;
    let mut search_key: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-f" | "--file" => {
                i += 1;
                if i < args.len() {
                    dict_path = Some(PathBuf::from(&args[i]));
                }
            }
            "-k" | "--열쇠" => {
                i += 1;
                if i < args.len() {
                    search_key = Some(args[i].clone());
                }
            }
            "-h" | "--help" => {
                print_usage();
                process::exit(0);
            }
            "-v" | "--version" => {
                println!("한자검색 {}", env!("CARGO_PKG_VERSION"));
                process::exit(0);
            }
            other => {
                // Assume it's a file path if it looks like one
                if other.ends_with(".txt") || PathBuf::from(other).exists() {
                    dict_path = Some(PathBuf::from(other));
                } else {
                    eprintln!("Unknown option: {other}");
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    let dict_path = dict_path.unwrap_or_else(|| PathBuf::from("data/hanja/hanja.txt"));

    if !dict_path.exists() {
        eprintln!("Error: Dictionary file not found: {}", dict_path.display());
        process::exit(1);
    }

    let 사전 = match 한자사전::적재(&dict_path) {
        Ok(사전) => 사전,
        Err(e) => {
            eprintln!("Error loading dictionary: {e}");
            process::exit(1);
        }
    };

    if let Some(열쇠) = search_key {
        search_and_print(&사전, &열쇠);
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let 열쇠 = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            let 열쇠 = 열쇠.trim().to_string();
            if 열쇠.is_empty() {
                continue;
            }
            search_and_print(&사전, &열쇠);
        }
    }
}

fn search_and_print(사전: &한자사전, 열쇠: &str) {
    let 결과 = 사전.앞부분일치(열쇠);

    for 항목 in &결과 {
        let 설명 = 항목.설명.as_deref().unwrap_or("");
        println!("{}:{}:{}", 항목.열쇠, 항목.값, 설명);
    }
}

fn print_usage() {
    println!(
        r#"Usage: hanja-search [OPTION]... [DICT_FILE]

Search hanja dictionary.

Options:
  -f, --file=FILE       Dictionary file (default: data/hanja/hanja.txt)
  -k, --key=KEY         Search key (if not provided, reads from stdin)
  -h, --help            Display this help
  -v, --version         Output version information

Examples:
  echo "인민" | hanja-search
  hanja-search -k "한자"
  hanja-search /path/to/hanja.txt"#
    );
}
