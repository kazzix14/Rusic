#[derive(Debug, Clone)]
pub struct MetaInner {
    pub bpm: f32,
    pub sample_rate: f32,
    pub composition: Vec<String>,
}

impl MetaInner {
    pub fn new() -> Self {
        Self {
            bpm: 120.0,
            sample_rate: 44100.0,
            composition: Vec::new(),
        }
    }
}

wrappable_struct!(MetaInner, MetaWrapper, META_WRAPPER);
