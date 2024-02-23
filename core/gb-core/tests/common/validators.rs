use gb_core::{GameBoy, SCREEN_HEIGHT, SCREEN_PIXELS_SIZE, SCREEN_WIDTH};

use super::error::Error;

pub fn validate_fibonacci(gb: &GameBoy) -> Result<(), Error> {
    let regs = gb.cpu().registers();

    let is_fibonacci = regs.a == 0
        && regs.b == 3
        && regs.c == 5
        && regs.d == 8
        && regs.e == 13
        && regs.h == 21
        && regs.l == 34;

    if is_fibonacci {
        Ok(())
    } else {
        Err(Error::Timeout)
    }
}

pub fn validate_screenshot(gb: &GameBoy, name: &'static str) -> Result<(), Error> {
    let mut frame = [0; SCREEN_PIXELS_SIZE];

    gb.draw_into_frame_rgba8888(&mut frame);

    let expected_image = image::open(format!("./tests/expected/{name}.png")).inspect_err(|_| {
        dump_image(format!("./tests/actual/{name}.png"), frame.as_slice()).unwrap();
    })?;

    if frame == expected_image.as_bytes() {
        Ok(())
    } else {
        dump_image(format!("./tests/actual/{name}.png"), frame.as_slice())?;
        Err(Error::SnapshotMismatch)
    }
}

fn dump_image(path: String, bytes: &[u8]) -> Result<(), Error> {
    image::save_buffer_with_format(
        path,
        bytes,
        SCREEN_WIDTH as u32,
        SCREEN_HEIGHT as u32,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )?;

    Ok(())
}
