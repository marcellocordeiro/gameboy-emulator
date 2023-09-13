use eframe::egui;
use egui::{Context, FontId, RichText, Window};
use gb_core::GameBoy;

#[derive(Debug, Default)]
pub struct State {
    opened: bool,
}

impl State {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Toggle state").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !self.opened {
            return;
        }

        Window::new("State")
            .open(&mut self.opened)
            .show(egui_ctx, |ui| {
                ui.label(RichText::new(gb_ctx.cpu.to_string()).font(FontId::monospace(14.0)));
            });
    }
}
