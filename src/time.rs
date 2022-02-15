use std::ops::Mul;

use num::{rational::Ratio, Integer};

pub trait Beat {
    fn seconds(self, bpm: f32) -> f32;
}

impl Beat for Ratio<i32> {
    fn seconds(self, bpm: f32) -> f32 {
        let beat = 60.0 / bpm; // 4th
        let bar = 4.0 * beat;
        bar * (*self.numer() as f32) / (*self.denom() as f32)
    }
}
