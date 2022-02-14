use std::collections::HashMap;

use crate::{instrument::Instrument, section::Section, util::ConvertOrPanic};
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

    pub fn gen(&self) {
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

        let mut notes = notes.into_iter();
        while let Some(note) = notes.next() {
            instrument.exec_before_each_note(note);
        }
        //instrument.exec_signal();
    }

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }
}

impl From<Value> for Track {
    fn from(value: Value) -> Self {
        Track { value }
    }
}

impl TryFrom<AnyObject> for Track {
    type Error = std::io::Error;

    fn try_from(obj: AnyObject) -> Result<Track, Self::Error> {
        if Class::from_existing("Track").case_equals(&obj) {
            Ok(Track::from(obj.value()))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::AddrInUse,
                "aaaaaaaaaokkkkkkk",
            ))
        }
    }
}

impl Object for Track {
    #[inline]
    fn value(&self) -> Value {
        self.value
    }
}

impl VerifiedObject for Track {
    fn is_correct_type<T: Object>(object: &T) -> bool {
        Class::from_existing("Track").case_equals(object)
    }

    fn error_message() -> &'static str {
        "Error converting to Track"
    }
}

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

use crate::instrument::InstrumentInner;

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
