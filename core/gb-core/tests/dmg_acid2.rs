use common::{runners::run_until_break, validators::validate_screenshot};
use gb_core::{constants::DeviceModel, GameBoy};

mod common;

#[test]
fn test_dmg_acid2_dmg() {
    let name = "dmg-acid2_dmg";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let mut gb = GameBoy::new(DeviceModel::Dmg);
    gb.load(None, rom.to_vec()).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}

#[test]
fn test_dmg_acid2_cgb() {
    let name = "dmg-acid2_cgb";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let mut gb = GameBoy::new(DeviceModel::Cgb);
    gb.load(None, rom.to_vec()).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}
