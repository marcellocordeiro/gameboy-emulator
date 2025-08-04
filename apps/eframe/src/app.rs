use egui::ViewportCommand;
use gb_core::{
    GameBoy,
    constants::{DeviceModel, EXTENSIONS, EXTENSIONS_DESCRIPTION},
    utils::button::Button,
};
use thiserror::Error;

use crate::{
    audio::Audio,
    cartridge::{load_battery, save_battery},
    gui::Gui,
    key_mappings::EguiKeyMappings,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("No ROM selected")]
    NoRomSelected,
    #[error("Invalid path")]
    InvalidPath,
}

pub struct App {
    gb: GameBoy,
    rom_path: String,

    _audio: Audio,
    gui: Gui,
}

impl App {
    pub fn try_new(
        cc: &eframe::CreationContext,
        device_model: DeviceModel,
        bootrom_path: Option<String>,
        rom_path: Option<String>,
    ) -> Result<Self, Error> {
        // Maybe let the UI handle the errors?
        let rom_path = {
            if let Some(path) = rom_path {
                path
            } else {
                let builder =
                    rfd::FileDialog::new().add_filter(EXTENSIONS_DESCRIPTION, &EXTENSIONS);

                builder
                    .pick_file()
                    .ok_or(Error::NoRomSelected)?
                    .to_str()
                    .ok_or(Error::InvalidPath)?
                    .to_owned()
            }
        };

        let audio = Audio::new();

        let rom = std::fs::read(&rom_path).unwrap();
        let bootrom = bootrom_path.map(|path| std::fs::read(path).unwrap());

        let mut gb = GameBoy::new(device_model);
        gb.load(bootrom, rom).unwrap();
        load_battery(&mut gb, &rom_path);

        gb.add_audio_callback(audio.get_callback());

        Ok(Self {
            gb,
            rom_path,
            _audio: audio,
            gui: Gui::new(&cc.egui_ctx),
        })
    }

    fn handle_input(&mut self, egui_ctx: &egui::Context) {
        egui_ctx.input(|i| {
            use egui::Key;

            if i.key_pressed(Key::Escape) {
                egui_ctx.send_viewport_cmd(ViewportCommand::Close);
            }

            for button in Button::ALL_CASES {
                let key = button.mapped_to();

                if i.key_pressed(key) {
                    self.gb.joypad_button_down(button);
                } else if i.key_released(key) || !i.key_down(key) {
                    self.gb.joypad_button_up(button);
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

    fn on_exit(&mut self) {
        save_battery(&self.gb, &self.rom_path);
    }
}
