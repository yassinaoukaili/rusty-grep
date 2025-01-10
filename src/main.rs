mod rustygrep;

use rustygrep::RustyGrep;
use std::env;
use std::process;

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: rustygrep <pattern> <starting_directory>");
        process::exit(1);
    }

    let pattern = &args[1];
    let starting_dir = &args[2];

    // Create an instance of RustyGrep 
    let rusty_grep = match RustyGrep::new(pattern, starting_dir) {
        Ok(rg) => rg,
        Err(e) => {
            eprintln!("Error: Invalid regex pattern: {}", e);
            process::exit(1);
        }
    };

    // Perform the search
    if let Err(e) = rusty_grep.search() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
