use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead}; // Import Error trait for error conversion

fn minigrep(folder_path: &str, search_term: &str) -> io::Result<()> {
    let re = Regex::new(search_term);

    // Loop through all files in the folder
    for entry in std::fs::read_dir(folder_path)? {
        let entry: std::fs::DirEntry = entry?;
        let file_path: std::path::PathBuf = entry.path();

        if let Ok(file) = File::open(&file_path) {
            let reader = io::BufReader::new(file);

            // Read each line in the file
            for (line_number, line) in reader.lines().enumerate() {
                let line: String = line?;
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

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: minigrep <folder_path> <search_term>");
        std::process::exit(1);
    }

    let folder_path = &args[1];
    let search_term = &args[2];

    match minigrep(folder_path, search_term) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}
