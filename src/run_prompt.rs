use std::io::{self, Write};

use crate::constants::{CRLF, LF};

pub fn run_prompt() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("> ");

        // In Rust the stdout is buffered for data that does not end with a new line character.
        // This will cause the buffer to flush to the terminal rendering the print!
        io::stdout().flush().expect("Flush error.");

        stdin
            .read_line(&mut buffer)
            .expect("Invalid line passed in to prompt!");

        if buffer.ends_with(&LF.repeat(3)) || buffer.ends_with(&CRLF.repeat(3)) {
            break;
        }
    }

    buffer
}
