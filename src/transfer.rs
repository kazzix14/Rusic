use std::collections::HashMap;

use crate::{impl_inner, ruby_class, section::Section, track::TrackInner, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, GC, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Transfer", Some(data_class)).define(|class| {
        class.def("load", transfer__load);
        class.def("save", transfer__save);
        class.def("offset", transfer__offset);
        class.def("out", transfer__out);
    });

    //parent
    //    .define_nested_class("Transfer", Some(data_class))
    //    .define(|class| {
    //        class.def("load", transfer__load);
    //        class.def("save", transfer__save);
    //        class.def("offset", transfer__offset);
    //        class.def("out", transfer__out);
    //    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Transfer {
    value: Value,
}

ruby_class!(Transfer);
impl_inner!(Transfer, TransferInner, TRANSFER_WRAPPER);
methods!(
    Transfer,
    itself,
    fn transfer__load(key: Symbol) -> AnyObject {
        Transfer::load(itself, key.unwrap())
    },
    fn transfer__save(key: Symbol, value: AnyObject) -> NilClass {
        Transfer::save(itself, key.unwrap(), value.unwrap())
    },
    fn transfer__offset(offset: Float) -> NilClass {
        Transfer::offset(itself, offset.unwrap())
    },
    fn transfer__out(signal: AnyObject) -> NilClass {
        Transfer::out(itself, signal.unwrap())
    },
);

impl Transfer {
    pub fn new() -> AnyObject {
        let inner = TransferInner::new();

        Class::from_existing("Transfer").wrap_data(inner, &*TRANSFER_WRAPPER)
    }

    pub fn load(mut itself: Transfer, key: Symbol) -> AnyObject {
        let transfer = itself.get_data_mut(&*TRANSFER_WRAPPER);

        transfer
            .store
            .get(key.to_str())
            .cloned()
            .expect("key: `{key}` does not exist")
    }

    pub fn save(mut itself: Transfer, key: Symbol, value: AnyObject) -> NilClass {
        let transfer = itself.get_data_mut(&*TRANSFER_WRAPPER);

        GC::register_mark(&value);
        transfer.store.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn offset(mut itself: Transfer, offset: Float) -> NilClass {
        let transfer = itself.get_data_mut(&*TRANSFER_WRAPPER);

        transfer.offset = offset.to_f64() as f32;

        NilClass::new()
    }

    pub fn out(mut itself: Transfer, signal: AnyObject) -> NilClass {
        let transfer = itself.get_data_mut(&*TRANSFER_WRAPPER);

        let signal = match signal {
            _ if Class::from_existing("NilClass").case_equals(&signal) => None,
            _ if Class::from_existing("Float").case_equals(&signal) => {
                let signal: Float = signal.convert_or_panic();
                Some(signal.to_f64() as f32)
            }
            _ => panic!("signal is invalid"),
        };

        transfer.out = signal;

        NilClass::new()
    }

    pub fn reset(&mut self) {
        self.get_data_mut(&*TRANSFER_WRAPPER).out = None;
    }
}

#[derive(Debug, Clone)]
pub struct TransferInner {
    pub offset: f32,
    pub out: Option<f32>,
    pub store: HashMap<String, AnyObject>,
}

impl TransferInner {
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            out: None,
            store: HashMap::new(),
        }
    }
}

wrappable_struct!(TransferInner, TransferWrapper, TRANSFER_WRAPPER);
