//! CLI tool for Hanja dictionary search.
//!
//! Usage:
//! ```bash
//! echo "삼국사기" | cargo run --bin hanja-search
//! ```

use korean::hanja::HanjaDict;
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
            "-k" | "--key" => {
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
                println!("hanja-search {}", env!("CARGO_PKG_VERSION"));
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

    let dict = match HanjaDict::load(&dict_path) {
        Ok(dict) => dict,
        Err(e) => {
            eprintln!("Error loading dictionary: {e}");
            process::exit(1);
        }
    };

    if let Some(key) = search_key {
        search_and_print(&dict, &key);
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let key = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            let key = key.trim().to_string();
            if key.is_empty() {
                continue;
            }
            search_and_print(&dict, &key);
        }
    }
}

fn search_and_print(dict: &HanjaDict, key: &str) {
    let results = dict.match_prefix(key);

    for entry in &results {
        let comment = entry.comment.as_deref().unwrap_or("");
        println!("{}:{}:{}", entry.key, entry.value, comment);
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
  echo "삼국사기" | hanja-search
  hanja-search -k "한자"
  hanja-search /path/to/hanja.txt"#
    );
}
