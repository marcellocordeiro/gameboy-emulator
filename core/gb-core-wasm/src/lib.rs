#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use gb_core::{GameBoy as GameBoyInternal, ScreenPixels, SCREEN_PIXELS_SIZE, SCREEN_WIDTH};
use wasm_bindgen::{prelude::wasm_bindgen, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

fn init_logging() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).unwrap();
}

#[wasm_bindgen]
pub struct GameBoy {
    gb: GameBoyInternal,
    frame: Box<ScreenPixels>,
}

#[wasm_bindgen]
impl GameBoy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        init_logging();

        Self {
            gb: GameBoyInternal::new(),
            frame: vec![0; SCREEN_PIXELS_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    pub fn reset(&mut self) {
        self.gb.reset();
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.gb.load_cartridge(rom).unwrap();
    }

    pub fn run_frame(&mut self) {
        self.gb.run_frame();
    }

    pub fn draw(&mut self, ctx: &CanvasRenderingContext2d) {
        self.gb.draw_into_frame_rgba8888(self.frame.as_mut());

        let img_data =
            ImageData::new_with_u8_clamped_array(Clamped(self.frame.as_ref()), SCREEN_WIDTH as u32)
                .unwrap();
        ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }
}
