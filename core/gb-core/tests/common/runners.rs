use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use gb_core::GameBoy;

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

pub fn run_until_break(gb: &mut GameBoy) {
    let start_time = Instant::now();

    while gb.cpu.memory.read(gb.cpu.registers.program_counter) != BREAK_OPCODE {
        gb.cpu.step();

        if Instant::now() - start_time > TIMEOUT {
            panic!("Timed out");
        }
    }
}

pub fn run_until_serial_passed(gb: &mut GameBoy) {
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
}

pub fn run_until_memory_status(gb: &mut GameBoy) {
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
            panic!("Timed out: {decoded}");
        }
    }
}
