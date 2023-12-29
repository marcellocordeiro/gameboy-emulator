use gb_core::{GameBoy, SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH};

pub fn validate_fibonacci(gb: GameBoy) {
    let regs = gb.cpu().registers();

    let is_fibonacci = regs.a == 0
        && regs.b == 3
        && regs.c == 5
        && regs.d == 8
        && regs.e == 13
        && regs.h == 21
        && regs.l == 34;

    assert!(is_fibonacci, "Validation failure");
}

pub fn validate_screenshot(gb: GameBoy, name: &'static str) {
    let mut frame = [0; SCREEN_PIXELS_SIZE];

    gb.draw_into_frame_rgba8888(&mut frame);

    let expected_image = match image::open(format!("./tests/expected/{name}.png")) {
        Ok(image) => image,
        Err(err) => {
            dump_image(format!("./tests/actual/{name}.png"), frame.as_slice());
            panic!("{err}");
        }
    };

    if frame != expected_image.as_bytes() {
        dump_image(format!("./tests/actual/{name}.png"), frame.as_slice());
        panic!("Assertion failed. The actual frame does not match the expected one.");
    }
}

fn dump_image(path: String, bytes: &[u8]) {
    image::save_buffer_with_format(
        path,
        bytes,
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .unwrap();
}
