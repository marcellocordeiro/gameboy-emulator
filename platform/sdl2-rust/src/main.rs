use app::App;
use clap::Parser;
use gb_core::GameBoy;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Optional ROM path (will show file picker if not provided)
    rom: Option<String>,

    /// (Unused) Set the device type to CGB
    #[arg(short, long, default_value_t = false)]
    cgb: bool,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let args = Args::parse();

    let rom_path = args.rom;
    let gb = GameBoy::new();

    App::new(gb, rom_path).run();
}

mod app;
mod key_mappings;
