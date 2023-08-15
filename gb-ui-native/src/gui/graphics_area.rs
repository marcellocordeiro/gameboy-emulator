use eframe::egui;
use egui::Context;

pub fn draw(egui_ctx: &Context, texture: &egui::TextureHandle) {
    let panel_frame = egui::Frame::default();

    egui::CentralPanel::default()
        .frame(panel_frame)
        .show(egui_ctx, |ui| {
            let size = ui.available_size();

            ui.add(egui::Image::new(texture, size));
        });
}
