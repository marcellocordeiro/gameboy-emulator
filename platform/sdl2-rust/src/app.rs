use std::time::{Duration, Instant};

use gb_core::{
    GameBoy,
    ScreenPixels,
    EXTENSIONS,
    EXTENSIONS_DESCRIPTION,
    SCREEN_HEIGHT,
    SCREEN_PIXELS_SIZE,
    SCREEN_WIDTH,
};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, render::Texture};

use crate::key_mappings;

pub struct App {
    gb: GameBoy,

    bootrom_path: Option<String>,
    rom_path: Option<String>,

    pixels: Box<ScreenPixels>,
}

impl App {
    pub fn new(gb: GameBoy, bootrom_path: Option<String>, rom_path: Option<String>) -> Self {
        Self {
            gb,
            bootrom_path,
            rom_path,
            pixels: vec![0; SCREEN_PIXELS_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    pub fn run(&mut self) {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                "gameboy-emulator",
                (SCREEN_WIDTH * 5) as u32,
                (SCREEN_HEIGHT * 5) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().present_vsync().build().unwrap();

        let texture_creator = canvas.texture_creator();
        let mut texture = texture_creator
            .create_texture_streaming(PixelFormatEnum::ABGR8888, 160, 144)
            .unwrap();

        canvas.clear();

        // Maybe let the UI handle the errors?
        let rom_path = {
            if let Some(path) = self.rom_path.clone() {
                path
            } else {
                let builder =
                    rfd::FileDialog::new().add_filter(EXTENSIONS_DESCRIPTION, &EXTENSIONS);
                let path = builder.pick_file().unwrap().to_str().unwrap().to_owned();

                path
            }
        };

        let rom = std::fs::read(rom_path).unwrap();
        let bootrom = self
            .bootrom_path
            .as_ref()
            .map(|path| std::fs::read(path).unwrap());

        self.gb.load(rom, bootrom).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut current_timer = Instant::now();
        let mut elapsed_frames = 0;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,

                    Event::KeyDown {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => {
                        let button = key_mappings::map_keycode(keycode);

                        if let Some(button) = button {
                            self.gb.joypad_button_down(button);
                        }
                    }

                    Event::KeyUp {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => {
                        let button = key_mappings::map_keycode(keycode);

                        if let Some(button) = button {
                            self.gb.joypad_button_up(button);
                        }
                    }

                    _ => {}
                }
            }

            self.gb.run_frame();
            self.update_tex(&mut texture);

            let elapsed = current_timer.elapsed();

            if elapsed > Duration::from_secs(1) {
                let fps = (elapsed_frames as f64) / elapsed.as_secs_f64();
                let title = format!("gameboy-emulator | {fps:5.2}fps");

                let window = canvas.window_mut();
                window.set_title(&title).unwrap();

                current_timer = Instant::now();
                elapsed_frames = 0;
            }

            elapsed_frames += 1;

            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    }

    fn update_tex(&mut self, texture: &mut Texture) {
        self.gb.draw_into_frame_rgba8888(&mut self.pixels);

        texture.update(None, self.pixels.as_mut(), 160 * 4).unwrap();
    }
}
