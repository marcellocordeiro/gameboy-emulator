use gb_core::constants::DeviceModel;

use super::{
    error::Error,
    runners::{run_test, run_until_break},
    validators::validate_fibonacci,
};

pub fn run(model: DeviceModel, rom: &[u8]) -> Result<(), Error> {
    run_test(model, rom, |gb| {
        run_until_break(gb)?;
        validate_fibonacci(gb)?;

        Ok(())
    })
}
