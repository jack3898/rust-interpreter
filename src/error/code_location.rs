use std::fmt::{Display, Formatter};

use super::DbgDisplay;

#[derive(Debug)]
pub struct CodeLocation {
    pub line: usize,
    pub display: DbgDisplay,
}

impl Display for CodeLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "\nAt {}, line {}", self.display, self.line)
    }
}
