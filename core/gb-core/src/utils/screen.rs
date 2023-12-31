use super::color::Color;
use crate::constants::{ScreenPixels, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct Screen {
    pub screen: Box<[Color; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen {
    pub fn new() -> Self {
        Self {
            screen: vec![Color::SYSTEM_DEFAULT; SCREEN_WIDTH * SCREEN_HEIGHT]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    pub fn draw_into_frame_rgba8888(&self, frame: &mut ScreenPixels) {
        for (i, pixel) in self.screen.iter().enumerate() {
            frame[i * 4] = pixel.red;
            frame[(i * 4) + 1] = pixel.green;
            frame[(i * 4) + 2] = pixel.blue;
            frame[(i * 4) + 3] = 0xFF;
        }
    }

    pub fn draw_into_frame_bgra8888(&self, frame: &mut ScreenPixels) {
        for (i, pixel) in self.screen.iter().enumerate() {
            frame[i * 4] = pixel.blue;
            frame[(i * 4) + 1] = pixel.green;
            frame[(i * 4) + 2] = pixel.red;
            frame[(i * 4) + 3] = 0xFF;
        }
    }
}
