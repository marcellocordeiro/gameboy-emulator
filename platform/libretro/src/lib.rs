use gb_core::{
    constants::{Frame, FRAME_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};
use libretro_rs::*;

struct Emulator {
    gb: GameBoy,

    pixels: Box<Frame>,
}

impl RetroCore for Emulator {
    fn init(_env: &RetroEnvironment) -> Self {
        env_logger::init();

        Self {
            gb: GameBoy::default(),
            pixels: vec![0; FRAME_SIZE].into_boxed_slice().try_into().unwrap(),
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
        self.gb.run_frame();
        self.gb.draw(&mut self.pixels);

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
                    Ok(rom) => rom.to_vec(),
                    Err(err) => {
                        log::error!("{err}");

                        return RetroLoadGameResult::Failure;
                    }
                }
            }
        };

        let result = self.gb.load_cartridge(rom);

        match result {
            Ok(_) => RetroLoadGameResult::Success {
                audio: RetroAudioInfo::new(44100.0),
                video: RetroVideoInfo::new(4194304.0 / 70224.0, 160, 144)
                    .with_pixel_format(RetroPixelFormat::XRGB8888),
            },
            Err(_) => RetroLoadGameResult::Failure,
        }
    }
}

libretro_core!(Emulator);