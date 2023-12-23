use super::{interpreter::Interpreter, parser::Parser, scanner::Scanner};

pub fn run(input: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(&tokens);
    let statements = parser.parse()?;
    let mut interpreter = Interpreter::new();

    interpreter.interpret_stmts(statements)?;

    Ok(())
}
