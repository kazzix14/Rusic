#[macro_use]
extern crate lazy_static;
extern crate rutie;

mod inner;
mod instrument;
mod instrument_dto;
mod meta;
mod oscillator;
mod piece;
mod section;
mod time;
mod track;
mod util;

use rutie::{Class, GC};

//module!(Jungru);

#[no_mangle]
pub extern "C" fn init_jungru() {
    let data_class = Class::from_existing("Object");

    GC::disable();

    //Module::new("Jungru").define(|module| {
    //    piece::define_class(module, &data_class);
    //    //oscillator::define_class(module, &data_class);
    //});

    piece::define_class(&data_class);
    instrument::define_class(&data_class);
    meta::define_class(&data_class);
    track::define_class(&data_class);
    section::define_class(&data_class);
    instrument_dto::define_class(&data_class);
    oscillator::define_class(&data_class);
}
