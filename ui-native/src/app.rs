use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use gameboy_emulator_core::{
    constants::{Button, HEIGHT, WIDTH},
    GameBoy,
};

use crate::overlay::Overlay;
use crate::overlay_framework::OverlayFramework;

const SCALE: f64 = 3.0;

pub fn run(rom: Vec<u8>) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new((WIDTH as f64) * SCALE, (HEIGHT as f64) * SCALE);

        WindowBuilder::new()
            .with_title("gameboy-emulator")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // pixels buffer
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    // Set up the egui backend
    let mut overlay_framework = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor();

        OverlayFramework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor as f32,
            &pixels,
        )
    };

    let mut gb = GameBoy::new();
    gb.load_cartridge(rom);

    let mut overlay = Overlay::new();

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                control_flow.set_exit();
                return;
            }

            for button in Button::to_array() {
                let key = map_button(button);

                if input.key_pressed(key) {
                    gb.key_down(button);
                } else if input.key_released(key) {
                    gb.key_up(button);
                }
            }

            // Update the scale factor
            if let Some(scale_factor) = input.scale_factor() {
                overlay_framework.scale_factor(scale_factor as f32);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("pixels.render() failed: {err}");
                    control_flow.set_exit();
                    return;
                }

                overlay_framework.resize(size.width, size.height);
            }

            // Update internal state and request a redraw
            if !overlay.manual_control {
                gb.run_frame();
            }

            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                overlay_framework.handle_event(&event);

                /* if let WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        if let Some(key) = map_key(virtual_keycode) {
                            match input.state {
                                winit::event::ElementState::Pressed => gb.key_down(key),
                                winit::event::ElementState::Released => gb.key_up(key),
                            }
                        }
                    }
                } */
            }

            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                gb.draw(pixels.frame_mut());

                // Prepare egui
                overlay_framework.prepare(&window, |egui_ctx| {
                    // Draw the demo application.
                    overlay.ui(egui_ctx, &mut gb);
                });

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    overlay_framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    println!("pixels.render() failed: {err}");
                    control_flow.set_exit();
                }
            }

            _ => (),
        }
    });
}

/* fn map_key(virtual_keycode: VirtualKeyCode) -> Option<Button> {
    match virtual_keycode {
        VirtualKeyCode::A => Some(Button::A),
        VirtualKeyCode::S => Some(Button::B),
        VirtualKeyCode::Left => Some(Button::Left),
        VirtualKeyCode::Up => Some(Button::Up),
        VirtualKeyCode::Right => Some(Button::Right),
        VirtualKeyCode::Down => Some(Button::Down),
        VirtualKeyCode::Back => Some(Button::Select),
        VirtualKeyCode::Return => Some(Button::Start),

        _ => None,
    }
} */

fn map_button(button: Button) -> VirtualKeyCode {
    match button {
        Button::A => VirtualKeyCode::A,
        Button::B => VirtualKeyCode::S,
        Button::Select => VirtualKeyCode::Back,
        Button::Start => VirtualKeyCode::Return,
        Button::Right => VirtualKeyCode::Right,
        Button::Left => VirtualKeyCode::Left,
        Button::Up => VirtualKeyCode::Up,
        Button::Down => VirtualKeyCode::Down,
    }
}
