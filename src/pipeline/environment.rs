use std::collections::HashMap;

use crate::types::literal_type::Lit;

pub struct Environment {
    values: HashMap<String, Lit>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, value: Lit) {
        self.values.insert(name.into(), value);
    }

    pub fn get(&mut self, name: &str) -> Option<&Lit> {
        self.values.get(name)
    }
}
