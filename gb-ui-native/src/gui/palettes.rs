use eframe::{egui, epaint::Color32};
use egui::{Context, Window};
use gb_core::GameBoy;

#[derive(Debug, Default)]
pub struct Palettes {
    opened: bool,
}

impl Palettes {
    pub fn toggle(&mut self) {
        self.opened = !self.opened;
    }

    pub fn draw_widget_toggle_button(&mut self, ui: &mut egui::Ui) {
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
                                for color in palette.chunks(2) {
                                    let rgb555 = {
                                        let lo = color[0] as u16;
                                        let hi = color[1] as u16;

                                        (hi << 8) | lo
                                    };

                                    let red = rgb555 & 0b1_1111;
                                    let green = (rgb555 >> 5) & 0b1_1111;
                                    let blue = (rgb555 >> 10) & 0b1_1111;

                                    let mut adjusted_red = red * 26 + green *  4 + blue *  2;
                                    let mut adjusted_green = green * 24 + blue *  8;
                                    let mut adjusted_blue = red *  6 + green *  4 + blue * 22;

                                    adjusted_red >>= 2;
                                    adjusted_green >>= 2;
                                    adjusted_blue >>= 2;

                                    let mut rgba = Color32::from_rgb(adjusted_red as u8, adjusted_green as u8, adjusted_blue as u8);

                                    ui.color_edit_button_srgba(&mut rgba).on_hover_text(format!(
                                        "RGB555: {rgb555:#06x}\n\nR: {red:#04x}\nG: {green:#04x}\nB: {blue:#04x}"
                                    ));
                                }
                            });
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        for palette in obj_palettes.chunks_exact(8) {
                            ui.horizontal(|ui| {
                                for color in palette.chunks(2) {
                                    let rgb555 = {
                                        let lo = color[0] as u16;
                                        let hi = color[1] as u16;

                                        (hi << 8) | lo
                                    };

                                    let red = rgb555 & 0b1_1111;
                                    let green = (rgb555 >> 5) & 0b1_1111;
                                    let blue = (rgb555 >> 10) & 0b1_1111;

                                    let mut adjusted_red = red * 26 + green *  4 + blue *  2;
                                    let mut adjusted_green = green * 24 + blue *  8;
                                    let mut adjusted_blue = red *  6 + green *  4 + blue * 22;

                                    adjusted_red >>= 2;
                                    adjusted_green >>= 2;
                                    adjusted_blue >>= 2;

                                    let mut rgba = Color32::from_rgb(adjusted_red as u8, adjusted_green as u8, adjusted_blue as u8);

                                    ui.color_edit_button_srgba(&mut rgba).on_hover_text(format!(
                                        "RGB555: {rgb555:#06x}\n\nR: {red:#04x}\nG: {green:#04x}\nB: {blue:#04x}"
                                    ));
                                }
                            });
                        }
                    });
                });
            });
    }
}
