use egui::{
    epaint::{ColorImage, TextureHandle, Vec2},
    CentralPanel, Color32, Context, TextureOptions,
};
use gb_core::{
    constants::{Frame, FRAME_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

pub struct ScreenArea {
    pixels: Box<Frame>,
    texture: TextureHandle,
}

impl ScreenArea {
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let texture = {
            let image = ColorImage::new([SCREEN_WIDTH, SCREEN_HEIGHT], Color32::WHITE);

            egui_ctx.load_texture("main", image, Self::FILTER)
        };

        Self {
            pixels: vec![0; FRAME_SIZE].into_boxed_slice().try_into().unwrap(),
            texture,
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        self.update_texture(gb_ctx);

        CentralPanel::default().show(egui_ctx, |ui| {
            ui.centered_and_justified(|ui| {
                let screen_size = ui.available_size();
                let screen_width = screen_size.x;
                let screen_height = screen_size.y;

                let texture_size = self.texture.size_vec2();
                let texture_width = texture_size.x;
                let texture_height = texture_size.y;

                let width_ratio = (screen_width / texture_width).max(1.0);
                let height_ratio = (screen_height / texture_height).max(1.0);

                let scale = width_ratio.clamp(1.0, height_ratio).floor();

                let scaled_width = texture_width * scale;
                let scaled_height = texture_height * scale;

                let size = Vec2::new(scaled_width, scaled_height);

                ui.image(&self.texture, size);
            });
        });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx.draw_into_frame_rgba8888(&mut self.pixels);

        let image =
            ColorImage::from_rgba_unmultiplied([SCREEN_WIDTH, SCREEN_HEIGHT], self.pixels.as_ref());
        self.texture.set(image, Self::FILTER);
    }
}
