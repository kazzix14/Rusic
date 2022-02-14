use std::collections::HashMap;

use crate::{instrument::Instrument, ruby_class, section::Section, util::ConvertOrPanic};
use itertools::Itertools;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Track", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def("symbol", track__symbol);
            klass.def("section", track__section);
        });
    });

    parent
        .define_nested_class("Track", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def("symbol", track__symbol);
                klass.def("section", track__section);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Track {
    value: Value,
}

ruby_class!(Track);
methods!(
    Track,
    itself,
    fn track__symbol(key: Symbol, value: Hash) -> NilClass {
        Track::symbol(itself, key.unwrap(), value.unwrap())
    },
    fn track__section(name: Symbol) -> NilClass {
        let name = name
            .expect("section name must be specified in Symbol")
            .to_string();

        Track::section(itself, name)
    },
);

impl Track {
    pub fn new(instrument: Instrument, composition: Vec<String>) -> AnyObject {
        let inner = TrackInner::new(instrument, composition);

        Class::from_existing("Track").wrap_data(inner, &*TRACK_WRAPPER)
    }

    pub fn symbol(mut itself: Track, key: Symbol, value: Hash) -> NilClass {
        let track = itself.get_data_mut(&*TRACK_WRAPPER);

        track.symbols.insert(key.to_string(), value);

        NilClass::new()
    }

    pub fn section(mut itself: Track, name: String) -> NilClass {
        let track = itself.get_data_mut(&*TRACK_WRAPPER);
        let section = Section::new(track.symbols.clone());
        let section = section.convert_or_panic();
        VM::yield_object(section);

        track.sections.insert(name, section);

        NilClass::new()
    }

    pub fn gen(&self, sample_rate: f32) -> Vec<f32> {
        let track = self.get_data(&*TRACK_WRAPPER);
        let mut instrument = track.instrument;

        instrument.exec_init();

        let notes = track
            .composition
            .iter()
            .map(|section_name| {
                track
                    .sections
                    .get(section_name)
                    .expect(&format!("could not find section: {section_name}"))
            })
            .map(|section| section.get_sheet())
            .concat();

        let mut track_signal = Vec::new();
        let mut notes = notes.into_iter();
        while let Some(note) = notes.next() {
            instrument.exec_before_each_note(&note);

            let mut time = 0.0;
            while let Some(signal) = instrument.exec_signal(&note, time) {
                time += 1.0 / sample_rate;

                track_signal.push(signal);
            }
        }

        track_signal
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

#[derive(Debug)]
pub struct TrackInner {
    pub instrument: Instrument,
    pub symbols: HashMap<String, Hash>,
    pub sections: HashMap<String, Section>,
    pub composition: Vec<String>,
}

impl TrackInner {
    pub fn new(instrument: Instrument, composition: Vec<String>) -> Self {
        Self {
            instrument: instrument,
            symbols: HashMap::new(),
            sections: HashMap::new(),
            composition: composition,
        }
    }
}

wrappable_struct!(TrackInner, TrackWrapper, TRACK_WRAPPER);
