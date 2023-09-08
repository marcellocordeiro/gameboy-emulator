use eframe::egui;
use egui::Context;
use gb_core::GameBoy;

use self::{control::Control, graphics_area::GraphicsArea, state::State};

pub struct Gui {
    pub control: Control,
    pub state: State,
    pub graphics_area: GraphicsArea,
}

impl Gui {
    pub fn new(egui_ctx: &Context) -> Self {
        Self {
            control: Control::default(),
            state: State::default(),
            graphics_area: GraphicsArea::new(egui_ctx),
        }
    }

    pub fn render(&mut self, frame: &mut eframe::Frame, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        self.render_ui(frame, egui_ctx, gb_ctx);
        self.render_graphics_area(egui_ctx, gb_ctx);
    }

    fn render_ui(&mut self, frame: &mut eframe::Frame, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                self.control.draw_manual_control_button(ui);
                self.control.draw_widget_toggle_button(ui);
                self.state.draw_widget_toggle_button(ui);
            });
        });

        self.control.draw(egui_ctx, gb_ctx);
        self.state.draw(egui_ctx, gb_ctx);
    }

    fn render_graphics_area(&mut self, egui_ctx: &Context, gb_ctx: &GameBoy) {
        self.graphics_area.draw(egui_ctx, gb_ctx);
    }
}

mod control;
mod graphics_area;
mod state;
