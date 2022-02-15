use std::collections::HashMap;

use crate::{
    impl_inner, inner::track::TRACK_WRAPPER, instrument::Instrument, meta::Meta, ruby_class,
    util::ConvertOrPanic,
    track::Track,
};
use itertools::Itertools;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PieceInner {
    pub meta: Option<Meta>,
    pub instruments: HashMap<String, Instrument>,
    pub tracks: HashMap<String, Track>,
}

impl PieceInner {
    pub fn new() -> Self {
        Self {
            meta: None,
            instruments: HashMap::new(),
            tracks: HashMap::new(),
        }
    }
}

wrappable_struct!(PieceInner, PieceWrapper, PIECE_WRAPPER);
