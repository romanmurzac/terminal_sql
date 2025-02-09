use std::io;
use polars::prelude::*;
use polars::sql::SQLContext;

use crate::file_io::file_loop;
use crate::utils::user_input;

/// Loop to process user SQL queries.
pub fn query_loop(ctx: &mut SQLContext) -> io::Result<()> {
    loop {
        match user_input("Enter SQL query or 'exit':") {
            Ok(query) if query.eq_ignore_ascii_case("exit") => {
                println!("\n\x1b[1;42mExiting...\x1b[0m\n");
                break;
            }
            Ok(query) => match execute_query(ctx, &query) {
                Ok(Some(mut df)) => {
                    if let Err(e) = file_loop(&mut df) {
                        eprintln!("\x1b[1;31mError handling query result: {}\x1b[0m", e);
                    }
                }
                Ok(None) => println!("\x1b[1;32mNo result.\x1b[0m"),
                Err(e) => eprintln!("\x1b[1;31mSQL Execution Error: {}\x1b[0m", e),
            },
            Err(e) => eprintln!("\x1b[1;31mError reading query: {}\x1b[0m", e),
        }
    }
    Ok(())
}

/// Executes an SQL query using the SQLContext.
pub fn execute_query(ctx: &mut SQLContext, query: &str) -> PolarsResult<Option<DataFrame>> {
    ctx.execute(query)?.collect().map(Some)
}
