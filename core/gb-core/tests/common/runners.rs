use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

use gb_core::{components::memory::MemoryInterface as _, constants::DeviceModel, GameBoy};

use super::error::Error;

const TIMEOUT: Duration = Duration::from_secs(20);
const BREAK_OPCODE: u8 = 0x40; // LD B,B

pub fn run_test<F>(device_model: DeviceModel, rom: &'static [u8], runner: F) -> Result<(), Error>
where
    F: FnOnce(&mut GameBoy) -> Result<(), Error>,
{
    let mut gb = GameBoy::new(device_model);
    gb.load(None, rom.to_vec())?;

    runner(&mut gb)?;

    Ok(())
}

pub fn run_until_break(gb: &mut GameBoy) -> Result<(), Error> {
    let start_time = Instant::now();

    while gb.memory().read(gb.cpu().registers().pc) != BREAK_OPCODE {
        gb.step();

        if start_time.elapsed() > TIMEOUT {
            return Err(Error::Timeout);
        }
    }

    Ok(())
}

pub fn run_until_serial_passed(gb: &mut GameBoy) -> Result<(), Error> {
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
            return Ok(());
        }

        if output.contains("Failed") {
            return Err(Error::SerialOutputFailure(output));
        }

        if start_time.elapsed() > TIMEOUT {
            return Err(Error::Timeout);
        }
    }
}

pub fn run_until_memory_status(gb: &mut GameBoy) -> Result<(), Error> {
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
            return Ok(());
        }

        if output.contains("Failed") {
            return Err(Error::MemoryOutputFailure(output));
        }

        if start_time.elapsed() > TIMEOUT {
            return Err(Error::Timeout);
        }
    }
}
