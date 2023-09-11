use eframe::egui;
use egui::{menu, Context, TopBottomPanel};
use gb_core::GameBoy;

use self::{
    control::Control, file_loader::FileLoader, screen_area::ScreenArea, state::State, tiles::Tiles,
};

pub struct Gui {
    pub control: Control,
    pub file_loader: FileLoader,
    pub state: State,
    pub tiles: Tiles,
    pub screen_area: ScreenArea,
}

impl Gui {
    pub fn new(egui_ctx: &Context) -> Self {
        Self {
            control: Control::default(),
            file_loader: FileLoader,
            state: State::default(),
            tiles: Tiles::new(egui_ctx),
            screen_area: ScreenArea::new(egui_ctx),
        }
    }

    pub fn render(&mut self, frame: &mut eframe::Frame, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        self.render_ui(frame, egui_ctx, gb_ctx);
        self.render_screen_area(egui_ctx, gb_ctx);
    }

    fn render_ui(&mut self, frame: &mut eframe::Frame, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    self.file_loader.draw_button(ui, gb_ctx);

                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                self.control.draw_manual_control_button(ui);
                self.control.draw_widget_toggle_button(ui);
                self.state.draw_widget_toggle_button(ui);
                self.tiles.draw_widget_toggle_button(ui);
            });
        });

        self.control.draw(egui_ctx, gb_ctx);
        self.state.draw(egui_ctx, gb_ctx);
        self.tiles.draw(egui_ctx, gb_ctx);
    }

    fn render_screen_area(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        self.screen_area.draw(egui_ctx, gb_ctx);
    }
}

mod control;
mod file_loader;
mod screen_area;
mod state;
mod tiles;
