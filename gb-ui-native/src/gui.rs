use eframe::egui;
use egui::Context;
use gb_core::GameBoy;

#[derive(Default)]
pub struct Gui {
    pub show_state: bool,
    pub show_controls: bool,
    pub manual_control: bool,
}

impl Gui {
    pub fn render_ui(
        &mut self,
        frame: &mut eframe::Frame,
        egui_ctx: &Context,
        gb_ctx: &mut GameBoy,
    ) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                if ui
                    .button(if self.manual_control {
                        "Manual"
                    } else {
                        "Auto"
                    })
                    .clicked()
                {
                    self.manual_control = !self.manual_control;
                }

                if ui.button("Toggle control").clicked() {
                    self.show_controls = !self.show_controls;
                }

                if ui.button("Toggle state").clicked() {
                    self.show_state = !self.show_state;
                }
            });
        });

        state::draw(egui_ctx, gb_ctx, &mut self.show_state);
        control::draw(
            egui_ctx,
            gb_ctx,
            &mut self.show_controls,
            &mut self.manual_control,
        );
    }

    pub fn render_graphics_area(&mut self, egui_ctx: &Context, texture: &egui::TextureHandle) {
        graphics_area::draw(egui_ctx, texture);
    }
}

pub mod control;
mod graphics_area;
pub mod state;
