use egui::{Button, Id, Key, Modal, RichText};

use crate::gui::Gui;

#[derive(Debug)]
pub struct BrowserFirstClick {
    pub opened: bool,
}

impl Default for BrowserFirstClick {
    fn default() -> Self {
        Self { opened: true }
    }
}

impl BrowserFirstClick {
    pub fn draw(ctx: &mut Gui, ui: &egui::Ui) {
        if !ctx.browser_first_click.opened {
            return;
        }

        Modal::new(Id::unique("browser_first_click")).show(ui, |ui| {
            ui.set_width(340.0);

            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(
                        "Would you kindly tap the OK button so we're able to play audio? This is required by current browser laws.",
                    )
                    .size(14.0)
                );

                ui.add_space(10.0);

                let button = Button::new(RichText::new("OK").size(16.0));

                let response = ui.add(button);
                response.request_focus();

                if response.clicked() || ui.input(|i| i.key_pressed(Key::Enter)) {
                    ctx.browser_first_click.opened = false;
                }
            });
        });
    }
}
