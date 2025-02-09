use std::io::{self, Write};
use polars::error::PolarsResult;
use polars::frame::DataFrame;

use crate::file_io::read_file;

/// Prompts the user for input.
pub fn user_input(message: &str) -> io::Result<String> {
    print!("\n\x1b[1;36m{}\x1b[0m\n", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();

    if input.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "\x1b[1;31mInput cannot be empty.\x1b[0m"));
    }
    Ok(input)
}

/// Check if all files are parsed correctly.
pub fn get_files() -> PolarsResult<Vec<(String, DataFrame)>> {
    let mut valid_files = Vec::new();
    let mut input_valid = false;

    while !input_valid {
        match user_input("Enter file paths (comma-separated) or 'exit':") {
            Ok(query) if query.eq_ignore_ascii_case("exit") => {
                println!("\n\x1b[1;42mExiting...\x1b[0m\n");
                return Ok(vec![]);
            }
            Ok(file_paths) => {
                let mut temp_files = Vec::new();
                let mut all_valid = true;
                
                for file_path in file_paths.split(',').map(str::trim) {
                    match read_file(file_path) {
                        Ok(df) => {
                            let table_name = format!("source_{}", temp_files.len() + 1);
                            temp_files.push((table_name, df));
                        }
                        Err(e) => {
                            eprintln!(
                                "\x1b[1;31mError reading file '{}': {}\x1b[0m",
                                file_path, e
                            );
                            all_valid = false;
                        }
                    }
                }

                if all_valid {
                    valid_files = temp_files;
                    input_valid = true;
                } else {
                    println!("\x1b[1;31mOne or more files are invalid. Please enter valid file paths.\x1b[0m");
                }
            }
            Err(e) => eprintln!("\x1b[1;31mError getting file paths: {}\x1b[0m", e),
        }
    }
    Ok(valid_files)
}