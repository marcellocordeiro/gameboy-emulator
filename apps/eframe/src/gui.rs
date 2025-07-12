use egui::{Context, MenuBar, TopBottomPanel, ViewportCommand};
use gb_core::GameBoy;

use self::{
    control::Control,
    palettes::Palettes,
    screen_area::ScreenArea,
    state::State,
    tiles::Tiles,
};
use crate::gui::audio::Audio;

pub struct Gui {
    pub audio: Audio,
    pub control: Control,
    pub palettes: Palettes,
    pub state: State,
    pub tiles: Tiles,
    pub screen_area: ScreenArea,
}

impl Gui {
    pub fn new(egui_ctx: &Context) -> Self {
        Self {
            audio: Audio::default(),
            control: Control::default(),
            palettes: Palettes::default(),
            state: State::default(),
            tiles: Tiles::new(egui_ctx),
            screen_area: ScreenArea::new(egui_ctx),
        }
    }

    pub fn render(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        self.render_ui(egui_ctx, gb_ctx);
        self.render_screen_area(egui_ctx, gb_ctx);
    }

    fn render_ui(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Reset").clicked() {
                        gb_ctx.reset();
                    }

                    if ui.button("Quit").clicked() {
                        egui_ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                self.control.draw_manual_control_button(ui);
                self.control.draw_widget_toggle_button(ui);
                self.state.draw_widget_toggle_button(ui);
                self.tiles.draw_widget_toggle_button(ui);
                self.palettes.draw_widget_toggle_button(ui);
                self.audio.draw_widget_toggle_button(ui);
            });
        });

        self.control.draw(egui_ctx, gb_ctx);
        self.state.draw(egui_ctx, gb_ctx);
        self.tiles.draw(egui_ctx, gb_ctx);
        self.palettes.draw(egui_ctx, gb_ctx);
        self.audio.draw(egui_ctx, gb_ctx);
    }

    fn render_screen_area(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        self.screen_area.draw(egui_ctx, gb_ctx);
    }
}

mod audio;
mod components;
mod control;
mod palettes;
mod screen_area;
mod state;
mod tiles;
