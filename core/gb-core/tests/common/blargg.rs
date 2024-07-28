use gb_core::constants::DeviceModel;

use super::{
    error::Error,
    runners::{run_test, run_until_memory_status, run_until_serial_passed},
};

pub fn run_serial(model: DeviceModel, rom: &'static [u8]) -> Result<(), Error> {
    run_test(model, rom, |gb| {
        run_until_serial_passed(gb)?;

        Ok(())
    })
}

pub fn run_memory(model: DeviceModel, rom: &'static [u8]) -> Result<(), Error> {
    run_test(model, rom, |gb| {
        run_until_memory_status(gb)?;

        Ok(())
    })
}
