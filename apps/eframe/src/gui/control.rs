use egui::{Button, Context, Ui, Window};
use gb_core::GameBoy;

use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct Control {
    opened: bool,

    pub manual_control: bool,
}

impl Control {
    pub fn draw_manual_control_button(ctx: &mut Gui, ui: &mut Ui) {
        let text = if ctx.control.manual_control {
            "Manual"
        } else {
            "Auto"
        };

        if ui.button(text).clicked() {
            ctx.control.manual_control = !ctx.control.manual_control;
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
                let enable_buttons = ctx.control.manual_control && gb_ctx.cartridge_inserted();

                if ui
                    .add_enabled(enable_buttons, Button::new("Step"))
                    .clicked()
                {
                    gb_ctx.step();
                    egui_ctx.request_repaint();
                }

                if ui
                    .add_enabled(enable_buttons, Button::new("Run frame"))
                    .clicked()
                {
                    gb_ctx.run_frame();
                    egui_ctx.request_repaint();
                }
            });
    }
}
