use eframe::egui;
use gb_core::{constants::Button, GameBoy};

use crate::{gui::Gui, key_mappings};

pub struct App {
    gb: GameBoy,
    gui: Gui,
}

impl App {
    pub fn new(cc: &eframe::CreationContext, rom: Vec<u8>) -> Self {
        let mut gb = GameBoy::new();
        gb.load_cartridge(rom).unwrap();

        cc.egui_ctx.set_pixels_per_point(1.0);

        Self {
            gb,
            gui: Gui::new(&cc.egui_ctx),
        }
    }

    fn handle_input(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.input(|i| {
            use egui::Key;

            if i.key_pressed(Key::Escape) {
                frame.close();
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);

        if !self.gui.control.manual_control {
            self.gb.run_frame();
            ctx.request_repaint();
        }

        self.handle_input(ctx, frame);

        self.gui.render(frame, ctx, &mut self.gb);
    }
}
