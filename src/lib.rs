#[macro_use]
extern crate rutie;
#[macro_use]
extern crate lazy_static;

//mod inner;
mod instrument;
mod time;
mod piece;
mod section;
mod track;
mod util;

use crate::util::ConvertOrPanic;
use rutie::{
    class, methods, module, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class,
    Hash, Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

module!(Jungrust);

#[no_mangle]
pub extern "C" fn init_jungru() {
    let data_class = Class::from_existing("Object");

    Module::new("Jungru").define(|module| {
        piece::define(module, &data_class);
        track::define(module, &data_class);
        section::define(module, &data_class);
        //instrument::define(module, &data_class);
    });
}
