use eframe::{egui, epaint::Vec2};
use egui::Context;

pub fn draw(egui_ctx: &Context, texture: &egui::TextureHandle) {
    let panel_frame = egui::Frame::default();

    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(egui_ctx, |ui| {
            let screen_size = ui.available_size();
            let screen_width = screen_size.x;
            let screen_height = screen_size.y;

            let texture_size = texture.size_vec2();
            let texture_width = texture_size.x;
            let texture_height = texture_size.y;

            let width_ratio = (screen_width / texture_width).max(1.0);
            let height_ratio = (screen_height / texture_height).max(1.0);

            let scale = width_ratio.clamp(1.0, height_ratio).floor();

            let scaled_width = texture_width * scale;
            let scaled_height = texture_height * scale;

            ui.centered_and_justified(|ui| {
                ui.add(egui::Image::new(
                    texture,
                    Vec2 {
                        x: scaled_width,
                        y: scaled_height,
                    },
                ));
            });
        });
}
