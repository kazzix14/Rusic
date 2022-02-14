use std::collections::HashMap;

use crate::{section::Section, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, VM,
};

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

    pub fn before_each_note(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        instrument.before_each_note_fn = Some(VM::block_proc());

        NilClass::new()
    }

    pub fn signal(mut itself: Instrument) -> NilClass {
        let instrument = itself.get_data_mut(&*INSTRUMENT_WRAPPER);

        instrument.signal_fn = Some(VM::block_proc());

        NilClass::new()
    }

    pub fn exec_init(&mut self) {
        let store = self.take_store();
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let arg = [store];

        if let Some(f) = &instrument.init_fn {
            f.call(&arg);
        }

        let [store] = arg;
        self.put_store(store);
    }

    pub fn exec_before_each_note(&mut self, note: Hash) -> f32 {
        let store = self.take_store();
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let offset = Float::new(0.0);
        let arg = [note.to_any_object(), offset.to_any_object(), store];

        dbg!(&arg);

        if let Some(f) = &instrument.before_each_note_fn {
            f.call(&arg);
        }

        dbg!(&arg);

        let [_, offset, store] = arg;
        self.put_store(store);

        let offset: Float = offset.convert_or_panic();
        offset.to_f64() as f32
    }

    pub fn exec_signal(&mut self, note: Hash, time: f32) -> f32 {
        let store = self.take_store();
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let out = Float::new(0.0);
        let time = Float::new(time as f64);
        let arg = [
            note.to_any_object(),
            time.to_any_object(),
            out.to_any_object(),
            store,
        ];

        dbg!(&arg);

        if let Some(f) = &instrument.signal_fn {
            f.call(&arg);
        }

        dbg!(&arg);

        let [_, _, out, store] = arg;
        self.put_store(store);

        let out: Float = out.convert_or_panic();
        out.to_f64() as f32
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }

    fn take_store(&mut self) -> AnyObject {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let store_ptr = Box::into_raw(instrument.store.take().unwrap());
        AnyObject::from(Value::from(store_ptr as usize))
    }

    fn put_store(&mut self, object: AnyObject) {
        let instrument = self.get_data_mut(&*INSTRUMENT_WRAPPER);
        let value: Value = object.into();
        let store_ptr = value.value;
        let store_ptr = store_ptr as *mut ();
        instrument.store = Some(unsafe { Box::from_raw(store_ptr) });
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

#[derive(Debug)]
pub struct InstrumentInner {
    pub init_fn: Option<Proc>,
    pub before_each_note_fn: Option<Proc>,
    pub signal_fn: Option<Proc>,
    pub store: Option<Box<()>>,
}

impl InstrumentInner {
    pub fn new() -> Self {
        Self {
            init_fn: None,
            before_each_note_fn: None,
            signal_fn: None,
            store: Some(Box::new(())),
        }
    }
}

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);
