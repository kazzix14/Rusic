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
