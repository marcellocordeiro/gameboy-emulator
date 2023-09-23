use egui::{Button, Context, Ui, Window};
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

    pub fn draw_manual_control_button(&mut self, ui: &mut Ui) {
        let text = if self.manual_control {
            "Manual"
        } else {
            "Auto"
        };

        if ui.button(text).clicked() {
            self.manual_control = !self.manual_control;
        }
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("Toggle control").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        if !self.opened {
            return;
        }

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

                if ui
                    .add_enabled(
                        self.manual_control && gb_ctx.cpu.memory.cartridge.is_some(),
                        Button::new("Run frame"),
                    )
                    .clicked()
                {
                    gb_ctx.run_frame();
                    egui_ctx.request_repaint();
                }
            });
    }
}
