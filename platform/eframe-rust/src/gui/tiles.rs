use egui::{
    epaint::{ColorImage, TextureHandle, Vec2},
    Color32, Context, TextureOptions, Ui, Window,
};
use gb_core::{
    constants::{
        TileDataFrame, TILE_DATA_FRAME_HEIGHT, TILE_DATA_FRAME_SIZE, TILE_DATA_FRAME_WIDTH,
    },
    GameBoy,
};

pub struct Tiles {
    opened: bool,

    pixels: Box<TileDataFrame>,
    texture: TextureHandle,
}

impl Tiles {
    const DEFAULT_SIZE: Vec2 = Vec2 {
        x: TILE_DATA_FRAME_WIDTH as f32 * 2.0,
        y: TILE_DATA_FRAME_HEIGHT as f32 * 2.05, // TODO: don't rely on this.
    };
    const FILTER: TextureOptions = TextureOptions::NEAREST;

    pub fn new(egui_ctx: &Context) -> Self {
        let texture = {
            let image = ColorImage::new(
                [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
                Color32::WHITE,
            );

            egui_ctx.load_texture("tiles", image, Self::FILTER)
        };

        Self {
            opened: false,
            pixels: vec![0; TILE_DATA_FRAME_SIZE]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            texture,
        }
    }

    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("Toggle tiles").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !self.opened {
            return;
        }

        self.update_texture(gb_ctx);

        Window::new("Tiles")
            .open(&mut self.opened)
            .default_size(Self::DEFAULT_SIZE)
            .min_width(Self::DEFAULT_SIZE.x)
            .min_height(Self::DEFAULT_SIZE.y)
            .show(egui_ctx, |ui| {
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

                ui.centered_and_justified(|ui| {
                    ui.image(&self.texture, size);
                });
            });
    }

    fn update_texture(&mut self, gb_ctx: &GameBoy) {
        gb_ctx
            .cpu
            .memory
            .graphics
            .vram
            .draw_tile_data_0_into_frame(&mut self.pixels);

        #[cfg(feature = "cgb")]
        gb_ctx
            .cpu
            .memory
            .graphics
            .vram
            .draw_tile_data_1_into_frame(&mut self.pixels);

        let image = ColorImage::from_rgba_unmultiplied(
            [TILE_DATA_FRAME_WIDTH, TILE_DATA_FRAME_HEIGHT],
            self.pixels.as_ref(),
        );

        self.texture.set(image, Self::FILTER);
    }
}
