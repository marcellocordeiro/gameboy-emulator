use egui::ViewportCommand;
use gb_core::{Button, GameBoy, EXTENSIONS, EXTENSIONS_DESCRIPTION};

use crate::{egui_framework::EguiUi, gui::Gui, key_mappings};

pub struct App {
    gb: GameBoy,
    rom_path: String,

    gui: Gui,
}

impl App {
    pub fn new(egui_ctx: &egui::Context, rom_path: Option<String>) -> Self {
        let mut gb = GameBoy::new();

        // Maybe let the UI handle the errors?
        let rom_path = {
            if let Some(path) = rom_path {
                path
            } else {
                let builder =
                    rfd::FileDialog::new().add_filter(EXTENSIONS_DESCRIPTION, &EXTENSIONS);
                let path = builder.pick_file().unwrap().to_str().unwrap().to_owned();

                path
            }
        };

        let rom = std::fs::read(&rom_path).unwrap();

        gb.load_cartridge(rom).unwrap();
        // load_battery(&mut gb, &rom_path);

        Self {
            gb,
            rom_path,
            gui: Gui::new(egui_ctx),
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
                    self.gb.joypad_button_down(button);
                } else if i.key_released(key) {
                    self.gb.joypad_button_up(button);
                }
            }
        });
    }

    pub fn update(&mut self) {
        if !self.gui.control.manual_control && self.gb.cartridge_inserted() {
            self.gb.run_frame();
        }
    }

    pub fn draw(&mut self, pixels: &mut [u8]) {
        self.gb.draw_into_frame_rgba8888(pixels.try_into().unwrap());
    }
}

impl EguiUi for App {
    fn ui(&mut self, egui_ctx: &egui::Context) {
        /*if !self.gui.control.manual_control && self.gb.cartridge_inserted() {
            self.gb.run_frame();
            egui_ctx.request_repaint();
        }*/

        self.handle_input(egui_ctx);
        self.gui.render(egui_ctx, &mut self.gb);
    }
}
