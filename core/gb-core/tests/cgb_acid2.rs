#![cfg(feature = "cgb")]

mod common;
use common::{runners::run_until_break, validators::validate_screenshot};

#[test]
fn test_cgb_acid2() {
    let name = "cgb-acid2";
    let rom = include_bytes!("../../../external/gameboy-test-roms/cgb-acid2.gbc");

    let gb = run_until_break(rom);

    validate_screenshot(gb, name);
}
