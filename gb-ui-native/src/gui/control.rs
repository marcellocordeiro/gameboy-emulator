use eframe::egui;
use egui::{Button, Context, Window};
use gb_core::GameBoy;

pub fn draw(
    egui_ctx: &Context,
    gb_ctx: &mut GameBoy,
    opened: &mut bool,
    manual_control: &mut bool,
) {
    Window::new("Control").open(opened).show(egui_ctx, |ui| {
        if ui
            .add_enabled(*manual_control, Button::new("Step"))
            .clicked()
        {
            gb_ctx.cpu.step();
            egui_ctx.request_repaint();
        }
    });
}
