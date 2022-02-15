use std::collections::HashMap;

use crate::{
    impl_inner, instrument::Instrument, ruby_class, section::Section, time::Beat,
    util::ConvertOrPanic,
};
use itertools::Itertools;
use num::{Rational32, ToPrimitive};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, GC, VM,
};
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
