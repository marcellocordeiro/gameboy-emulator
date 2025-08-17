use common::{runners::run_until_break, validators::validate_screenshot};
use gb_core::{GameBoy, constants::DeviceModel};

mod common;

#[test]
fn test_cgb_acid2() {
    let name = "cgb-acid2";

    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../",
        "external/gameboy-test-roms/",
        "cgb-acid2.gbc"
    );
    let rom = std::fs::read(path).unwrap();

    let mut gb = GameBoy::new(DeviceModel::Cgb);
    gb.load(None, rom.into()).unwrap();

    run_until_break(&mut gb).unwrap();
    validate_screenshot(&gb, name).unwrap();
}
