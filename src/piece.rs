use crate::{
    impl_inner, inner::piece::*, instrument::Instrument, meta::Meta, ruby_class, track::Track,
    util::ConvertOrPanic,
};

use itertools::Itertools;
use rutie::{methods, types::Value, AnyObject, Class, Module, NilClass, Object, Symbol, VM};

pub fn define_class(parent: &mut Module, super_class: &Class) {
    Class::new("Piece", Some(super_class)).define(|class| {
        class.def_self("new", piece_new);
        class.def("track", piece_track);
        class.def("instrument", piece_instrument);
        class.def("meta", piece_meta);
        class.def("gen", piece_gen);
    });

    parent
        .define_nested_class("Piece", Some(super_class))
        .define(|class| {
            class.def_self("new", piece_new);
            class.def("track", piece_track);
            class.def("instrument", piece_instrument);
            class.def("meta", piece_meta);
            class.def("gen", piece_gen);
        });
}

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct Piece {
    value: Value,
}

impl Piece {
    pub fn new() -> AnyObject {
        let inner = PieceInner::new();

        Class::from_existing("Piece").wrap_data(inner, &*PIECE_WRAPPER)
    }

    pub fn gen(&mut self) -> NilClass {
        let piece = self.get_data_mut(&*PIECE_WRAPPER);

        let sample_rate = piece.meta.unwrap().inner().sample_rate;

        // (tracks: Vec<notes: Vec<(samples: Vec<f32>>, offset, start)>, tracks: Vec<estimated_size>)
        let (signals, estimates): (Vec<Vec<(Vec<f32>, f32)>>, Vec<usize>) = piece
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
        while let Some((signal, start_at)) = signals.next() {
            let mut start = (start_at * sample_rate) as usize;

            let mut signal = signal.into_iter();
            while let Some(s) = signal.next() {
                let p = unsafe { result_signal.get_unchecked_mut(start) };
                *p += s;
                start += 1;
            }
        }

        if let Some(v) = result_signal.iter().find(|&&v| v <= -1.0 || 1.0 <= v) {
            println!("warning: audio is clipping. {v}");
        }

        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };

        let mut writer = hound::WavWriter::create("out.wav", spec).unwrap();
        result_signal.into_iter().for_each(|s| {
            writer.write_sample(s).unwrap();
            writer.write_sample(s).unwrap();
        });
        writer.finalize().unwrap();

        NilClass::new()
    }

    pub fn meta(&mut self) -> NilClass {
        let piece = self.get_data_mut(&*PIECE_WRAPPER);
        let meta = Meta::new();
        let meta = meta.convert_or_panic();

        VM::yield_object(meta);

        piece.meta = Some(meta);

        NilClass::new()
    }

    pub fn track(&mut self, name: String, instrument_name: String) -> NilClass {
        let piece = self.get_data_mut(&*PIECE_WRAPPER);
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

    pub fn instrument(&mut self, name: String) -> NilClass {
        let piece = self.get_data_mut(&*PIECE_WRAPPER);

        let instrument = Instrument::new();
        let instrument = instrument.convert_or_panic();
        piece.instruments.insert(name, instrument);

        VM::yield_object(instrument);

        NilClass::new()
    }
}

ruby_class!(Piece);
impl_inner!(Piece, PieceInner, PIECE_WRAPPER);
methods!(
    Piece,
    itself,
    fn piece_new() -> AnyObject {
        Piece::new()
    },
    fn piece_track(name: Symbol, instrument_name: Symbol) -> NilClass {
        itself.track(
            name.expect("track name must be specified in Symbol")
                .to_string(),
            instrument_name
                .expect("instrument must be specified in Symbol")
                .to_string(),
        )
    },
    fn piece_instrument(name: Symbol) -> NilClass {
        itself.instrument(
            name.expect("instrument must be specified in Symbol")
                .to_string(),
        )
    },
    fn piece_meta() -> NilClass {
        itself.meta()
    },
    fn piece_gen() -> NilClass {
        itself.gen()
    }
);
