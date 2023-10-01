use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use gb_core::GameBoy;

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

pub fn run_until_break(rom: &[u8]) -> GameBoy {
    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    let start_time = Instant::now();

    loop {
        if Instant::now() - start_time > TIMEOUT {
            panic!("Timed out");
        }

        gb.cpu.step();

        if gb.cpu.memory.read(gb.cpu.registers.program_counter) == BREAK_OPCODE {
            gb.cpu.step();
            break;
        }
    }

    gb
}

pub fn run_until_serial_passed(rom: &[u8]) -> GameBoy {
    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    let (sender, receiver) = mpsc::channel::<u8>();

    let mut output = String::default();
    gb.cpu.memory.serial.add_sender(sender);

    let start_time = Instant::now();

    loop {
        for _ in 0..30 {
            gb.run_frame();
        }

        while let Ok(ch) = receiver.try_recv() {
            output.push(ch as char);
        }

        if output.contains("Passed") {
            break;
        }

        if output.contains("Failed") {
            panic!("Failed");
        }

        if Instant::now() - start_time > TIMEOUT {
            panic!("Timed out");
        }
    }

    gb
}

pub fn run_until_memory_status(rom: &[u8]) -> GameBoy {
    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    let start_time = Instant::now();

    loop {
        for _ in 0..30 {
            gb.run_frame();
        }

        let contents = (0xA004..(0xA004 + 100))
            .map(|address| gb.cpu.memory.read(address))
            .take_while(|value| *value != 0)
            .collect::<Vec<u8>>();

        let decoded = String::from_utf8_lossy(&contents)
            .trim()
            .chars()
            .filter(char::is_ascii)
            .collect::<String>();

        if decoded.contains("Passed") {
            break;
        }

        if decoded.contains("Failed") {
            panic!("Failed");
        }

        if Instant::now() - start_time > TIMEOUT {
            panic!("Timed out: {decoded}",);
        }
    }

    gb
}
