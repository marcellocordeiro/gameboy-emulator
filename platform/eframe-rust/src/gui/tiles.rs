use egui::{
    epaint::{ColorImage, TextureHandle, Vec2},
    Color32, Context, Image, TextureOptions, Ui, Window,
};
use gb_core::{
    constants::{
        TileDataFrame, TILE_DATA_FRAME_HEIGHT, TILE_DATA_FRAME_SIZE, TILE_DATA_FRAME_WIDTH,
    },
    GameBoy,
};

use crate::utils::scaling::integer_scaling_size;

pub struct Tiles {
    opened: bool,

    pixels: Box<TileDataFrame>,
    texture: TextureHandle,
}

impl Tiles {
    #[allow(clippy::cast_precision_loss)]
    const DEFAULT_SIZE: Vec2 = Vec2 {
        x: (TILE_DATA_FRAME_WIDTH * 2) as f32,
        y: (TILE_DATA_FRAME_HEIGHT * 2 + 6) as f32, // TODO: don't rely on this.
    };
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let texture = {
            let image = ColorImage::new(
                [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
                Color32::WHITE,
            );

            egui_ctx.load_texture("tiles", image, Self::FILTER)
        };

        Self {
            opened: false,
            pixels: vec![0; TILE_DATA_FRAME_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            texture,
        }
    }

    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("Toggle tiles").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !self.opened {
            return;
        }

        self.update_texture(gb_ctx);

        Window::new("Tiles")
            .open(&mut self.opened)
            .default_size(Self::DEFAULT_SIZE)
            .min_width(Self::DEFAULT_SIZE.x)
            .min_height(Self::DEFAULT_SIZE.y)
            .show(egui_ctx, |ui| {
                let size = integer_scaling_size(ui.available_size(), self.texture.size_vec2());

                ui.centered_and_justified(|ui| {
                    ui.add(Image::from_texture(&self.texture).fit_to_exact_size(size));
                });
            });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx
            .memory
            .ppu
            .vram
            .draw_tile_data_0_into_frame(self.pixels.as_mut());

        #[cfg(feature = "cgb")]
        gb_ctx
            .memory
            .ppu
            .vram
            .draw_tile_data_1_into_frame(self.pixels.as_mut());

        let image = ColorImage::from_rgba_unmultiplied(
            [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
            self.pixels.as_ref(),
        );

        self.texture.set(image, Self::FILTER);
    }
}
