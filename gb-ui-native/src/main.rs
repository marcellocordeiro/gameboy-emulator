#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::multiple_inherent_impl,
    clippy::cast_possible_truncation, // Might be nice to solve this one.
)]

use app::App;

fn get_rom(file: String) -> Vec<u8> {
    match std::fs::read(file) {
        Ok(bytes) => bytes,
        Err(e) => panic!("{}", e),
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let matches = clap::Command::new("gameboy-emulator")
        .arg(clap::Arg::new("rom").required(true))
        .get_matches();

    let rom_path = matches.get_one::<String>("rom").expect("required");
    let rom = get_rom(rom_path.to_string());

    let initial_window_size = eframe::egui::vec2(
        gb_core::constants::WIDTH as f32 * 3.0,
        gb_core::constants::WIDTH as f32 * 3.0,
    );

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(initial_window_size),
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "gameboy-emulator",
        native_options,
        Box::new(move |cc| Box::new(App::new(cc, rom))),
    )
}

mod app;
mod gui;
mod key_mappings;
