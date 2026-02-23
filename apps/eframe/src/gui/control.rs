use std::sync::{Arc, Mutex};

use egui::{Context, Ui, Window};
use gb_core::GameBoy;

use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct Control {
    opened: bool,
    running: Arc<Mutex<bool>>,
}

impl Control {
    pub fn new(running: Arc<Mutex<bool>>) -> Self {
        Self {
            opened: false,
            running,
        }
    }

    pub fn draw_manual_control_button(ctx: &Gui, ui: &mut Ui) {
        let mut running = ctx.control.running.lock().unwrap();

        let text = if *running { "Auto" } else { "Manual" };

        if ui.button(text).clicked() {
            *running = !*running;
        }
    }

    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut Ui) {
        if ui.button("Control").clicked() {
            ctx.control.opened = !ctx.control.opened;
        }
    }

    pub fn draw(ctx: &mut Gui, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        if !ctx.control.opened {
            return;
        }

        Window::new("Control")
            .open(&mut ctx.control.opened)
            .show(egui_ctx, |ui| {
                let enable_buttons =
                    *ctx.control.running.lock().unwrap() && gb_ctx.cartridge_inserted();

                ui.add_enabled_ui(enable_buttons, |ui| {
                    if ui.button("Step").clicked() {
                        gb_ctx.step();
                    }

                    if ui.button("Run frame").clicked() {
                        gb_ctx.run_frame();
                    }
                });
            });
    }
}
