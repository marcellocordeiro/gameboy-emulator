use egui::{Context, Ui, Window};
use gb_core::{
    GameBoy,
    components::{apu::Channels, memory::MemoryInterface},
};

use crate::gui::Gui;

#[derive(Debug, Default)]
pub struct Audio {
    opened: bool,
}

impl Audio {
    pub fn draw_widget_toggle_button(ctx: &mut Gui, ui: &mut Ui) {
        if ui.button("Audio").clicked() {
            ctx.audio.opened = !ctx.audio.opened;
        }
    }

    pub fn draw(ctx: &mut Gui, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        if !ctx.audio.opened {
            return;
        }

        Window::new("Audio")
            .open(&mut ctx.audio.opened)
            .show(egui_ctx, |ui| {
                let apu = gb_ctx.memory_mut().apu_mut();
                let channels = &mut apu.ui_channel_overrides;

                for (index, flag) in Channels::all().iter().enumerate() {
                    let current_value = channels.contains(flag);
                    let mut value = current_value;

                    ui.checkbox(&mut value, format!("Channel {}", index + 1));

                    if value != current_value {
                        channels.set(flag, value);
                    }
                }
            });
    }
}
