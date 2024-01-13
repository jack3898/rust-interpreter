use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct DbgDisplay(Box<dyn Display>);

impl DbgDisplay {
    pub fn from<T: Display + Clone + 'static>(item: &T) -> Self {
        Self(Box::new(item.clone()))
    }
}

impl Debug for DbgDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Display for DbgDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
