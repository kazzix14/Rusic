use crate::{impl_inner, inner::section::*, ruby_class};

use std::collections::HashMap;

use num::Rational32;
use rutie::{
    methods, types::Value, AnyObject, Class, Hash, Integer, NilClass, Object, RString, Symbol, GC,
};

pub fn define_class(super_class: &Class) {
    Class::new("Section", Some(super_class)).define(|class| {
        class.def("symbol", section_symbol);
        class.def("sheet", section_sheet);
        class.def("division", section_division);
        class.def("length", section_length);
    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Section {
    value: Value,
}

impl Section {
    pub fn new(symbols: HashMap<String, Hash>) -> AnyObject {
        let inner = SectionInner::new(symbols);

        Class::from_existing("Section").wrap_data(inner, &*SECTION_WRAPPER)
    }

    pub fn symbol(&mut self, key: Symbol, value: Hash) -> NilClass {
        let section = self.get_data_mut(&*SECTION_WRAPPER);

        //GC::mark(&value);
        section.symbols.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn sheet(&mut self, sheet: RString) -> NilClass {
        let section = self.get_data_mut(&*SECTION_WRAPPER);

        let sheet = sheet.to_string();
        let sheet = sheet
            .chars()
            .map(|c| match c {
                _ if c.is_whitespace() => None,
                _ if c.is_alphanumeric() => section.get_symbol(c.to_string()),
                _ => panic!("unexpected token in sheet"),
            })
            .filter_map(|v| v)
            .collect();

        section.sheet = Some(sheet);

        NilClass::new()
    }

    pub fn division(&mut self, numerator: Integer, denominator: Integer) -> NilClass {
        let section = self.get_data_mut(&*SECTION_WRAPPER);

        section.division = Some(Rational32::new(numerator.to_i32(), denominator.to_i32()));

        NilClass::new()
    }

    pub fn length(&mut self, numerator: Integer, denominator: Integer) -> NilClass {
        let section = self.get_data_mut(&*SECTION_WRAPPER);

        section.length = Some(Rational32::new(numerator.to_i32(), denominator.to_i32()));

        NilClass::new()
    }
}

ruby_class!(Section);
impl_inner!(Section, SectionInner, SECTION_WRAPPER);
methods!(
    Section,
    itself,
    fn section_symbol(key: Symbol, value: Hash) -> NilClass {
        itself.symbol(key.unwrap(), value.unwrap())
    },
    fn section_sheet(sheet: RString) -> NilClass {
        itself.sheet(sheet.unwrap())
    },
    fn section_division(numerator: Integer, denominator: Integer) -> NilClass {
        itself.division(numerator.unwrap(), denominator.unwrap())
    },
    fn section_length(numerator: Integer, denominator: Integer) -> NilClass {
        itself.length(numerator.unwrap(), denominator.unwrap())
    },
);
