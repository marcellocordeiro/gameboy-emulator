use eframe::egui;
use egui::{Button, Context, Window};
use gb_core::GameBoy;

#[derive(Debug, Default)]
pub struct Control {
    opened: bool,

    pub manual_control: bool,
}

impl Control {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_manual_control_button(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(if self.manual_control {
                "Manual"
            } else {
                "Auto"
            })
            .clicked()
        {
            self.manual_control = !self.manual_control;
        }
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut egui::Ui) {
        if ui.button("Toggle control").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        Window::new("Control")
            .open(&mut self.opened)
            .show(egui_ctx, |ui| {
                if ui
                    .add_enabled(
                        self.manual_control && gb_ctx.cpu.memory.cartridge.is_some(),
                        Button::new("Step"),
                    )
                    .clicked()
                {
                    gb_ctx.cpu.step();
                    egui_ctx.request_repaint();
                }
            });
    }
}
