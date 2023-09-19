#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::multiple_inherent_impl,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

use gb_core::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    cpu::Cpu,
};

#[wasm_bindgen]
pub fn init_logging() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Trace).unwrap();
}

#[wasm_bindgen]
pub struct GameBoy {
    cpu: Cpu,
}

#[wasm_bindgen]
impl GameBoy {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut cpu = Cpu::default();

        #[cfg(not(feature = "bootrom"))]
        cpu.skip_bootrom();

        Self { cpu }
    }

    pub fn reset(&mut self) {
        *self = GameBoy::new();
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.reset();

        self.cpu.memory.load_cartridge(rom).unwrap();
    }

    pub fn run_frame(&mut self) {
        self.cpu.cycles = 0;
        while self.cpu.cycles < (70224 * 2) {
            self.cpu.step();
        }
    }

    #[allow(clippy::identity_op)]
    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let frame = &mut [0u8; SCREEN_WIDTH * SCREEN_HEIGHT * 4];

        self.cpu.memory.graphics.draw_into_frame(frame);

        let img_data =
            ImageData::new_with_u8_clamped_array(Clamped(frame), SCREEN_WIDTH as u32).unwrap();
        ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }
}
