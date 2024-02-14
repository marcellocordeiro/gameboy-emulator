use gb_core::{Button, GameBoy, ScreenPixels, SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH};
use key_mappings::LibretroKeyMappings;
use libretro_rs::{
    libretro_core,
    sys::RETRO_MEMORY_SAVE_RAM,
    RetroAudioInfo,
    RetroCore,
    RetroEnvironment,
    RetroGame,
    RetroLoadGameResult,
    RetroPixelFormat,
    RetroRuntime,
    RetroSystemInfo,
    RetroVideoInfo,
};

struct Emulator {
    gb: GameBoy,

    pixels: Box<ScreenPixels>,
}

impl RetroCore for Emulator {
    fn init(_env: &RetroEnvironment) -> Self {
        env_logger::init();

        Self {
            gb: GameBoy::default(),
            pixels: vec![0; SCREEN_PIXELS_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    fn get_system_info() -> RetroSystemInfo {
        RetroSystemInfo::new("gameboy-emulator", env!("CARGO_PKG_VERSION"))
            .with_valid_extensions(["gb", "gbc"].as_slice())
    }

    fn reset(&mut self, _env: &RetroEnvironment) {
        self.gb.reset();
    }

    fn run(&mut self, _env: &RetroEnvironment, runtime: &RetroRuntime) {
        for button in Button::ALL_CASES {
            let key = button.mapped_to();
            let value = runtime.is_joypad_button_pressed(0, key);

            self.gb.set_joypad_button(button, value);
        }

        self.gb.run_frame();
        self.gb.draw_into_frame_bgra8888(&mut self.pixels);

        runtime.upload_video_frame(
            self.pixels.as_ref(),
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
            SCREEN_WIDTH * 4,
        );
    }

    fn load_game(&mut self, _env: &RetroEnvironment, game: RetroGame) -> RetroLoadGameResult {
        let rom = match game {
            RetroGame::None { .. } => return RetroLoadGameResult::Failure,
            RetroGame::Data { data, .. } => data.to_vec(),
            RetroGame::Path { path, .. } => {
                let result = std::fs::read(path);

                match result {
                    Ok(rom) => rom,
                    Err(err) => {
                        log::error!("{err}");

                        return RetroLoadGameResult::Failure;
                    }
                }
            }
        };

        let result = self.gb.load_cartridge(rom);

        match result {
            Ok(()) => RetroLoadGameResult::Success {
                audio: RetroAudioInfo::new(44100.0),
                video: RetroVideoInfo::new(4_194_304.0 / 70224.0, 160, 144)
                    .with_pixel_format(RetroPixelFormat::XRGB8888),
            },
            Err(_) => RetroLoadGameResult::Failure,
        }
    }

    #[allow(clippy::as_ptr_cast_mut)]
    fn get_memory_data(&mut self, _env: &RetroEnvironment, id: u32) -> *mut () {
        match id {
            // This is horrible. Maybe try to find a better way.
            RETRO_MEMORY_SAVE_RAM => self
                .gb
                .get_battery()
                .map_or(std::ptr::null_mut(), |ram| ram.as_ptr() as *mut ()),

            _ => std::ptr::null_mut(),
        }
    }

    fn get_memory_size(&self, _env: &RetroEnvironment, id: u32) -> usize {
        match id {
            RETRO_MEMORY_SAVE_RAM => self.gb.get_battery().map_or(0, <[u8]>::len),

            _ => 0,
        }
    }
}

libretro_core!(Emulator);

mod key_mappings;
