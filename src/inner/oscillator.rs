use rutie::wrappable_struct;

#[derive(Debug, Clone, Copy)]
pub struct OscillatorInner {
    pub phase: f64,
}

impl OscillatorInner {
    pub fn new(phase: f64) -> Self {
        Self { phase }
    }

    pub fn push_phase(&mut self, frequency: f64, delta: f64) {
        self.phase = self.phase + frequency * delta;
    }
}

wrappable_struct!(OscillatorInner, OscillatorWrapper, OSCILLATOR_WRAPPER);
