use egui::{Context, Ui, Window, epaint::Color32};
use gb_core::{GameBoy, utils::color::Color};

use super::components::color_rect::color_rect;
use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct Palettes {
    opened: bool,
}

impl Palettes {
    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut Ui) {
        if ui.button("Palettes").clicked() {
            ctx.palettes.opened = !ctx.palettes.opened;
        }
    }

    pub fn draw(ctx: &mut Gui, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !ctx.palettes.opened {
            return;
        }

        Window::new("Palettes")
            .open(&mut ctx.palettes.opened)
            .show(egui_ctx, |ui| {
                let bg_palettes = &gb_ctx.memory().ppu.bg_cram;
                let obj_palettes = &gb_ctx.memory().ppu.obj_cram;

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for palette_number in 0..8 {
                            ui.horizontal(|ui| {
                                for color_id in 0..4 {
                                    let raw_color = bg_palettes.get_color_rgb555(palette_number, color_id);

                                    let raw_pixel = Color::from_rgb555(raw_color);
                                    let pixel = Color::from_rgb555_accurate(raw_color);

                                    let rgb = Color32::from_rgb(pixel.red, pixel.green, pixel.blue);
                                    let tooltip = format!(
                                        "RGB555: {raw_color:#06x}\n\nR: {:#04x}\nG: {:#04x}\nB: {:#04x}", raw_pixel.red, raw_pixel.green, raw_pixel.blue
                                    );

                                    color_rect(ui, rgb).on_hover_text(tooltip);
                                }
                            });
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        for palette_number in 0..8 {
                            ui.horizontal(|ui| {
                                for color_id in 0..4 {
                                    let raw_color = obj_palettes.get_color_rgb555(palette_number, color_id);

                                    let raw_pixel = Color::from_rgb555(raw_color);
                                    let pixel = Color::from_rgb555_accurate(raw_color);

                                    let rgb = Color32::from_rgb(pixel.red, pixel.green, pixel.blue);
                                    let tooltip = format!(
                                        "RGB555: {raw_color:#06x}\n\nR: {:#04x}\nG: {:#04x}\nB: {:#04x}", raw_pixel.red, raw_pixel.green, raw_pixel.blue
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
