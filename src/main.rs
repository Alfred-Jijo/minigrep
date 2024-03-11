use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead}; // Import Error trait for error conversion

fn minigrep(folder_path: &str, search_term: &str) {
    // Compile the regex pattern
    let re = match Regex::new(search_term) {
        Ok(re) => re,
        Err(err) => {
            eprintln!("Error: Invalid regex pattern: {}", err);
            return;
        }
    };

    // Loop through all files in the folder
    for entry in std::fs::read_dir(folder_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();

        if let Ok(file) = File::open(&file_path) {
            let reader = io::BufReader::new(file);

            // Read each line in the file
            for (line_number, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                if re.is_match(&line) {
                    println!(
                        "Found in {:?} at line {}: {}",
                        file_path,
                        line_number + 1,
                        line
                    );
                }
            }
        } else {
            eprintln!("Error: Could not read file {:?}", file_path);
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: minigrep <folder_path> <search_term>");
        std::process::exit(1);
    }

    let folder_path: &String = &args[1];
    let search_term: &String = &args[2];

    minigrep(folder_path, search_term);
}
