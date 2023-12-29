use egui::ViewportCommand;
use gb_core::{Button, GameBoy};

use crate::{
    cartridge::{load_battery, save_battery},
    gui::Gui,
    key_mappings,
};

pub struct App {
    gb: GameBoy,
    rom_path: String,

    gui: Gui,
}

impl App {
    pub fn new(cc: &eframe::CreationContext, rom_path: Option<String>) -> Self {
        let mut gb = GameBoy::new();

        // Maybe let the UI handle the errors?
        let rom_path = {
            if let Some(path) = rom_path {
                path
            } else {
                let builder = rfd::FileDialog::new()
                    .add_filter("Game Boy/Game Boy Color ROM", &["gb", "gbc"]);
                let path = builder.pick_file().unwrap().to_str().unwrap().to_owned();

                path
            }
        };

        let rom = std::fs::read(&rom_path).unwrap();

        gb.load_cartridge(rom).unwrap();
        load_battery(&mut gb, &rom_path);

        Self {
            gb,
            rom_path,
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
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        save_battery(&mut self.gb, &self.rom_path);
    }

    fn update(&mut self, egui_ctx: &egui::Context, _eframe_frame: &mut eframe::Frame) {
        if !self.gui.control.manual_control && self.gb.cartridge_inserted() {
            self.gb.run_frame();
            egui_ctx.request_repaint();
        }

        self.handle_input(egui_ctx);
        self.gui.render(egui_ctx, &mut self.gb);
    }
}
