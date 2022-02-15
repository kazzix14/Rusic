use std::collections::HashMap;

use rutie::{wrappable_struct, AnyObject, GC};

#[derive(Debug, Clone)]
pub struct InstrumentDtoInner {
    pub offset: f32,
    pub out: Option<f32>,
    pub store: HashMap<String, AnyObject>,
}

impl InstrumentDtoInner {
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            out: None,
            store: HashMap::new(),
        }
    }
}

wrappable_struct!(
    InstrumentDtoInner,
    InstrumentDtoWrapper,
    TRANSFER_WRAPPER,
    mark(data) {
        for v in data.store.values() {
            GC::mark(v);
        }
    }
);
