mod common;
use common::{runners::run_until_break, validators::validate_screenshot};

#[cfg(not(feature = "cgb"))]
#[test]
fn test_dmg_acid2_dmg() {
    let name = "dmg-acid2_dmg";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let gb = run_until_break(rom);

    validate_screenshot(gb, name);
}

#[cfg(all(feature = "cgb", feature = "bootrom"))]
#[test]
#[ignore = "need to implement proper CGB support first (preferably without relying on the bootrom)"]
fn test_dmg_acid2_cgb() {
    let name = "dmg-acid2_cgb";
    let rom = include_bytes!("../../../external/gameboy-test-roms/dmg-acid2.gb");

    let gb = run_until_break(rom);

    validate_screenshot(gb, name);
}
