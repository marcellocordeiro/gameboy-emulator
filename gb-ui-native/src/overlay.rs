use gb_core::GameBoy;

use crate::widgets::{control::Control, state::State};

/// Example application state. A real application will need a lot more state than this.
pub(crate) struct Overlay {
    /// Only show the egui window when true.
    pub manual_control: bool,

    state: State,
    control: Control,
}

impl Overlay {
    /// Create a `Gui`.
    pub(crate) fn new() -> Self {
        Self {
            manual_control: true,
            state: State::new(),
            control: Control::new(),
        }
    }

    /// Create the UI using egui.
    pub(crate) fn ui(&mut self, egui_ctx: &egui::Context, gb_ctx: &mut GameBoy) {
        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About...").clicked() {
                        // self.window_open = true;
                        ui.close_menu();
                    }

                    if ui.button("Quit").clicked() {
                        // frame.close();
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
                    self.control.toggle();
                }

                if ui.button("Toggle state").clicked() {
                    self.state.toggle();
                }
            });
        });

        self.control.draw(egui_ctx, gb_ctx, self.manual_control);
        self.state.draw(egui_ctx, gb_ctx);
    }
}
