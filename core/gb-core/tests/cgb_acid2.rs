#![cfg(feature = "cgb")]

mod common;
use common::{runners::run_until_break, validators::validate_screenshot};

use gb_core::GameBoy;

#[test]
fn test_cgb_acid2() {
    let name = "cgb-acid2";
    let rom = include_bytes!("../../../external/gameboy-test-roms/cgb-acid2.gbc");

    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    run_until_break(&mut gb);
    validate_screenshot(gb, name);
}
