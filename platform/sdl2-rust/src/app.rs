use gb_core::{
    constants::{Frame, FRAME_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};
use sdl2::{event::Event, keyboard::Keycode, pixels::PixelFormatEnum, render::Texture};

use crate::key_mappings;

pub struct App {
    gb: GameBoy,

    pixels: Box<Frame>,
}

impl App {
    pub fn new(gb: GameBoy) -> Self {
        Self {
            gb,
            pixels: vec![0; FRAME_SIZE].into_boxed_slice().try_into().unwrap(),
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

        let mut event_pump = sdl_context.event_pump().unwrap();

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
                            self.gb.key_down(button);
                        }
                    }

                    Event::KeyUp {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => {
                        let button = key_mappings::map_keycode(keycode);

                        if let Some(button) = button {
                            self.gb.key_up(button);
                        }
                    }

                    _ => {}
                }
            }

            self.gb.run_frame();
            self.update_tex(&mut texture);

            canvas.copy(&texture, None, None).unwrap();
            canvas.present();
        }
    }

    fn update_tex(&mut self, texture: &mut Texture) {
        self.gb.draw_into_frame_rgba8888(&mut self.pixels);

        texture.update(None, self.pixels.as_mut(), 160 * 4).unwrap();
    }
}
