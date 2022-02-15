use crate::{impl_inner, inner::transfer::*, ruby_class, util::ConvertOrPanic};

use rutie::{methods, types::Value, AnyObject, Class, Float, Module, NilClass, Object, Symbol, GC};

pub fn define_class(super_class: &Class) {
    Class::new("Transfer", Some(super_class)).define(|class| {
        class.def("load", transfer__load);
        class.def("save", transfer__save);
        class.def("offset", transfer__offset);
        class.def("out", transfer__out);
    });

    //parent
    //    .define_nested_class("Transfer", Some(super_class))
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
