use app::App;
use clap::Parser;
use gb_core::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional ROM path (will show file picker if not provided)
    rom: Option<String>,

    /// (Unused) Set the device type to CGB
    #[arg(short, long, default_value_t = false)]
    cgb: bool,
}

fn main() -> Result<(), eframe::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let args = Args::parse();

    let rom_path = args.rom;

    #[allow(clippy::cast_precision_loss)]
    let initial_window_size = egui::vec2((SCREEN_WIDTH * 5) as f32, (SCREEN_HEIGHT * 5) as f32);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(initial_window_size),
        ..Default::default()
    };

    eframe::run_native(
        "gameboy-emulator",
        native_options,
        Box::new(move |cc| Box::new(App::new(cc, rom_path))),
    )
}

mod app;
mod cartridge;
mod gui;
mod key_mappings;
mod utils;
