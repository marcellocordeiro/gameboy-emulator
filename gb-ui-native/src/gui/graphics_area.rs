use eframe::egui;
use egui::{
    epaint::{ColorImage, Vec2},
    Context,
};
use gb_core::{
    constants::{HEIGHT, WIDTH},
    GameBoy,
};

pub struct GraphicsArea {
    pixels: [u8; WIDTH * HEIGHT * 4],
    texture: egui::TextureHandle,
}

impl GraphicsArea {
    const FILTER: egui::TextureOptions = egui::TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let pixels = [0; WIDTH * HEIGHT * 4];

        let texture = {
            let image = ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &pixels);

            egui_ctx.load_texture("main", image, Self::FILTER)
        };

        Self { pixels, texture }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        self.update_texture(gb_ctx);

        let panel_frame = egui::Frame::default();

        egui::CentralPanel::default()
            .frame(panel_frame)
            .show(egui_ctx, |ui| {
                let screen_size = ui.available_size();
                let screen_width = screen_size.x;
                let screen_height = screen_size.y;

                let texture_size = self.texture.size_vec2();
                let texture_width = texture_size.x;
                let texture_height = texture_size.y;

                let width_ratio = (screen_width / texture_width).max(1.0);
                let height_ratio = (screen_height / texture_height).max(1.0);

                let scale = width_ratio.clamp(1.0, height_ratio).floor();

                let scaled_width = texture_width * scale;
                let scaled_height = texture_height * scale;

                ui.centered_and_justified(|ui| {
                    ui.add(egui::Image::new(
                        &self.texture,
                        Vec2 {
                            x: scaled_width,
                            y: scaled_height,
                        },
                    ));
                });
            });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx.draw(&mut self.pixels);

        let image = ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &self.pixels);
        self.texture.set(image, Self::FILTER);
    }
}
