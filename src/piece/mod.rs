use crate::{
    track::{Track, TRACK_WRAPPER},
    util::ConvertOrPanic,
};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Piece", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def_self("new", piece__new);
            klass.def("track", piece__track);
        });
    });

    parent
        .define_nested_class("Piece", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def_self("new", piece__new);
                klass.def("track", piece__track);
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
    fn piece__track(name: Symbol) -> NilClass {
        Piece::track(
            itself,
            name.expect("track name must be specified in Symbol")
                .to_string(),
        )
    }
);

impl Piece {
    pub fn new() -> AnyObject {
        let inner = PieceInner::new();

        Class::from_existing("Piece").wrap_data(inner, &*PIECE_WRAPPER)
    }

    pub fn track(mut itself: Piece, name: String) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);
        let track = Track::new(name);
        let track = track.convert_or_panic();
        piece.tracks.push(track);

        VM::yield_object(track);

        NilClass::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PieceInner {
    tracks: Vec<Track>,
}

impl PieceInner {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }
}

wrappable_struct!(PieceInner, PieceWrapper, PIECE_WRAPPER);
