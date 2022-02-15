use crate::{impl_inner, inner::instrument::*, ruby_class};
use rutie::{
    methods, types::Value, AnyObject, Class, Float, Hash, Module, NilClass, Object, GC, VM,
};

use crate::transfer::*;

pub fn define_class(super_class: &Class) {
    Class::new("Instrument", Some(super_class)).define(|class| {
        class.def("init", instrument_init);
        class.def("before_each_note", instrument_before_each_note);
        class.def("signal", instrument_signal);
    });

    //parent
    //    .define_nested_class("Instrument", Some(super_class))
    //    .define(|class| {
    //        class.def("init", instrument_init);
    //        class.def("before_each_note", instrument_before_each_note);
    //        class.def("signal", instrument_signal);
    //    });
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
}

ruby_class!(Instrument);
impl_inner!(Instrument, InstrumentInner, INSTRUMENT_WRAPPER);
methods!(
    Instrument,
    itself,
    fn instrument_init() -> NilClass {
        Instrument::init(itself)
    },
    fn instrument_before_each_note() -> NilClass {
        Instrument::before_each_note(itself)
    },
    fn instrument_signal() -> NilClass {
        Instrument::signal(itself)
    },
);
