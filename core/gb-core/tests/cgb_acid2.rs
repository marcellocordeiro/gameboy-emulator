use common::{runners::run_until_break, validators::validate_screenshot};
use gb_core::{DeviceModel, GameBoy};

mod common;

#[test]
fn test_cgb_acid2() {
    let name = "cgb-acid2";
    let rom = include_bytes!("../../../external/gameboy-test-roms/cgb-acid2.gbc");

    let mut gb = GameBoy::new(DeviceModel::Cgb);
    gb.load(rom.to_vec(), None).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}
