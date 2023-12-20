use crate::{ast::parser::Parser, scanner::scanner::Scanner};

pub fn run(input: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse()?;

    println!("{:?}", expr.to_string());

    Ok(())
}
