use std::fs::File;
use polars::prelude::*;

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
