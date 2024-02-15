use egui::{menu, Context, TopBottomPanel, ViewportCommand};
use gb_core::GameBoy;

use self::{control::Control, palettes::Palettes, state::State, tiles::Tiles};

pub struct Gui {
    pub control: Control,
    pub palettes: Palettes,
    pub state: State,
    pub tiles: Tiles,
}

impl Gui {
    pub fn new(egui_ctx: &Context) -> Self {
        Self {
            control: Control::default(),
            palettes: Palettes::default(),
            state: State::default(),
            tiles: Tiles::new(egui_ctx),
        }
    }

    pub fn render(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        self.render_ui(egui_ctx, gb_ctx);
    }

    fn render_ui(&mut self, egui_ctx: &Context, gb_ctx: &mut GameBoy) {
        TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            menu::bar(ui, |ui| {
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
            });
        });

        self.control.draw(egui_ctx, gb_ctx);
        self.state.draw(egui_ctx, gb_ctx);
        self.tiles.draw(egui_ctx, gb_ctx);
        self.palettes.draw(egui_ctx, gb_ctx);
    }
}

mod components;
mod control;
mod palettes;
mod state;
mod tiles;

/*
/// Example application state. A real application will need a lot more state than this.
pub struct Gui {
    /// Only show the egui window when true.
    window_open: bool,
}

impl Gui {
    /// Create a `Gui`.
    pub fn new() -> Self {
        Self { window_open: true }
    }
}

impl EguiUi for Gui {
    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About...").clicked() {
                        self.window_open = true;
                        ui.close_menu();
                    }
                })
            });
        });

        egui::Window::new("Hello, egui!")
            .open(&mut self.window_open)
            .show(ctx, |ui| {
                ui.label("This example demonstrates using egui with pixels.");
                ui.label("Made with 💖 in San Francisco!");

                ui.separator();

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x /= 2.0;
                    ui.label("Learn more about egui at");
                    ui.hyperlink("https://docs.rs/egui");
                });
            });
    }
}*/
