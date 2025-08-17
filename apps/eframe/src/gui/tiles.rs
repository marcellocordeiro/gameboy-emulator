use egui::{
    Color32,
    Context,
    Image,
    TextureOptions,
    Ui,
    Window,
    epaint::{ColorImage, TextureHandle, Vec2},
};
use gb_core::{
    GameBoy,
    constants::{
        DeviceModel,
        TILE_DATA_FRAME_HEIGHT,
        TILE_DATA_FRAME_SIZE_CGB,
        TILE_DATA_FRAME_WIDTH_CGB,
        TileDataFrameCgb,
    },
};

use crate::{gui::Gui, utils::scaling::integer_scaling_size};

pub struct Tiles {
    opened: bool,

    pixels: Box<TileDataFrameCgb>,
    texture: TextureHandle,
}

impl Tiles {
    #[allow(clippy::cast_precision_loss)]
    const DEFAULT_SIZE: Vec2 = Vec2 {
        x: (TILE_DATA_FRAME_WIDTH_CGB * 2) as f32,
        y: (TILE_DATA_FRAME_HEIGHT * 2 + 6) as f32, // TODO: don't rely on this.
    };
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let pixels = vec![0; TILE_DATA_FRAME_SIZE_CGB].into_boxed_slice();
        let texture = {
            let image = ColorImage::new(
                [TILE_DATA_FRAME_WIDTH_CGB, TILE_DATA_FRAME_HEIGHT],
                vec![Color32::BLACK; pixels.len() / 4],
            );

            egui_ctx.load_texture("tiles", image, Self::FILTER)
        };

        Self {
            opened: false,
            pixels: pixels.try_into().unwrap(),
            texture,
        }
    }

    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut Ui) {
        if ui.button("Tiles").clicked() {
            ctx.tiles.opened = !ctx.tiles.opened;
        }
    }

    pub fn draw(ctx: &mut Gui, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !ctx.tiles.opened {
            return;
        }

        ctx.tiles.update_texture(gb_ctx);

        Window::new("Tiles")
            .open(&mut ctx.tiles.opened)
            .default_size(Self::DEFAULT_SIZE)
            .min_width(Self::DEFAULT_SIZE.x)
            .min_height(Self::DEFAULT_SIZE.y)
            .show(egui_ctx, |ui| {
                let size = integer_scaling_size(ui.available_size(), ctx.tiles.texture.size_vec2());

                ui.centered_and_justified(|ui| {
                    ui.add(Image::from_texture(&ctx.tiles.texture).fit_to_exact_size(size));
                });
            });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx
            .memory()
            .ppu
            .vram
            .draw_tile_data_0_into_frame(self.pixels.as_mut());

        if gb_ctx.device_model == DeviceModel::Cgb {
            gb_ctx
                .memory()
                .ppu
                .vram
                .draw_tile_data_1_into_frame(self.pixels.as_mut());
        }

        let image = ColorImage::from_rgba_unmultiplied(
            [TILE_DATA_FRAME_WIDTH_CGB, TILE_DATA_FRAME_HEIGHT],
            self.pixels.as_ref(),
        );

        self.texture.set(image, Self::FILTER);
    }
}
