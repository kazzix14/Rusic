use crate::{impl_inner, inner::instrument::*, ruby_class};
use rutie::{methods, types::Value, AnyObject, Class, Float, Hash, NilClass, Object, GC, VM};

pub fn define_class(super_class: &Class) {
    Class::new("Instrument", Some(super_class)).define(|class| {
        class.def("init", instrument_init);
        class.def("before_each_note", instrument_before_each_note);
        class.def("signal", instrument_signal);
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

    pub fn init(&mut self) -> NilClass {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);

        let init_fn = VM::block_proc();
        instrument.init_fn = Some(init_fn);

        NilClass::new()
    }

    pub fn before_each_note(&mut self) -> NilClass {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);

        let before_each_note_fn = VM::block_proc();
        instrument.before_each_note_fn = Some(before_each_note_fn);

        NilClass::new()
    }

    pub fn signal(&mut self) -> NilClass {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);

        let signal_fn = VM::block_proc();
        instrument.signal_fn = Some(signal_fn);

        NilClass::new()
    }

    pub fn exec_init(&mut self) {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let instrument_dto = instrument.instrument_dto;
        let arg = [instrument_dto.to_any_object()];

        if let Some(f) = &instrument.init_fn {
            f.call(&arg);
        }
    }

    pub fn exec_before_each_note(&mut self, note: &Hash) -> f32 {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let mut instrument_dto = instrument.instrument_dto;
        instrument_dto.reset();

        let arg = [instrument_dto.to_any_object(), note.to_any_object()];

        if let Some(f) = &instrument.before_each_note_fn {
            f.call(&arg);
        }

        instrument_dto.inner().offset
    }

    pub fn exec_signal(
        &mut self,
        note: &Hash,
        length: f32,
        time: f32,
        delta: f64,
    ) -> Option<(f32, f32)> {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let mut instrument_dto = instrument.instrument_dto;
        instrument_dto.reset();

        let arg = [
            instrument_dto.to_any_object(),
            note.to_any_object(),
            Float::new(length as f64).to_any_object(),
            Float::new(time as f64).to_any_object(),
            Float::new(delta).to_any_object(),
        ];

        if let Some(f) = &instrument.signal_fn {
            f.call(&arg);
        }

        if let (Some(left), Some(right)) =
            (instrument_dto.inner().left, instrument_dto.inner().right)
        {
            Some((left, right))
        } else {
            None
        }
    }
}

ruby_class!(Instrument);
impl_inner!(Instrument, InstrumentInner, INSTRUMENT_WRAPPER);
methods!(
    Instrument,
    itself,
    fn instrument_init() -> NilClass {
        itself.init()
    },
    fn instrument_before_each_note() -> NilClass {
        itself.before_each_note()
    },
    fn instrument_signal() -> NilClass {
        itself.signal()
    },
);
