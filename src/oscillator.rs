use std::any::Any;

use crate::{impl_inner, inner::oscillator::*, ruby_class};

use rutie::{methods, types::Value, AnyObject, Class, Float, Object};

pub fn define_class(super_class: &Class) {
    Class::new("Oscillator", Some(super_class)).define(|class| {
        class.def_self("new", oscillator_new);
        class.def("sin", oscillator_sin);
        class.def("saw", oscillator_saw);
        class.def("square", oscillator_square);
    });
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Oscillator {
    value: Value,
}

impl Oscillator {
    pub fn new(phase: Float) -> AnyObject {
        let inner = OscillatorInner::new(phase.to_f64());

        Class::from_existing("Oscillator").wrap_data(inner, &*OSCILLATOR_WRAPPER)
    }

    pub fn sin(&mut self, frequency: f64, delta: f64) -> f64 {
        let osc = self.inner_mut();

        osc.push_phase(frequency, delta);

        (osc.phase * std::f64::consts::TAU).sin()
    }

    pub fn saw(&mut self, frequency: f64, delta: f64) -> f64 {
        let osc = self.inner_mut();

        osc.push_phase(frequency, delta);

        2.0 * (osc.phase.rem_euclid(1.0) - 0.5)
    }

    pub fn square(&mut self, frequency: f64, delta: f64) -> f64 {
        let osc = self.inner_mut();

        osc.push_phase(frequency, delta);

        2.0 * (osc.phase.rem_euclid(1.0) - 0.5)
    }
}

ruby_class!(Oscillator);
impl_inner!(Oscillator, OscillatorInner, OSCILLATOR_WRAPPER);
methods!(
    Oscillator,
    itself,
    fn oscillator_new(p: Float) -> AnyObject {
        Oscillator::new(p.unwrap())
    },
    fn oscillator_sin(f: Float, d: Float) -> Float {
        Float::new(itself.sin(f.unwrap().to_f64(), d.unwrap().to_f64()))
    },
    fn oscillator_saw(f: Float, d: Float) -> Float {
        Float::new(itself.saw(f.unwrap().to_f64(), d.unwrap().to_f64()))
    },
    fn oscillator_square(f: Float, d: Float) -> Float {
        Float::new(itself.square(f.unwrap().to_f64(), d.unwrap().to_f64()))
    }
);
