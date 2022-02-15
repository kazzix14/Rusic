use crate::{transfer::*, util::ConvertOrPanic};

use rutie::{wrappable_struct, Proc};

#[derive(Debug)]
pub struct InstrumentInner {
    pub init_fn: Option<Proc>,
    pub before_each_note_fn: Option<Proc>,
    pub signal_fn: Option<Proc>,
    pub transfer: Transfer,
}

impl InstrumentInner {
    pub fn new() -> Self {
        Self {
            init_fn: None,
            before_each_note_fn: None,
            signal_fn: None,
            transfer: Transfer::new().convert_or_panic(),
        }
    }
}

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);