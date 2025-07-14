#[derive(Debug, Default)]
pub struct HighPassFilter {
    capacitor: f32,
}

impl HighPassFilter {
    // https://gbdev.io/pandocs/Audio_details.html#obscure-behavior
    pub fn apply(&mut self, in_sample: f32) -> f32 {
        // 0.999958 ^ (4194304 / AUDIO_SAMPLE_RATE)
        const FACTOR: f32 = 0.996; // At 44.1kHz

        let out = in_sample - self.capacitor;
        self.capacitor = out.mul_add(-FACTOR, in_sample); // in_sample - (out * FACTOR)

        out
    }
}
