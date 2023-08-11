use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{CanvasRenderingContext2d, ImageData};

use gameboy_emulator_core::{
    constants::{HEIGHT, WIDTH},
    GameBoy,
};

#[wasm_bindgen]
pub struct GameBoyWasm {
    gb: GameBoy,
}

#[wasm_bindgen]
impl GameBoyWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameBoyWasm {
        let gb = GameBoy::new();
        GameBoyWasm { gb }
    }

    pub fn reset(&mut self) {
        self.gb.reset();
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.gb.load_cartridge(rom);
    }

    pub fn run_frame(&mut self) {
        self.gb.run_frame();
    }

    pub fn draw(&self, ctx: CanvasRenderingContext2d) {
        let pixels = &mut [0u8; WIDTH * HEIGHT * 4];

        self.gb.draw(pixels);

        let img_data = ImageData::new_with_u8_clamped_array(Clamped(pixels), WIDTH as u32).unwrap();
        ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
    }
}
