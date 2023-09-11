use eframe::egui;
use egui::{
    epaint::{ColorImage, Vec2},
    Context, Window,
};
use gb_core::{
    constants::{TileDataFrame, TILE_DATA_FRAME_HEIGHT, TILE_DATA_FRAME_WIDTH},
    GameBoy,
};

pub struct Tiles {
    opened: bool,

    pixels_bank0: TileDataFrame,
    texture_bank0: egui::TextureHandle,
}

impl Tiles {
    const DEFAULT_SIZE: Vec2 = Vec2 {
        x: TILE_DATA_FRAME_WIDTH as f32 * 2.0,
        y: TILE_DATA_FRAME_HEIGHT as f32 * 2.05, // TODO: don't rely on this.
    };
    const FILTER: egui::TextureOptions = egui::TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let pixels_bank0 = [0; TILE_DATA_FRAME_WIDTH * TILE_DATA_FRAME_HEIGHT * 4];

        let texture_bank0 = {
            let image = ColorImage::from_rgba_unmultiplied(
                [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
                &pixels_bank0,
            );

            egui_ctx.load_texture("tiles_bank0", image, Self::FILTER)
        };

        Self {
            opened: false,
            pixels_bank0,
            texture_bank0,
        }
    }

    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut egui::Ui) {
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
                let screen_size = ui.available_size();
                let screen_width = screen_size.x;
                let screen_height = screen_size.y;

                let texture_size = self.texture_bank0.size_vec2();
                let texture_width = texture_size.x;
                let texture_height = texture_size.y;

                let width_ratio = (screen_width / texture_width).max(1.0);
                let height_ratio = (screen_height / texture_height).max(1.0);

                let scale = width_ratio.clamp(1.0, height_ratio).floor();

                let scaled_width = texture_width * scale;
                let scaled_height = texture_height * scale;

                let size = Vec2 {
                    x: scaled_width,
                    y: scaled_height,
                };

                ui.centered_and_justified(|ui| {
                    ui.image(&self.texture_bank0, size);
                });
            });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx
            .cpu
            .memory
            .graphics
            .vram
            .draw_tile_data0_into_frame(&mut self.pixels_bank0);

        let image = ColorImage::from_rgba_unmultiplied(
            [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
            &self.pixels_bank0,
        );
        self.texture_bank0.set(image, Self::FILTER);
    }
}
