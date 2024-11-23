use app::App;
use clap::Parser;
use gb_core::{GameBoy, constants::DeviceModel};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the device model to CGB
    #[arg(short, long, default_value_t = false)]
    cgb: bool,

    /// Optional bootrom path
    #[arg(short, long)]
    bootrom: Option<String>,

    /// Optional ROM path (will show file picker if not provided)
    rom: Option<String>,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let args = Args::parse();

    let device_model = if args.cgb {
        DeviceModel::Cgb
    } else {
        DeviceModel::Dmg
    };
    let bootrom_path = args.bootrom;
    let rom_path = args.rom;
    let gb = GameBoy::new(device_model);

    App::new(gb, bootrom_path, rom_path).run();
}

mod app;
mod key_mappings;
