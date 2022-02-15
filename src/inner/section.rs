use std::collections::HashMap;

use crate::{ruby_class, time::Beat, util::ConvertOrPanic, impl_inner};
use itertools::Itertools;
use num::Rational32;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, GC, VM,
};

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
