#[macro_use]
extern crate rutie;
#[macro_use]
extern crate lazy_static;

//mod inner;
mod inner;
mod instrument;
mod meta;
mod oscillator;
mod piece;
mod section;
mod support;
mod time;
mod track;
mod transfer;
#[macro_use]
mod util;

use rutie::{
    class, methods, module, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class,
    Hash, Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, GC, VM,
};

module!(Jungru);

#[no_mangle]
pub extern "C" fn init_jungru() {
    let data_class = Class::from_existing("Object");

    Module::new("Jungru").define(|module| {
        piece::define(module, &data_class);
        instrument::define(module, &data_class);
        meta::define(module, &data_class);
        track::define(module, &data_class);
        section::define(module, &data_class);
        transfer::define(module, &data_class);
        support::define(module, &data_class);
        oscillator::define_class(&data_class);
    });
}
