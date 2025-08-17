use egui::ViewportCommand;
use gb_core::{GameBoy, constants::DeviceModel, utils::button::Button};

use crate::{
    audio::Audio,
    file_manager::{FileInfo, FileManager},
    gui::{Event, Gui},
    key_mappings::EguiKeyMappings,
};

pub struct App {
    gb: GameBoy,
    file_manager: FileManager,

    audio: Option<Audio>,
    gui: Gui,
}

impl App {
    #[must_use]
    pub fn new(
        cc: &eframe::CreationContext,
        device_model: DeviceModel,
        file_manager: Option<FileManager>,
    ) -> Self {
        let mut app = Self {
            gb: GameBoy::new(device_model),
            file_manager: file_manager.unwrap_or_default(),
            audio: None,
            gui: Gui::new(&cc.egui_ctx),
        };

        if let Some(rom_file) = &app.file_manager.rom {
            app.load_from_file(rom_file.clone());
        }

        app
    }

    fn handle_events(&mut self, egui_ctx: &egui::Context) {
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

        while let Ok(event) = self.gui.event_receiver.try_recv() {
            match event {
                Event::BootromSelected(file) => {
                    self.file_manager.bootrom = Some(file);
                }

                Event::RomSelected(file) => self.load_from_file(file),
            }
        }
    }

    fn load_from_file(&mut self, file: FileInfo) {
        let bootrom = self.file_manager.bootrom.as_ref().map(|b| b.data.clone());

        let rom = file.data.clone();

        self.file_manager.rom = Some(file);

        let audio = Audio::new();

        self.gb.add_audio_callback(audio.get_callback());
        self.audio = Some(audio);

        self.gb.load(bootrom, rom).unwrap();
        self.file_manager.load_battery(&mut self.gb).unwrap();
    }
}

impl eframe::App for App {
    fn update(&mut self, egui_ctx: &egui::Context, _eframe_frame: &mut eframe::Frame) {
        if !self.gui.control.manual_control && self.gb.cartridge_inserted() {
            self.gb.run_frame();
            egui_ctx.request_repaint();
        }

        self.handle_events(egui_ctx);
        self.gui.render(egui_ctx, &mut self.gb);
    }

    fn on_exit(&mut self) {
        self.file_manager.save_battery(&self.gb).unwrap();
    }
}
