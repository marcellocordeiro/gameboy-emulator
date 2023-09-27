#![cfg(feature = "cgb")]

use std::time::{Duration, Instant};

use gb_core::{
    constants::{FRAME_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
    GameBoy,
};

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

fn run(name: &str, rom: Vec<u8>) {
    let mut gb = GameBoy::new();
    gb.load_cartridge(rom).unwrap();

    let start_time = Instant::now();

    loop {
        if Instant::now() - start_time > TIMEOUT {
            panic!("Timeout");
        }

        gb.cpu.step();

        if gb.cpu.memory.read(gb.cpu.registers.program_counter) == BREAK_OPCODE {
            gb.cpu.step();
            break;
        }
    }

    let mut frame = [0; FRAME_SIZE];

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

#[test]
fn test_cgb_acid2() {
    let rom = include_bytes!("../../../external/gameboy-test-roms/cgb-acid2.gbc");

    run("cgb-acid2", rom.to_vec());
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
