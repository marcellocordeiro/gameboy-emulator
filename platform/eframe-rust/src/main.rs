#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use app::App;
use gb_core::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let matches = clap::Command::new("gameboy-emulator")
        .arg(clap::Arg::new("rom"))
        .get_matches();

    let rom_path = matches.get_one::<String>("rom");

    let mut gb = GameBoy::new();

    // Maybe let the UI handle the errors?
    if let Some(path) = rom_path {
        let rom = std::fs::read(path).unwrap();
        gb.load_cartridge(rom).unwrap();
    } else {
        let builder = rfd::FileDialog::new().add_filter("GB/GBC ROM", &["gb", "gbc"]);
        let path = builder.pick_file().unwrap();

        let rom = std::fs::read(path).unwrap();
        gb.load_cartridge(rom).unwrap();
    }

    let initial_window_size =
        eframe::egui::vec2(SCREEN_WIDTH as f32 * 5.0, SCREEN_HEIGHT as f32 * 5.0);

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(initial_window_size),
        ..Default::default()
    };

    eframe::run_native(
        "gameboy-emulator",
        native_options,
        Box::new(move |cc| Box::new(App::new(cc, gb))),
    )
}

mod app;
mod gui;
mod key_mappings;
