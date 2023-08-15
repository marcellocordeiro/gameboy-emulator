use eframe::egui;
use egui::{Context, FontId, RichText, Window};
use gb_core::GameBoy;

pub fn draw(egui_ctx: &Context, gb_ctx: &GameBoy, opened: &mut bool) {
    Window::new("State").open(opened).show(egui_ctx, |ui| {
        ui.label(RichText::new(gb_ctx.cpu.to_string()).font(FontId::monospace(14.0)));
    });
}
