use gb_core::constants::DeviceModel;

use crate::app::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use std::path::PathBuf;

    use cli::parse_args;
    use gb_core::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};

    use crate::file_manager::{FileInfo, FileManager};

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

    let [bootrom, rom] = [bootrom_path, rom_path]
        .map(|path| path.map(PathBuf::from))
        .map(|path| {
            if let Some(path) = path {
                let data = std::fs::read(&path).unwrap();
                Some(FileInfo {
                    data: data.into(),
                    path,
                })
            } else {
                None
            }
        });

    let file_manager = FileManager { bootrom, rom };

    #[allow(clippy::cast_precision_loss)]
    let initial_window_size = egui::vec2((SCREEN_WIDTH * 5) as f32, (SCREEN_HEIGHT * 5) as f32);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(initial_window_size),
        persist_window: false,
        ..Default::default()
    };

    eframe::run_native(
        "Game Boy",
        native_options,
        Box::new(move |cc| Ok(Box::new(App::new(cc, device_model, Some(file_manager))))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    eframe::WebLogger::init(log::LevelFilter::Info).ok();

    let web_options = eframe::WebOptions {
        max_fps: Some(60),
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("eframe_canvas")
            .expect("Failed to find eframe_canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("eframe_canvas was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(move |cc| Ok(Box::new(App::new(cc, DeviceModel::Cgb, None)))),
            )
            .await;

        // Remove the loading text and the spinner
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(()) => {
                    loading_text.remove();
                }
                Err(err) => {
                    loading_text.set_inner_html(
                        "<p>The app has crashed. See the developer console for details.</p>",
                    );
                    panic!("Failed to start eframe: {err:?}");
                }
            }
        }
    });
}

mod app;
mod audio;
#[cfg(not(target_arch = "wasm32"))]
mod cli;
mod file_manager;
mod gui;
mod key_mappings;
mod utils;
