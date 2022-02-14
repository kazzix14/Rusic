pub mod transfer;

use std::collections::HashMap;

use crate::{ruby_class, section::Section, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, GC, VM,
};

use self::transfer::Transfer;

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Instrument", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def("init", instrument__init);
            klass.def("before_each_note", instrument__before_each_note);
            klass.def("signal", instrument__signal);
        });
    });

    parent
        .define_nested_class("Instrument", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def("init", instrument__init);
                klass.def("before_each_note", instrument__before_each_note);
                klass.def("signal", instrument__signal);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Instrument {
    value: Value,
}

ruby_class!(Instrument);
methods!(
    Instrument,
    itself,
    fn instrument__init() -> NilClass {
        Instrument::init(itself)
    },
    fn instrument__before_each_note() -> NilClass {
        Instrument::before_each_note(itself)
    },
    fn instrument__signal() -> NilClass {
        Instrument::signal(itself)
    },
);

impl Instrument {
    pub fn new() -> AnyObject {
        let inner = InstrumentInner::new();

        Class::from_existing("Instrument").wrap_data(inner, &*INSTRUMENT_WRAPPER)
    }

    pub fn init(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        let init_fn = VM::block_proc();
        GC::register_mark(&init_fn);
        instrument.init_fn = Some(init_fn);

        NilClass::new()
    }

    pub fn before_each_note(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        let before_each_note_fn = VM::block_proc();
        GC::register_mark(&before_each_note_fn);
        instrument.before_each_note_fn = Some(before_each_note_fn);

        NilClass::new()
    }

    pub fn signal(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        let signal_fn = VM::block_proc();
        GC::register_mark(&signal_fn);
        instrument.signal_fn = Some(signal_fn);

        NilClass::new()
    }

    pub fn exec_init(&mut self) {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let transfer = instrument.transfer;
        let arg = [transfer.to_any_object()];

        if let Some(f) = &instrument.init_fn {
            f.call(&arg);
        }
    }

    pub fn exec_before_each_note(&mut self, note: &Hash) -> f32 {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let mut transfer = instrument.transfer;
        transfer.reset();

        let arg = [transfer.to_any_object(), note.to_any_object()];

        if let Some(f) = &instrument.before_each_note_fn {
            f.call(&arg);
        }

        transfer.inner().offset
    }

    pub fn exec_signal(&mut self, note: &Hash, length: f32, time: f32) -> Option<f32> {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let mut transfer = instrument.transfer;
        transfer.reset();

        let arg = [
            transfer.to_any_object(),
            note.to_any_object(),
            Float::new(length as f64).to_any_object(),
            Float::new(time as f64).to_any_object(),
        ];

        if let Some(f) = &instrument.signal_fn {
            f.call(&arg);
        }

        transfer.inner().out
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);
#[derive(Debug)]
pub struct InstrumentInner {
    pub init_fn: Option<Proc>,
    pub before_each_note_fn: Option<Proc>,
    pub signal_fn: Option<Proc>,
    pub transfer: Transfer,
}

impl InstrumentInner {
    pub fn new() -> Self {
        Self {
            init_fn: None,
            before_each_note_fn: None,
            signal_fn: None,
            transfer: Transfer::new().convert_or_panic(),
        }
    }
}
