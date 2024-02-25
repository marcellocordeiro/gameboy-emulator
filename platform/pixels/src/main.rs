use app::App;
use clap::Parser;
use error_iter::ErrorIter as _;
use gb_core::{DeviceModel, SCREEN_HEIGHT, SCREEN_WIDTH};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::KeyCode,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use crate::egui_framework::EguiFramework;

mod app;
mod egui_framework;
mod gui;
mod key_mappings;
mod utils;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set the device model to CGB
    #[arg(short, long, default_value_t = false)]
    cgb: bool,

    /// Optional ROM path (will show file picker if not provided)
    rom: Option<String>,
}

fn main() -> Result<(), Error> {
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
    let rom_path = args.rom;

    #[allow(clippy::cast_precision_loss)]
    let initial_window_size =
        LogicalSize::new((SCREEN_WIDTH * 5) as f32, (SCREEN_HEIGHT * 5) as f32);

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        // let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let size = initial_window_size;
        WindowBuilder::new()
            .with_title("Hello Pixels + egui")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture)?
    };

    let mut egui_framework = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;

        EguiFramework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        )
    };

    // let mut gui = Gui::new();
    let mut app = App::new(&egui_framework.egui_ctx, device_model, rom_path);

    let result = event_loop.run(|event, elwt| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                egui_framework.scale_factor(scale_factor);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    elwt.exit();
                    return;
                }
                egui_framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            // world.update();
            app.update();
            window.request_redraw();
        }

        match event {
            // Draw the current frame
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Draw the world
                // world.draw(pixels.frame_mut());
                app.draw(pixels.frame_mut());

                // Prepare egui
                egui_framework.prepare(&window, &mut app);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    egui_framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    elwt.exit();
                }
            }

            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                egui_framework.handle_event(&window, &event);
            }

            _ => (),
        }
    });

    result.map_err(|e| Error::UserDefined(Box::new(e)))
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
