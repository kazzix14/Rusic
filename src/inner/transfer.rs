use std::collections::HashMap;

use crate::{impl_inner, ruby_class, section::Section, util::ConvertOrPanic};
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, Proc, RString, Symbol, VerifiedObject, GC, VM,
};

#[derive(Debug, Clone)]
pub struct TransferInner {
    pub offset: f32,
    pub out: Option<f32>,
    pub store: HashMap<String, AnyObject>,
}

impl TransferInner {
    pub fn new() -> Self {
        Self {
            offset: 0.0,
            out: None,
            store: HashMap::new(),
        }
    }
}

wrappable_struct!(TransferInner, TransferWrapper, TRANSFER_WRAPPER);
