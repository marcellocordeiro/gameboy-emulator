use std::path::Path;

use gb_core::GameBoy;

pub fn load_battery<P: AsRef<Path>>(gb: &mut GameBoy, rom_path: P) {
    let rom_path = rom_path.as_ref();

    let path = Path::new(rom_path).with_extension("srm");

    let result = path.try_exists().unwrap();

    if result {
        log::info!("Loading battery file from {}", path.display());
        let file = std::fs::read(path).unwrap();
        gb.load_battery(file);
    } else {
        log::info!("No battery file was found.");
    }
}

pub fn save_battery<P: AsRef<Path>>(gb: &GameBoy, rom_path: P) {
    if let Some(battery) = gb.get_battery() {
        log::info!("Saving battery file...");

        let rom_path = rom_path.as_ref();
        let path = Path::new(rom_path).with_extension("srm");

        std::fs::write(path, battery).unwrap();
    }
}
