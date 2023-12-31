use app::App;
use gb_core::GameBoy;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

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
        let builder =
            rfd::FileDialog::new().add_filter("Game Boy/Game Boy Color ROM", &["gb", "gbc"]);
        let path = builder.pick_file().unwrap();

        let rom = std::fs::read(path).unwrap();
        gb.load_cartridge(rom).unwrap();
    }

    App::new(gb).run();
}

mod app;
mod key_mappings;
