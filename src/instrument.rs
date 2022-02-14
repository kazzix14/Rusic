use crate::util::ConvertOrPanic;

use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(module: &mut Module) {
    module
        .define_nested_class("InstrumentInner", None)
        .define(|klass| {
            klass.def_self("load", instrument__load);
            //klass.define_method("gen_signal", instrument__gen_signal);
        });
}

class!(Instrument);

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);

#[derive(Debug, Clone)]
#[repr(C)]
pub struct InstrumentInner {}

methods!(
    Instrument,
    itself,
    fn instrument__load(name: RString) -> Instrument {
        let name = name.map_err(|e| VM::raise_ex(e)).unwrap();

        let inner = InstrumentInner::load(name.to_str()).unwrap();

        Class::from_existing("RubyServer").wrap_data(inner, &*INSTRUMENT_WRAPPER)
    },
    fn instrument__neko() -> RString {
        let mut a = Array::from(itself.value());
        a.push(RString::new_utf8("neko"));
        a.pop().convert_or_panic()
    },
    //fn instrument__gen_signal(note: Hash) -> AnyObject {
    //    instrument__gen_signal__inner(itself, note)
    //}
);

fn instrument__gen_signal__inner(itself: Instrument, note: Result<Hash, AnyException>) -> Hash {
    let note = note.map_err(|e| VM::raise_ex(e)).unwrap();

    let velocity: Integer = note.at(&Symbol::new("vel")).convert_or_panic();

    let hash = Hash::new();
    hash
    //hash.store(Symbol::new("sample"), Symbol)

    //note.into_iter().map(|note| match note {
    //    _ if Class::from_existing("NilClass").case_equals(&note) => None,
    //    _ if Class::from_existing("Hash").case_equals(&note) => {
    //        Some(note.try_convert_to::<Hash>().unwrap())
    //    }
    //    _ => panic!("could not convert note #{note:?}"),
    //});

    //match note {
    //    Some(note) => Array::new()
    //        .push(
    //            note.at(&Symbol::new("vel"))
    //                .try_convert_to::<Integer>()
    //                .unwrap(),
    //        )
    //        .to_any_object(),
    //    None => NilClass::new().to_any_object(),
    //}
}

impl InstrumentInner {
    pub fn load(ident: &str) -> Result<Self, ()> {
        Ok(Self {})
    }
}
