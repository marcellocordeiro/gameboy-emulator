use egui;
use gameboy_emulator_core::GameBoy;

pub struct Control {
    open: bool,
}

impl Control {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn toggle(&mut self) {
        self.open = !self.open;
    }

    pub fn draw(&mut self, egui_ctx: &egui::Context, gb_ctx: &mut GameBoy, manual_control: bool) {
        egui::Window::new("Control")
            .open(&mut self.open)
            .show(egui_ctx, |ui| {
                if manual_control {
                    if ui.button("Step").clicked() {
                        gb_ctx.cpu.step();
                    }
                } else if ui.add_enabled(false, egui::Button::new("Step")).clicked() {
                    unreachable!();
                }
            });
    }
}
