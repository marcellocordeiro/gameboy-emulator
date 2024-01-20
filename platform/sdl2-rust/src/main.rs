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

    let rom_path = matches.get_one::<String>("rom").map(String::to_owned);

    let gb = GameBoy::new();

    App::new(gb, rom_path).run();
}

mod app;
mod key_mappings;
