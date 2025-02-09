use polars::prelude::*;
use polars::sql::SQLContext;

use terminal_sql::query::query_loop;
use terminal_sql::utils::get_files;

/// Entry point of the program.
fn main() -> PolarsResult<()> {
    println!("\n\x1b[1;42mEntering...\x1b[0m");

    let valid_files = get_files()?;
    if valid_files.is_empty() {
        return Ok(());
    }
    
    let mut ctx = SQLContext::new();

    for (table_name, df) in valid_files {
        ctx.register(&table_name, df.lazy());
    }

    if let Err(e) = query_loop(&mut ctx) {
        eprintln!("\x1b[1;31mError in query loop: {}\x1b[0m", e);
    }

    Ok(())
}