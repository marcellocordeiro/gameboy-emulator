use gb_core::{
    GameBoy as GameBoyInternal,
    constants::{DeviceModel, SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH, ScreenPixels},
};
use wasm_bindgen::{Clamped, prelude::wasm_bindgen};
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn init_logging() {
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            gb: GameBoyInternal::new(DeviceModel::Cgb),
            frame: vec![0; SCREEN_PIXELS_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    #[wasm_bindgen(js_name = "screenWidth")]
    #[must_use]
    pub fn screen_width() -> usize {
        SCREEN_WIDTH
    }

    #[wasm_bindgen(js_name = "screenHeight")]
    #[must_use]
    pub fn screen_height() -> usize {
        SCREEN_HEIGHT
    }

    pub fn reset(&mut self) {
        self.gb.reset();
    }

    pub fn load(&mut self, rom: Vec<u8>) {
        self.gb.load(None, rom).unwrap();
    }

    #[wasm_bindgen(js_name = "runFrame")]
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
