use std::{
    io::Write,
    time::{Duration, Instant},
};

use gb_core::{constants::FRAME_SIZE, GameBoy};

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

fn run(rom: Vec<u8>, expected_bytes: [u8; FRAME_SIZE]) {
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

    gb.draw(&mut frame);

    if frame != expected_bytes {
        panic!("Assertion failed. The actual frame does not match the expected one.");
    }
}

#[cfg(not(feature = "cgb"))]
#[test]
fn test_dmg_acid2_dmg() {
    let rom = include_bytes!("../../external/gameboy-test-roms/dmg-acid2.gb");
    let expected_bytes = include_bytes!("./expected/dmg-acid2-dmg-expected.bin");

    run(rom.to_vec(), *expected_bytes);
}

#[cfg(feature = "cgb")]
#[test]
#[ignore = "need to implement proper CGB support first"]
fn test_dmg_acid2_cgb() {
    let rom = include_bytes!("../../external/gameboy-test-roms/dmg-acid2.gb");
    let expected_bytes = include_bytes!("./expected/dmg-acid2-dmg-expected.bin");

    run(rom.to_vec(), *expected_bytes);
}

#[allow(dead_code)]
fn dump_bytes(bytes: [u8; FRAME_SIZE]) {
    let mut file = std::fs::File::create("./tests/expected/new_file.bin").unwrap();
    file.write_all(&bytes).unwrap();
}
