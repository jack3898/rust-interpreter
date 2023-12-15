use crate::scanner::scanner::Scanner;

pub fn run(input: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;

    println!("{:?}", tokens);

    Ok(())
}
