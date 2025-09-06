use egui::{Context, FontId, RichText, Ui, Window};
use gb_core::{GameBoy, components::memory::MemoryInterface as _};

use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct State {
    opened: bool,
}

impl State {
    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut Ui) {
        if ui.button("State").clicked() {
            ctx.state.opened = !ctx.state.opened;
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn draw(ctx: &mut Gui, egui_ctx: &Context, gb_ctx: &GameBoy) {
        if !ctx.state.opened {
            return;
        }

        Window::new("State")
            .open(&mut ctx.state.opened)
            .show(egui_ctx, |ui| {
                let interrupts = gb_ctx.memory().interrupts();

                let registers = gb_ctx.cpu().registers();
                let ie = interrupts.read_enable();
                let r#if = interrupts.read_flags();

                let text = format!(
                    "\
                    {registers}\n\
                    \n\
                    IE: {ie:#04X}\n\
                    IF: {if:#04X}\
                    "
                );

                ui.label(RichText::new(text).font(FontId::monospace(14.0)));
            });
    }
}
