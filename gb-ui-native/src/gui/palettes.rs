use eframe::egui;
use egui::{epaint::Color32, Context, Ui, Window};
use gb_core::{utils::color::Color, GameBoy};

use super::components::color_rect::color_rect;

#[derive(Debug, Default)]
pub struct Palettes {
    opened: bool,
}

impl Palettes {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("Toggle palettes").clicked() {
            self.toggle();
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !self.opened {
            return;
        }

        let bg_palettes = gb_ctx.cpu.memory.graphics.get_bg_palette_ram();
        let obj_palettes = gb_ctx.cpu.memory.graphics.get_obj_palette_ram();

        Window::new("Palettes")
            .open(&mut self.opened)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for palette in bg_palettes.chunks_exact(8) {
                            ui.horizontal(|ui| {
                                for color_bytes in palette.chunks(2) {
                                    let rgb555 = {
                                        let lo = color_bytes[0] as u16;
                                        let hi = color_bytes[1] as u16;

                                        (hi << 8) | lo
                                    };

                                    let raw_pixel = Color::from_rgb555_u16_raw(rgb555);
                                    let pixel = Color::from_rgb555_u16_to_rgba8888(rgb555);

                                    let rgb = Color32::from_rgb(pixel.red, pixel.green, pixel.blue);
                                    let tooltip = format!(
                                        "RGB555: {rgb555:#06x}\n\nR: {:#04x}\nG: {:#04x}\nB: {:#04x}", raw_pixel.red, raw_pixel.green, raw_pixel.blue
                                    );

                                    color_rect(ui, rgb).on_hover_text(tooltip);
                                }
                            });
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        for palette in obj_palettes.chunks_exact(8) {
                            ui.horizontal(|ui| {
                                for color_bytes in palette.chunks(2) {
                                    let rgb555 = {
                                        let lo = color_bytes[0] as u16;
                                        let hi = color_bytes[1] as u16;

                                        (hi << 8) | lo
                                    };

                                    let raw_pixel = Color::from_rgb555_u16_raw(rgb555);
                                    let pixel = Color::from_rgb555_u16_to_rgba8888(rgb555);

                                    let rgb = Color32::from_rgb(pixel.red, pixel.green, pixel.blue);
                                    let tooltip = format!(
                                        "RGB555: {rgb555:#06x}\n\nR: {:#04x}\nG: {:#04x}\nB: {:#04x}", raw_pixel.red, raw_pixel.green, raw_pixel.blue
                                    );

                                    color_rect(ui, rgb).on_hover_text(tooltip);
                                }
                            });
                        }
                    });
                });
            });
    }
}
