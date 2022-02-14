use std::collections::HashMap;

use crate::{
    instrument::Instrument,
    meta::Meta,
    ruby_class,
    track::{Track, TRACK_WRAPPER},
    util::ConvertOrPanic,
};
use itertools::Itertools;
use rutie::{
    class, methods, types::Value, wrappable_struct, AnyException, AnyObject, Array, Class, Hash,
    Integer, Module, NilClass, Object, RString, Symbol, VerifiedObject, VM,
};

pub fn define(parent: &mut Module, data_class: &Class) {
    Class::new("Piece", Some(data_class)).define(|class| {
        class.define(|class| {
            class.def_self("new", piece__new);
            class.def("track", piece__track);
            class.def("instrument", piece__instrument);
            class.def("meta", piece__meta);
            class.def("gen", piece__gen);
        });
    });

    parent
        .define_nested_class("Piece", Some(data_class))
        .define(|class| {
            class.define(|class| {
                class.def_self("new", piece__new);
                class.def("track", piece__track);
                class.def("instrument", piece__instrument);
                class.def("meta", piece__meta);
                class.def("gen", piece__gen);
            });
        });
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Piece {
    value: Value,
}

ruby_class!(Piece);
methods!(
    Piece,
    itself,
    fn piece__new() -> AnyObject {
        Piece::new()
    },
    fn piece__track(name: Symbol, instrument_name: Symbol) -> NilClass {
        Piece::track(
            itself,
            name.expect("track name must be specified in Symbol")
                .to_string(),
            instrument_name
                .expect("instrument must be specified in Symbol")
                .to_string(),
        )
    },
    fn piece__instrument(name: Symbol) -> NilClass {
        Piece::instrument(
            itself,
            name.expect("instrument must be specified in Symbol")
                .to_string(),
        )
    },
    fn piece__meta() -> NilClass {
        Piece::meta(itself)
    },
    fn piece__gen() -> NilClass {
        Piece::gen(itself)
    }
);

impl Piece {
    pub fn new() -> AnyObject {
        let inner = PieceInner::new();

        Class::from_existing("Piece").wrap_data(inner, &*PIECE_WRAPPER)
    }

    pub fn gen(mut itself: Piece) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);

        let sample_rate = piece.meta.unwrap().inner().sample_rate;

        // (tracks: Vec<notes: Vec<(samples: Vec<f32>>, offset, start)>, tracks: Vec<estimated_size>)
        let (signals, estimates): (Vec<Vec<(Vec<f32>, f32, f32)>>, Vec<usize>) = piece
            .tracks
            .values()
            .map(|track| track.gen(piece.meta.unwrap().inner().bpm, sample_rate))
            .unzip();

        let signals = signals.into_iter().concat();
        let mut signals = signals.into_iter();
        let estimated_size = estimates.into_iter().max().unwrap();

        // init buffer
        let mut result_signal = Vec::with_capacity(estimated_size);
        unsafe { result_signal.set_len(estimated_size) };
        result_signal.iter_mut().for_each(|v| *v = 0.0);

        // put together
        while let Some((signal, offset, start)) = signals.next() {
            let start = start + offset;
            let mut start = (start * sample_rate) as usize;

            let mut signal = signal.into_iter();
            while let Some(s) = signal.next() {
                let p = unsafe { result_signal.get_unchecked_mut(start) };
                *p += s;
                start += 1;
            }
        }

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();
        result_signal
            .into_iter()
            .for_each(|s| writer.write_sample(s).unwrap());
        writer.finalize().unwrap();

        NilClass::new()
    }

    pub fn meta(mut itself: Piece) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);
        let meta = Meta::new();
        let meta = meta.convert_or_panic();

        VM::yield_object(meta);

        piece.meta = Some(meta);

        NilClass::new()
    }

    pub fn track(mut itself: Piece, name: String, instrument_name: String) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);
        let instrument = piece
            .instruments
            .get(&instrument_name)
            .expect("could not find Instrument `{instrument_name}`");

        let track = Track::new(
            instrument.clone(),
            piece.meta.unwrap().inner().composition.clone(),
        );
        let track = track.convert_or_panic();
        piece.tracks.insert(name, track);

        VM::yield_object(track);

        NilClass::new()
    }

    pub fn instrument(mut itself: Piece, name: String) -> NilClass {
        let piece = itself.get_data_mut(&*PIECE_WRAPPER);

        let instrument = Instrument::new();
        let instrument = instrument.convert_or_panic();
        piece.instruments.insert(name, instrument);

        VM::yield_object(instrument);

        NilClass::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PieceInner {
    meta: Option<Meta>,
    instruments: HashMap<String, Instrument>,
    tracks: HashMap<String, Track>,
}

impl PieceInner {
    pub fn new() -> Self {
        Self {
            meta: None,
            instruments: HashMap::new(),
            tracks: HashMap::new(),
        }
    }
}

wrappable_struct!(PieceInner, PieceWrapper, PIECE_WRAPPER);
