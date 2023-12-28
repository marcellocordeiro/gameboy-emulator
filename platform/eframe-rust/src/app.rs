use egui::ViewportCommand;
use gb_core::{utils::button::Button, GameBoy};

use crate::{gui::Gui, key_mappings};

pub struct App {
    gb: GameBoy,
    gui: Gui,
}

impl App {
    pub fn new(cc: &eframe::CreationContext, gb: GameBoy) -> Self {
        Self {
            gb,
            gui: Gui::new(&cc.egui_ctx),
        }
    }

    fn handle_input(&mut self, egui_ctx: &egui::Context) {
        egui_ctx.input(|i| {
            use egui::Key;

            if i.key_pressed(Key::Escape) {
                egui_ctx.send_viewport_cmd(ViewportCommand::Close);
            }

            for button in Button::ALL_CASES {
                let key = key_mappings::map_button(button);

                if i.key_pressed(key) {
                    self.gb.key_down(button);
                } else if i.key_released(key) {
                    self.gb.key_up(button);
                }
            }
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, egui_ctx: &egui::Context, _eframe_frame: &mut eframe::Frame) {
        if !self.gui.control.manual_control && self.gb.cartridge_inserted() {
            self.gb.run_frame();
            egui_ctx.request_repaint();
        }

        self.handle_input(egui_ctx);
        self.gui.render(egui_ctx, &mut self.gb);
    }
}
