use std::collections::HashMap;

use rutie::{wrappable_struct, AnyObject};

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
