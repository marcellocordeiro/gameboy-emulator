#[derive(Debug, Default)]
pub struct FrameSequencer {
    step: usize,
}

impl FrameSequencer {
    pub fn next_step(&mut self) -> usize {
        let next_step = self.step;
        self.step = (self.step + 1) & 0b111;

        next_step
    }
}
