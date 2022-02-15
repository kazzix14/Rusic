use std::collections::HashMap;

use crate::{instrument::Instrument, meta::Meta, track::Track};

use rutie::wrappable_struct;

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
