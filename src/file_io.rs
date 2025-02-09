use std::io;
use std::fs::File;
use polars::prelude::*;

use crate::utils::user_input;

/// Reads a file and loads it as a DataFrame.
pub fn read_file(file_path: &str) -> PolarsResult<DataFrame> {
    let mut file = File::open(file_path)?;
    match file_path {
        path if path.ends_with(".csv") => CsvReader::new(&mut file).finish(),
        path if path.ends_with(".json") => JsonReader::new(&mut file).finish(),
        path if path.ends_with(".parquet") => ParquetReader::new(&mut file).finish(),
        _ => Err(PolarsError::ComputeError("\x1b[1;31mUnsupported file format.\x1b[0m".into())),
    }
}

/// Saves the DataFrame to a file.
pub fn save_file(df: &mut DataFrame, filename: &str) -> PolarsResult<()> {
    let file = File::create(filename)?;
    match filename {
        path if path.ends_with(".csv") => CsvWriter::new(file).include_header(true).finish(df),
        path if path.ends_with(".json") => JsonWriter::new(file).with_json_format(JsonFormat::Json).finish(df),
        path if path.ends_with(".parquet") => ParquetWriter::new(file).finish(df).map(|_| ()),
        _ => Err(PolarsError::ComputeError("\x1b[1;31mUnsupported file format.\x1b[0m".into())),
    }
}

/// Handles the query result and provides an option to save it.
pub fn file_loop(df: &mut DataFrame) -> io::Result<()> {
    println!("\n\x1b[1;32mQuery Result:\x1b[0m {}", df);
    loop {
        match user_input("Do you want to save the result? (yes/no):") {
            Ok(save_choice) if save_choice.eq_ignore_ascii_case("yes") => {
                loop {
                    match user_input("Enter file path:") {
                        Ok(filename) => match save_file(df, &filename) {
                            Ok(_) => {
                                println!("\x1b[1;32mResult saved to {}\x1b[0m", filename);
                                return Ok(());
                            }
                            Err(e) => eprintln!("\x1b[1;31mFailed to save file: {}. Try again.\x1b[0m", e),
                        },
                        Err(e) => eprintln!("\x1b[1;31mError reading filename: {}. Try again.\x1b[0m", e),
                    }
                }
            }
            Ok(_) => return Ok(()),
            Err(e) => eprintln!("\x1b[1;31mError reading save choice: {}. Try again.\x1b[0m", e),
        }
    }
}