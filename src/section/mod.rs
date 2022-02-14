use std::collections::HashMap;

use crate::{
    time::{Beat, Time},
    util::ConvertOrPanic,
};
use itertools::Itertools;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Section", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def("symbol", section__symbol);
            klass.def("sheet", section__sheet);
            klass.def("division", section__division);
        });
    });

    parent
        .define_nested_class("Section", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def("symbol", section__symbol);
                klass.def("sheet", section__sheet);
                klass.def("division", section__division);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Section {
    value: Value,
}

impl Section {
    pub fn new(symbols: Option<HashMap<String, Hash>>) -> AnyObject {
        let inner = SectionInner::new(symbols);

        Class::from_existing("Section").wrap_data(inner, &*SECTION_WRAPPER)
    }

    pub fn symbol(mut itself: Section, key: Symbol, value: Hash) -> NilClass {
        let section = itself.get_data_mut(&*SECTION_WRAPPER);

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

        section.division = Some(Beat {
            numerator: numerator.to_u32(),
            denominator: denominator.to_u32(),
        });

        NilClass::new()
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

impl From<Value> for Section {
    fn from(value: Value) -> Self {
        Section { value }
    }
}

impl TryFrom<AnyObject> for Section {
    type Error = std::io::Error;

    fn try_from(obj: AnyObject) -> Result<Section, Self::Error> {
        if Class::from_existing("Section").case_equals(&obj) {
            Ok(Section::from(obj.value()))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "aaaaaaaaaokkkkkkk",
            ))
        }
    }
}

impl Object for Section {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Section {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        Class::from_existing("Section").case_equals(object)
    }

    fn error_message() -> &'static str {
        "Error converting to Section"
    }
}

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
);

use crate::instrument::InstrumentInner;

#[derive(Debug, Clone)]
pub struct SectionInner {
    pub symbols: HashMap<String, Hash>,
    pub sheet: Option<Vec<Hash>>,
    pub division: Option<Beat>,
}

impl SectionInner {
    pub fn new(symbols: Option<HashMap<String, Hash>>) -> Self {
        Self {
            symbols: symbols.unwrap_or(HashMap::new()),
            sheet: None,
            division: None,
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
