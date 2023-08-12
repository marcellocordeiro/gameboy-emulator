use egui::{Context, FontId, RichText, Window};
use gb_core::GameBoy;

pub struct State {
    open: bool,
}

impl State {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        Window::new("State")
            .open(&mut self.open)
            .show(egui_ctx, |ui| {
                ui.label(RichText::new(gb_ctx.cpu.to_string()).font(FontId::monospace(14.0)));
            });
    }
}
