use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use gb_core::{GameBoy, MemoryInterface};

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

pub fn run_until_break(gb: &mut GameBoy) {
    let start_time = Instant::now();

    while gb.memory().read(gb.cpu().registers().pc) != BREAK_OPCODE {
        gb.step();

        assert!(start_time.elapsed() <= TIMEOUT, "Timed out");
    }
}

pub fn run_until_serial_passed(gb: &mut GameBoy) {
    let (sender, receiver) = mpsc::channel::<u8>();

    let mut output = String::default();
    gb.add_serial_channel(sender);

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

        assert!(!output.contains("Failed"), "Failed");
        assert!(start_time.elapsed() <= TIMEOUT, "Timed out");
    }
}

pub fn run_until_memory_status(gb: &mut GameBoy) {
    let start_time = Instant::now();

    loop {
        for _ in 0..30 {
            gb.run_frame();
        }

        let output = (0xA004..(0xA004 + 100))
            .take_while(|value| *value != 0)
            .map(|address| gb.memory().read(address) as char)
            .collect::<String>();

        if output.contains("Passed") {
            break;
        }

        assert!(!output.contains("Failed"), "Failed");
        assert!(start_time.elapsed() <= TIMEOUT, "Timed out");
    }
}
