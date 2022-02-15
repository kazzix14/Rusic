use crate::{impl_inner, inner::meta::*, ruby_class};

use std::str::Chars;

use itertools::PeekingNext;
use rutie::{methods, types::Value, AnyObject, Class, Float, NilClass, Object, RString};

pub fn define_class(super_class: &Class) {
    Class::new("Meta", Some(super_class)).define(|class| {
        class.def("bpm", meta_bpm);
        class.def("sample_rate", meta_sample_rate);
        class.def("composite", meta_composite);
    });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Meta {
    value: Value,
}

impl Meta {
    pub fn new() -> AnyObject {
        let inner = MetaInner::new();

        Class::from_existing("Meta").wrap_data(inner, &*META_WRAPPER)
    }

    pub fn bpm(&mut self, bpm: Float) -> NilClass {
        let meta = self.get_data_mut(&*META_WRAPPER);

        meta.bpm = bpm.to_f64() as f32;

        NilClass::new()
    }

    pub fn sample_rate(&mut self, sample_rate: Float) -> NilClass {
        let meta = self.get_data_mut(&*META_WRAPPER);

        meta.sample_rate = sample_rate.to_f64() as f32;

        NilClass::new()
    }

    pub fn composite(&mut self, source: String) -> NilClass {
        let meta = self.get_data_mut(&*META_WRAPPER);
        let source = source.to_string();
        let mut source = source.chars();
        let mut composition = Vec::new();

        while let Some(c) = source.next() {
            if let Some(s) = match c {
                _ if c.is_whitespace() => None,
                _ if c.is_alphanumeric() => Some(read_alphanumeric(c, &mut source)),
                _ => panic!("unexpected token in composition"),
            } {
                composition.push(s);
            }
        }

        meta.composition = composition;

        NilClass::new()
    }
}

fn read_alphanumeric(initial: char, source: &mut Chars) -> String {
    let mut s = initial.to_string();

    while let Some(c) = source.peeking_next(|c| c.is_alphanumeric()) {
        s.push(c);
    }

    s
}

ruby_class!(Meta);
impl_inner!(Meta, MetaInner, META_WRAPPER);
methods!(
    Meta,
    itself,
    fn meta_bpm(bpm: Float) -> NilClass {
        itself.bpm(bpm.unwrap())
    },
    fn meta_sample_rate(sample_rate: Float) -> NilClass {
        itself.sample_rate(sample_rate.unwrap())
    },
    fn meta_composite(composition: RString) -> NilClass {
        itself.composite(composition.unwrap().to_string())
    },
);
