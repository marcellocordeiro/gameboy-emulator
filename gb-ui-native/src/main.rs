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

fn get_rom(file: String) -> Vec<u8> {
    std::fs::read(file).unwrap()
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
