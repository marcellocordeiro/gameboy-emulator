mod common;
use common::{runners::run_until_break, validators::validate_screenshot};

use gb_core::GameBoy;

#[cfg(not(feature = "cgb"))]
#[test]
fn test_dmg_acid2_dmg() {
    let name = "dmg-acid2_dmg";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    run_until_break(&mut gb);
    validate_screenshot(gb, name);
}

#[cfg(feature = "cgb")]
#[test]
fn test_dmg_acid2_cgb() {
    let name = "dmg-acid2_cgb";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let mut gb = GameBoy::new();
    gb.load_cartridge(rom.to_vec()).unwrap();

    run_until_break(&mut gb);
    validate_screenshot(gb, name);
}
