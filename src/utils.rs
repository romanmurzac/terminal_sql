use std::io::{self, Write};

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
