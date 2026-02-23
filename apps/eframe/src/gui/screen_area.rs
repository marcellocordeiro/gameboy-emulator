use egui::{
    Color32,
    Context,
    TextureOptions,
    Ui,
    epaint::{ColorImage, TextureHandle},
};
use gb_core::{
    GameBoy,
    constants::{SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH},
};

use crate::{gui::Gui, utils::scaling::integer_scaling_size};

pub struct ScreenArea {
    image: ColorImage,
    texture: TextureHandle,
}

impl ScreenArea {
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let image = ColorImage::new(
            [SCREEN_WIDTH, SCREEN_HEIGHT],
            vec![Color32::BLACK; SCREEN_PIXELS_SIZE / 4],
        );

        let texture = egui_ctx.load_texture("main", image.clone(), Self::FILTER);

        Self { image, texture }
    }

    pub fn draw(ctx: &Gui, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            let size =
                integer_scaling_size(ui.available_size(), ctx.screen_area.texture.size_vec2());

            ui.image((ctx.screen_area.texture.id(), size));
        });
    }

    pub fn update(ctx: &mut Gui, gb_ctx: &GameBoy) {
        let screen = gb_ctx.screen();

        for (dst, src) in ctx
            .screen_area
            .image
            .pixels
            .iter_mut()
            .zip(screen.pixels.iter())
        {
            *dst = Color32::from_rgba_unmultiplied(src.red, src.green, src.blue, src.alpha);
        }

        ctx.screen_area
            .texture
            .set(ctx.screen_area.image.clone(), Self::FILTER);
    }
}
