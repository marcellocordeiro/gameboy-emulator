use egui::{Context, Modal, Ui};

#[derive(Debug, Default)]
pub struct About {
    opened: bool,
}

impl About {
    pub fn draw_widget_toggle_button(&mut self, ui: &mut Ui) {
        if ui.button("About").clicked() {
            self.opened = !self.opened;
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context) {
        if !self.opened {
            return;
        }

        let modal = Modal::new("about".into()).show(egui_ctx, |ui| {
            ui.heading("About");
            ui.label("About.");

            ui.hyperlink_to("Repository", env!("CARGO_PKG_REPOSITORY"));
        });

        if modal.should_close() {
            self.opened = false;
        }
    }
}
