use crate::{
    ast::parser::Parser, interpreter::interpreter::Interpreter, scanner::scanner::Scanner,
};

pub fn run(input: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse()?;
    let interpreter = Interpreter::new(&expr);

    println!("{:?}", interpreter.evaluate(None).unwrap().to_string());

    Ok(())
}
