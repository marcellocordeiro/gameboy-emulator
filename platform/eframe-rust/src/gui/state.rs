use egui::{Context, FontId, RichText, Ui, Window};
use gb_core::GameBoy;

#[derive(Debug, Default)]
pub struct State {
    opened: bool,
}

impl State {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
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
                let ie_line = format!("IE: {:#04X}", gb_ctx.memory.read(0xFFFF));
                let if_line = format!("IF: {:#04X}", gb_ctx.memory.read(0xFF0F));

                let ime_line = format!("EI: {}", gb_ctx.cpu.registers.ime);

                let text = format!(
                    "{}\n\n{}\n\n{}\n{}",
                    gb_ctx.cpu.registers, ime_line, ie_line, if_line
                );

                ui.label(RichText::new(text).font(FontId::monospace(14.0)));
            });
    }
}
