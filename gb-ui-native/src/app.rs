use eframe::{egui, epaint::ColorImage};
use gb_core::{
    constants::{Button, HEIGHT, WIDTH},
    GameBoy,
};

use crate::{gui::Gui, key_mappings};

const FILTER: egui::TextureOptions = egui::TextureOptions::NEAREST;

pub struct App {
    gb: GameBoy,
    texture: egui::TextureHandle,
    pixels: [u8; WIDTH * HEIGHT * 4],

    gui: Gui,
}

impl App {
    pub fn new(cc: &eframe::CreationContext, rom: Vec<u8>) -> Self {
        let mut gb = GameBoy::new();
        gb.load_cartridge(rom).unwrap();

        let mut pixels = [0; WIDTH * HEIGHT * 4];

        let texture = {
            gb.draw(&mut pixels);

            let image = ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &pixels);

            cc.egui_ctx.load_texture("main", image, FILTER)
        };

        cc.egui_ctx.set_pixels_per_point(1.0);

        Self {
            gb,
            texture,
            pixels,

            gui: Gui::default(),
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

    fn update_texture(&mut self) {
        self.gb.draw(&mut self.pixels);

        let image = ColorImage::from_rgba_unmultiplied([WIDTH, HEIGHT], &self.pixels);
        self.texture.set(image, FILTER);
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);

        if !self.gui.manual_control {
            self.gb.run_frame();
            ctx.request_repaint();
        }

        self.update_texture();
        self.handle_input(ctx, frame);

        self.gui.render_ui(frame, ctx, &mut self.gb);
        self.gui.render_graphics_area(ctx, &self.texture);
    }
}
