use crate::{
    pipeline::{evaluator::Evaluator, parser::Parser, scanner::Scanner},
    types::literal_type::Lit,
};

pub fn run(input: &str) -> Result<Lit, String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;
    let mut parser = Parser::new(&tokens);
    let expr = parser.parse()?;
    let evaluator = Evaluator::new(&expr);

    Ok(evaluator.evaluate(None).unwrap())
}
