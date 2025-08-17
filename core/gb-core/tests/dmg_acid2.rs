use common::{runners::run_until_break, validators::validate_screenshot};
use gb_core::{GameBoy, constants::DeviceModel};

mod common;

#[test]
fn test_dmg_acid2_dmg() {
    let name = "dmg-acid2_dmg";

    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../",
        "external/gameboy-test-roms/",
        "dmg-acid2.gb"
    );
    let rom = std::fs::read(path).unwrap();

    let mut gb = GameBoy::new(DeviceModel::Dmg);
    gb.load(None, rom.into()).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}

#[test]
fn test_dmg_acid2_cgb() {
    let name = "dmg-acid2_cgb";

    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../",
        "external/gameboy-test-roms/",
        "dmg-acid2.gb"
    );
    let rom = std::fs::read(path).unwrap();

    let mut gb = GameBoy::new(DeviceModel::Cgb);
    gb.load(None, rom.into()).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}
