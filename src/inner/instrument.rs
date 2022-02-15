use crate::{instrument_dto::*, util::ConvertOrPanic};

use rutie::{wrappable_struct, Proc, GC};

#[derive(Debug)]
pub struct InstrumentInner {
    pub init_fn: Option<Proc>,
    pub before_each_note_fn: Option<Proc>,
    pub signal_fn: Option<Proc>,
    pub instrument_dto: InstrumentDto,
}

impl InstrumentInner {
    pub fn new() -> Self {
        Self {
            init_fn: None,
            before_each_note_fn: None,
            signal_fn: None,
            instrument_dto: InstrumentDto::new().convert_or_panic(),
        }
    }
}

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER,
    mark(data) {
        if let Some(f) = &data.init_fn {
            GC::mark(f);
        }
        if let Some(f) = &data.before_each_note_fn{
            GC::mark(f);
        }
        if let Some(f) = &data.signal_fn{
            GC::mark(f);
        }
    }
);
