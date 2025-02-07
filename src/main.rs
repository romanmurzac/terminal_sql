use polars::prelude::*;
use polars::sql::SQLContext;

use terminal_sql::file_io::read_file;
use terminal_sql::query::query_loop;
use terminal_sql::utils::user_input;

/// Entry point of the program.
fn main() -> PolarsResult<()> {
    println!("\n\x1b[1;42mEntering...\x1b[0m");
    match user_input("Enter file paths (comma-separated):") {
        Ok(file_paths) => {
            let mut ctx = SQLContext::new();

            // Register each file as a table in SQLContext.
            for (i, file_path) in file_paths.split(',').map(str::trim).enumerate() {
                match read_file(file_path) {
                    Ok(df) => {
                        let table_name = format!("source_{}", i + 1);
                        ctx.register(&table_name, df.lazy());
                    }
                    Err(e) => eprintln!("\x1b[1;31mError reading file '{}': {}\x1b[0m", file_path, e),
                }
            }
            // Start the SQL query loop.
            if let Err(e) = query_loop(&mut ctx) {
                eprintln!("\x1b[1;31mError in query loop: {}\x1b[0m", e);
            }
        }
        Err(e) => eprintln!("\x1b[1;31mError getting file paths: {}\x1b[0m", e),
    }
    Ok(())
}
