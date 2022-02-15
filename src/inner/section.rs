use std::collections::HashMap;

use num::Rational32;
use rutie::{wrappable_struct, Hash};

#[derive(Debug, Clone)]
pub struct SectionInner {
    pub symbols: HashMap<String, Hash>,
    pub sheet: Option<Vec<Hash>>,
    pub division: Option<Rational32>,
    pub length: Option<Rational32>,
}

impl SectionInner {
    pub fn new(symbols: HashMap<String, Hash>) -> Self {
        Self {
            symbols: symbols,
            sheet: None,
            division: None,
            length: None,
        }
    }

    // TODO Symbolに移す
    pub fn get_symbol(&self, key: String) -> Option<Hash> {
        let value = self.symbols.get(&key);

        match value {
            Some(c) => Some(c.clone()),
            None => {
                println!("warning: Symbol `{key}` is not defined");
                None
            }
        }
    }
}

wrappable_struct!(SectionInner, SectionWrapper, SECTION_WRAPPER);
