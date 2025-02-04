use terminal_sql::file_io::read_file;
use terminal_sql::query::execute_query;
use terminal_sql::utils::user_input;
use polars::sql::SQLContext;

#[test]
fn test_prompt_empty() {
    let result = user_input("").err();
    assert!(result.is_some());
}

#[test]
fn test_read_file_unsupported() {
    let result = read_file("test.txt").err();
    assert!(result.is_some());
}

#[test]
fn test_execute_query_empty() {
    let mut ctx = SQLContext::new();
    let result = execute_query(&mut ctx, "SELECT * FROM non_existent");
    assert!(result.is_err());
}
