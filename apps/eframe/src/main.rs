use app::App;
use cli::parse_args;
use gb_core::constants::{DeviceModel, SCREEN_HEIGHT, SCREEN_WIDTH};

fn main() -> Result<(), eframe::Error> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .init();

    let args = parse_args();

    let device_model = if args.dmg {
        DeviceModel::Dmg
    } else {
        DeviceModel::Cgb
    };
    let bootrom_path = args.bootrom;
    let rom_path = args.rom;

    #[allow(clippy::cast_precision_loss)]
    let initial_window_size = egui::vec2((SCREEN_WIDTH * 5) as f32, (SCREEN_HEIGHT * 5) as f32);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(initial_window_size),
        ..Default::default()
    };

    eframe::run_native(
        "Game Boy",
        native_options,
        Box::new(move |cc| {
            Ok(Box::new(App::try_new(
                cc,
                device_model,
                bootrom_path,
                rom_path,
            )?))
        }),
    )
}

mod app;
mod audio;
mod cartridge;
mod cli;
mod gui;
mod key_mappings;
mod utils;
