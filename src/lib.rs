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
mod util;

use rutie::{module, Class, Module, Object};

module!(Jungru);

#[no_mangle]
pub extern "C" fn init_jungru() {
    let data_class = Class::from_existing("Object");

    Module::new("Jungru").define(|module| {
        piece::define_class(module, &data_class);
    });

    instrument::define_class(&data_class);
    meta::define_class(&data_class);
    track::define_class(&data_class);
    section::define_class(&data_class);
    transfer::define_class(&data_class);
    support::define_class(&data_class);
    oscillator::define_class(&data_class);
}
