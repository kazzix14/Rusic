use crate::{instrument::Instrument, section::Section};

use std::collections::HashMap;

use rutie::Hash;

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
