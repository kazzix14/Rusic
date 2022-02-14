use std::collections::HashMap;

use crate::util::ConvertOrPanic;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Section", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def_self("new", section__new);
            klass.def("symbol", section__symbol);
            klass.def("instrument", section__instrument);
        });
    });

    parent
        .define_nested_class("Section", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def_self("new", section__new);
                klass.def("symbol", section__symbol);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Section {
    value: Value,
}

impl Section {
    pub fn new() -> AnyObject {
        let inner = SectionInner::new();

        Class::from_existing("Section").wrap_data(inner, &*SECTION_WRAPPER)
    }

    pub fn symbol(mut itself: Section, key: Symbol, value: Hash) -> NilClass {
        let inner = itself.get_data_mut(&*SECTION_WRAPPER);

        inner.symbols.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn instrument(mut itself: Section, name: Symbol) -> NilClass {
        let inner = itself.get_data_mut(&*SECTION_WRAPPER);

        inner.instrument =
            Some(InstrumentInner::load(name.to_str()).expect("failed to load instrument"));

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
    fn section__new() -> AnyObject {
        Section::new()
    },
    fn section__symbol(key: Symbol, value: Hash) -> NilClass {
        Section::symbol(itself, key.unwrap(), value.unwrap())
    },
    fn section__instrument(name: Symbol) -> NilClass {
        Section::instrument(itself, name.unwrap())
    },
);

use crate::instrument::InstrumentInner;

#[derive(Debug, Clone)]
pub struct SectionInner {
    pub instrument: Option<InstrumentInner>,
    pub symbols: HashMap<String, Hash>,
}

impl SectionInner {
    pub fn new() -> Self {
        Self {
            instrument: None,
            symbols: HashMap::new(),
        }
    }
}

wrappable_struct!(SectionInner, TrackWrapper, SECTION_WRAPPER);
