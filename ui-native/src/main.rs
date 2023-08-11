#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::multiple_inherent_impl,
    clippy::cast_possible_truncation, // Might be nice to solve this one.
)]

fn get_rom(file: String) -> Vec<u8> {
    match std::fs::read(file) {
        Ok(bytes) => bytes,
        Err(e) => panic!("{}", e),
    }
}

fn main() -> Result<(), pixels::Error> {
    let matches = clap::Command::new("gameboy-emulator")
        .arg(clap::Arg::new("rom").required(true))
        .get_matches();

    let rom_path = matches.get_one::<String>("rom").expect("required");
    let rom = get_rom(rom_path.to_string());

    app::run(rom)
}

mod app;
mod overlay;
mod overlay_framework;
mod widgets;
