use eframe::egui;
use gb_core::{constants::Button, GameBoy};

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

    fn handle_input(&mut self, eframe_frame: &mut eframe::Frame, egui_ctx: &egui::Context) {
        egui_ctx.input(|i| {
            use egui::Key;

            if i.key_pressed(Key::Escape) {
                eframe_frame.close();
            }

            for button in Button::to_array() {
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
    fn update(&mut self, egui_ctx: &egui::Context, eframe_frame: &mut eframe::Frame) {
        if !self.gui.control.manual_control && self.gb.cpu.memory.cartridge.is_some() {
            self.gb.run_frame();
            egui_ctx.request_repaint();
        }

        self.handle_input(eframe_frame, egui_ctx);

        self.gui.render(eframe_frame, egui_ctx, &mut self.gb);
    }
}
