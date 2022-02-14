use std::collections::HashMap;

use crate::{section::Section, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Instrument", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def("init", instrument__init);
            klass.def("signal", instrument__signal);
        });
    });

    parent
        .define_nested_class("Instrument", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def("init", instrument__init);
                klass.def("signal", instrument__signal);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Instrument {
    value: Value,
}

impl Instrument {
    pub fn new() -> AnyObject {
        let inner = InstrumentInner::new();

        Class::from_existing("Instrument").wrap_data(inner, &*INSTRUMENT_WRAPPER)
    }

    pub fn init(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        instrument.init_fn = Some(VM::block_proc());

        NilClass::new()
    }

    pub fn signal(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        instrument.signal_fn = Some(VM::block_proc());

        NilClass::new()
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

impl From<Value> for Instrument {
    fn from(value: Value) -> Self {
        Instrument { value }
    }
}

impl TryFrom<AnyObject> for Instrument {
    type Error = std::io::Error;

    fn try_from(obj: AnyObject) -> Result<Instrument, Self::Error> {
        if Class::from_existing("Instrument").case_equals(&obj) {
            Ok(Instrument::from(obj.value()))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "aaaaaaaaaokkkkkkk",
            ))
        }
    }
}

impl Object for Instrument {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Instrument {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        Class::from_existing("Instrument").case_equals(object)
    }

    fn error_message() -> &'static str {
        "Error converting to Instrument"
    }
}

methods!(
    Instrument,
    itself,
    fn instrument__init(name: RString) -> NilClass {
        Instrument::init(itself)
    },
    fn instrument__signal(name: RString) -> NilClass {
        Instrument::signal(itself)
    },
);

#[derive(Debug)]
pub struct InstrumentInner {
    pub init_fn: Option<Proc>,
    pub signal_fn: Option<Proc>,
}

impl InstrumentInner {
    pub fn new() -> Self {
        Self {
            init_fn: None,
            signal_fn: None,
        }
    }
}

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);
