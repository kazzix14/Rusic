use std::collections::HashMap;

use crate::{ruby_class, time::Beat, util::ConvertOrPanic};
use itertools::Itertools;
use num::Rational32;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, GC, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Section", Some(data_class)).define(|class| {
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

ruby_class!(Section);
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

    pub fn inner(&self) -> &SectionInner {
        self.get_data(&*SECTION_WRAPPER)
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

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
