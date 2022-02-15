use std::collections::HashMap;

use crate::{impl_inner, ruby_class, section::Section, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, GC, VM,
};

use crate::transfer::*;

wrappable_struct!(InstrumentInner, InstrumentWrapper, INSTRUMENT_WRAPPER);
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
