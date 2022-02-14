use std::collections::HashMap;

use crate::instrument::InstrumentInner;

use rutie::{wrappable_struct, Hash, Symbol};

wrappable_struct!(TrackInner, TrackWrapper, TRACK_WRAPPER);

pub struct TrackInner {
    pub name: String,
    pub instrument: Option<InstrumentInner>,
    pub symbols: HashMap<String, Hash>,
}

impl TrackInner {
    pub fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or(String::new()),
            instrument: None,
            symbols: HashMap::new(),
        }
    }
}
