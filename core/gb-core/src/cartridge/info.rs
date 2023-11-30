use crate::constants::ONE_KIB;

use self::{
    dmg_compatibility_palettes::DmgCompatibilityPalettes,
    licensee_code::LicenseeCode,
    ram_size::{get_ram_banks, RAM_BANKS_CODE_ADDRESS},
    rom_size::{get_rom_banks, ROM_BANKS_CODE_ADDRESS},
    sgb_flag::SGB_FLAG_ADDRESS,
    title::Title,
};

pub use self::{
    cartridge_type::CartridgeType, cgb_flag::CgbFlag, extra_features::ExtraFeature,
    ram_size::RAM_BANK_SIZE, rom_size::ROM_BANK_SIZE,
};

pub struct Info {
    // Header info
    pub title: Title,
    pub cartridge_type: CartridgeType,
    pub extra_features: Box<[ExtraFeature]>,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub cgb_flag: CgbFlag,
    pub sgb_flag: bool,

    pub licensee_code: LicenseeCode,

    // File info
    pub file_size: usize,
}

impl TryFrom<&Vec<u8>> for Info {
    type Error = super::error::Error;

    fn try_from(rom: &Vec<u8>) -> Result<Self, Self::Error> {
        let title = Title::with_rom(rom)?;

        let rom_size_code = *rom
            .get(ROM_BANKS_CODE_ADDRESS)
            .ok_or(Self::Error::InvalidRom)?;

        let ram_size_code = *rom
            .get(RAM_BANKS_CODE_ADDRESS)
            .ok_or(Self::Error::InvalidRom)?;

        let cgb_flag = CgbFlag::with_rom(rom)?;

        let sgb_flag_code = *rom.get(SGB_FLAG_ADDRESS).ok_or(Self::Error::InvalidRom)?;

        let licensee_code = LicenseeCode::with_rom(rom)?;

        let rom_banks = get_rom_banks(rom_size_code)?;
        let ram_banks = get_ram_banks(ram_size_code)?;
        let cartridge_type = CartridgeType::with_rom(rom)?;
        let extra_features = ExtraFeature::features_with_rom(rom)?;
        let sgb_flag = sgb_flag::from(sgb_flag_code);

        let file_size = rom.len();

        // Print debug info. Maybe show this elsewhere?
        log::info!("**Cartridge info**");
        log::info!("Title: {}", title.as_string());
        log::info!("Type: {cartridge_type}");
        log::info!(
            "Extra features: {}",
            extra_features
                .iter()
                .map(ExtraFeature::to_string)
                .collect::<Box<[String]>>()
                .join("+")
        );
        log::info!("ROM banks: {rom_banks}");
        log::info!("RAM banks: {ram_banks}");
        log::info!("CGB flag: {cgb_flag}");
        log::info!("SGB flag: {sgb_flag:?}");
        log::info!("Old licensee code: {:#04X}", licensee_code.old());
        log::info!("New licensee code: {}", licensee_code.new_as_string());

        Ok(Self {
            title,
            cartridge_type,
            extra_features,
            rom_banks,
            ram_banks,
            cgb_flag,
            sgb_flag,
            licensee_code,
            file_size,
        })
    }
}

impl Info {
    pub fn validate(&self) {
        // MBC2 always contains RAM, even when `ram_banks == 0`.
        if self.cartridge_type != CartridgeType::Mbc2 {
            let has_ram = self.extra_features.contains(&ExtraFeature::Ram);
            let has_battery = self.extra_features.contains(&ExtraFeature::Battery);

            assert_eq!(self.ram_banks > 0, has_ram);
            assert!(has_ram || !has_battery);
        }

        assert_eq!(
            self.file_size,
            self.rom_banks * (16 * ONE_KIB),
            "ROM length = {} KiB, with ROM banks = {}. Expected {} KiB.",
            self.file_size,
            self.file_size / ONE_KIB,
            self.rom_banks * 16
        );
    }

    pub fn dmg_compatibility_palettes(&self) -> DmgCompatibilityPalettes {
        DmgCompatibilityPalettes::with_header_info(&self.licensee_code, &self.title)
    }
}

mod cartridge_type;
mod cgb_flag;
pub mod dmg_compatibility_palettes;
mod extra_features;
mod licensee_code;
mod ram_size;
mod rom_size;
mod sgb_flag;
mod title;
