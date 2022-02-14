use std::{collections::HashMap, str::Chars};

use crate::{instrument::Instrument, ruby_class, section::Section, util::ConvertOrPanic};
use itertools::PeekingNext;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Float,
    Hash, Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Meta", Some(data_class)).define(|class| {
        class.define(|klass| {
            klass.def("bpm", meta__bpm);
            klass.def("composite", meta__composite);
        });
    });

    parent
        .define_nested_class("Meta", Some(data_class))
        .define(|class| {
            class.define(|klass| {
                klass.def("bpm", meta__bpm);
                klass.def("composite", meta__composite);
            });
        });
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Meta {
    value: Value,
}

ruby_class!(Meta);
methods!(
    Meta,
    itself,
    fn meta__bpm(bpm: Float) -> NilClass {
        Meta::bpm(itself, bpm.unwrap())
    },
    fn meta__composite(composition: RString) -> NilClass {
        Meta::composite(itself, composition.unwrap().to_string())
    },
);

impl Meta {
    pub fn new() -> AnyObject {
        let inner = MetaInner::new();

        Class::from_existing("Meta").wrap_data(inner, &*META_WRAPPER)
    }

    pub fn bpm(mut itself: Meta, bpm: Float) -> NilClass {
        let meta = itself.get_data_mut(&*META_WRAPPER);

        meta.bpm = bpm.to_f64() as f32;

        NilClass::new()
    }

    pub fn composite(mut itself: Meta, source: String) -> NilClass {
        let meta = itself.get_data_mut(&*META_WRAPPER);
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

    pub fn to_any_object(&self) -> AnyObject {
        AnyObject::from(self.value())
    }

    pub fn inner(&self) -> &MetaInner {
        self.get_data(&*META_WRAPPER)
    }
}

fn read_alphanumeric(initial: char, source: &mut Chars) -> String {
    let mut s = initial.to_string();

    while let Some(c) = source.peeking_next(|c| c.is_alphanumeric()) {
        s.push(c);
    }

    s
}

#[derive(Debug, Clone)]
pub struct MetaInner {
    pub bpm: f32,
    pub composition: Vec<String>,
}

impl MetaInner {
    pub fn new() -> Self {
        Self {
            bpm: 120.0,
            composition: Vec::new(),
        }
    }
}

wrappable_struct!(MetaInner, MetaWrapper, META_WRAPPER);
