#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Time {
    Beat(Beat),
    Second(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Beat {
    pub numerator: u32,
    pub denominator: u32,
}

impl Beat {
    pub fn seconds(self, bpm: f32) -> f32 {
        let beat = 60.0 / bpm; // 4th
        let bar = 4.0 * beat;
        bar * self.numerator as f32 / self.denominator as f32
    }
}
