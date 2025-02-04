# TerminalSQL

## Overview
Run SQL query on your local files directly in terminal.

This project provides a simple SQL query execution framework using the **Polars** library in Rust. It includes modules for file handling, query execution, and utility functions to facilitate working with data.

## Features
- Read and process data files
- Execute SQL queries using the Polars SQL engine
- Utility functions for handling input
- Unit and integration tests

## Installation
Ensure you have **Rust** installed. If not, install it using [Rustup](https://rustup.rs/):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository:
```sh
git clone https://github.com/romanmurzac/terminal_sql.git
cd terminal_sql
```

Build the project:
```sh
cargo build
```

## Usage
Run the project:
```sh
cargo run
```

Run the tests:
```sh
cargo test
```

## Project Structure
```
project/
├── data/
│   ├── data.csv        # Test CSV file
│   ├── data.json       # Test JSON file
│   ├── data.parquet    # Test Parquet file
├── src/
│   ├── file_io.rs      # File handling logic
│   ├── query.rs        # Query execution logic
│   ├── utils.rs        # Utility functions
│   ├── main.rs         # Entry point
│   ├── lib.rs          # Module declarations
├── tests/
│   ├── integration_tests.rs  # Integration tests
├── Cargo.toml          # Rust package configuration
└── README.md           # Project documentation
```

## Modules

### `file_io`
Handles reading data files.
```rust
/// Reads a file and returns its contents as a string.
pub fn read_file(path: &str) -> Result<String, String> {
}
```

### `query`
Executes SQL queries using Polars.
```rust
/// Executes a SQL query on a given Polars SQLContext.
pub fn execute_query(ctx: &mut SQLContext, query: &str) -> Result<(), String> {
}
```

### `utils`
Utility functions for user input and error handling.
```rust
/// Prompts the user for input and returns the response.
pub fn user_input(input: &str) -> Result<String, String> {
}
```

## Contributing
Feel free to fork this repository and submit a pull request with improvements!

## License
This project is licensed under the MIT License.

