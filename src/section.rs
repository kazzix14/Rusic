use crate::{impl_inner, inner::section::*, ruby_class};

use std::collections::HashMap;

use num::Rational32;
use rutie::{
    methods, types::Value, AnyObject, Class, Hash, Integer, Module, NilClass, Object, RString,
    Symbol, GC,
};

pub fn define_class(super_class: &Class) {
    Class::new("Section", Some(super_class)).define(|class| {
        class.def("symbol", section__symbol);
        class.def("sheet", section__sheet);
        class.def("division", section__division);
        class.def("length", section__length);
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

    pub fn symbol(mut itself: Section, key: Symbol, value: Hash) -> NilClass {
        let section = itself.get_data_mut(&*SECTION_WRAPPER);

        GC::register_mark(&value);
        section.symbols.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn sheet(mut itself: Section, sheet: RString) -> NilClass {
        let section = itself.get_data_mut(&*SECTION_WRAPPER);

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

    pub fn division(mut itself: Section, numerator: Integer, denominator: Integer) -> NilClass {
        let section = itself.get_data_mut(&*SECTION_WRAPPER);

        section.division = Some(Rational32::new(numerator.to_i32(), denominator.to_i32()));

        NilClass::new()
    }

    pub fn length(mut itself: Section, numerator: Integer, denominator: Integer) -> NilClass {
        let section = itself.get_data_mut(&*SECTION_WRAPPER);

        section.length = Some(Rational32::new(numerator.to_i32(), denominator.to_i32()));

        NilClass::new()
    }
}

ruby_class!(Section);
impl_inner!(Section, SectionInner, SECTION_WRAPPER);
methods!(
    Section,
    itself,
    fn section__symbol(key: Symbol, value: Hash) -> NilClass {
        Section::symbol(itself, key.unwrap(), value.unwrap())
    },
    fn section__sheet(sheet: RString) -> NilClass {
        Section::sheet(itself, sheet.unwrap())
    },
    fn section__division(numerator: Integer, denominator: Integer) -> NilClass {
        Section::division(itself, numerator.unwrap(), denominator.unwrap())
    },
    fn section__length(numerator: Integer, denominator: Integer) -> NilClass {
        Section::length(itself, numerator.unwrap(), denominator.unwrap())
    },
);
