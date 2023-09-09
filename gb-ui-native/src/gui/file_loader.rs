use eframe::egui;
use gb_core::GameBoy;

#[derive(Debug)]
pub struct FileLoader;

impl FileLoader {
    pub fn draw_button(&mut self, ui: &mut egui::Ui, gb_ctx: &mut GameBoy) {
        if ui.button("Load...").clicked() {
            let builder = rfd::FileDialog::new().add_filter("", &["gb", "gbc"]);

            if let Some(path) = builder.pick_file() {
                let rom = std::fs::read(path).unwrap();
                gb_ctx.load_cartridge(rom).unwrap();
            }
        }
    }
}
