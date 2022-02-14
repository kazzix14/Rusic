use std::collections::HashMap;

use crate::{
    instrument::Instrument,
    track::{Track, TRACK_WRAPPER},
    util::ConvertOrPanic,
};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Piece", Some(data_class)).define(|class| {
        class.define(|class| {
            class.def_self("new", piece__new);
            class.def("track", piece__track);
            class.def("instrument", piece__instrument);
        });
    });

    parent
        .define_nested_class("Piece", Some(data_class))
        .define(|class| {
            class.define(|class| {
                class.def_self("new", piece__new);
                class.def("track", piece__track);
                class.def("instrument", piece__instrument);
            });
        });
}

class!(Piece);
methods!(
    Piece,
    itself,
    fn piece__new() -> AnyObject {
        Piece::new()
    },
    fn piece__track(name: Symbol, instrument_name: Symbol) -> NilClass {
        Piece::track(
            itself,
            name.expect("track name must be specified in Symbol")
                .to_string(),
            instrument_name
                .expect("instrument must be specified in Symbol")
                .to_string(),
        )
    },
    fn piece__instrument(name: Symbol) -> NilClass {
        Piece::instrument(
            itself,
            name.expect("instrument must be specified in Symbol")
                .to_string(),
        )
    }
);

impl Piece {
    pub fn new() -> AnyObject {
        let inner = PieceInner::new();

        Class::from_existing("Piece").wrap_data(inner, &*PIECE_WRAPPER)
    }

    pub fn track(mut itself: Piece, name: String, instrument_name: String) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);
        let instrument = piece
            .instruments
            .get(&instrument_name)
            .expect("could not find Instrument `{instrument_name}`");

        let track = Track::new(instrument.clone());
        let track = track.convert_or_panic();
        piece.tracks.insert(name, track);

        VM::yield_object(track);

        NilClass::new()
    }

    pub fn instrument(mut itself: Piece, name: String) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);

        let instrument = Instrument::new();
        let instrument = instrument.convert_or_panic();
        piece.instruments.insert(name, instrument);

        VM::yield_object(instrument);

        NilClass::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PieceInner {
    instruments: HashMap<String, Instrument>,
    tracks: HashMap<String, Track>,
}

impl PieceInner {
    pub fn new() -> Self {
        Self {
            instruments: HashMap::new(),
            tracks: HashMap::new(),
        }
    }
}

wrappable_struct!(PieceInner, PieceWrapper, PIECE_WRAPPER);
