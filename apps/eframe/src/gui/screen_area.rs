use egui::{
    Color32,
    Context,
    Image,
    TextureOptions,
    Ui,
    epaint::{ColorImage, TextureHandle},
};
use gb_core::{
    GameBoy,
    constants::{SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH, ScreenPixels},
};

use crate::{gui::Gui, utils::scaling::integer_scaling_size};

pub struct ScreenArea {
    pixels: Box<ScreenPixels>,
    texture: TextureHandle,
}

impl ScreenArea {
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let pixels = vec![0; SCREEN_PIXELS_SIZE].into_boxed_slice();

        let texture = {
            let image = ColorImage::new(
                [SCREEN_WIDTH, SCREEN_HEIGHT],
                vec![Color32::BLACK; pixels.len() / 4],
            );

            egui_ctx.load_texture("main", image, Self::FILTER)
        };

        Self {
            pixels: pixels.try_into().unwrap(),
            texture,
        }
    }

    pub fn draw(ctx: &mut Gui, ui: &mut Ui, gb_ctx: &GameBoy) {
        ctx.screen_area.update_texture(gb_ctx);

        ui.centered_and_justified(|ui| {
            let size =
                integer_scaling_size(ui.available_size(), ctx.screen_area.texture.size_vec2());

            ui.add(Image::from_texture(&ctx.screen_area.texture).fit_to_exact_size(size));
        });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx.draw_into_frame_rgba8888(self.pixels.as_mut());

        let image =
            ColorImage::from_rgba_unmultiplied([SCREEN_WIDTH, SCREEN_HEIGHT], self.pixels.as_ref());
        self.texture.set(image, Self::FILTER);
    }
}
