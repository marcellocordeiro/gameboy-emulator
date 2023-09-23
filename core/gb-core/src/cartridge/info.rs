use log::info;

use crate::{
    cartridge::{
        error::Error as CartridgeError,
        info::{
            new_licensee_code::{NEW_LICENSEE_CODE_ADDRESS_BEGIN, NEW_LICENSEE_CODE_ADDRESS_END},
            old_licensee_code::OLD_LICENSEE_CODE_ADDRESS,
        },
    },
    constants::ONE_KIB,
};

use self::{
    cartridge_type::CARTRIDGE_TYPE_ADDRESS,
    cgb_flag::CGB_FLAG_ADDRESS,
    ram_size::{get_ram_banks, RAM_BANKS_CODE_ADDRESS},
    rom_size::{get_rom_banks, ROM_BANKS_CODE_ADDRESS},
    sgb_flag::SGB_FLAG_ADDRESS,
    title::{get_title, TITLE_ADDRESS_BEGIN, TITLE_ADDRESS_END},
};

pub use self::{
    cartridge_type::CartridgeType, cgb_flag::CgbFlag, extra_features::ExtraFeature,
    ram_size::RAM_BANK_SIZE, rom_size::ROM_BANK_SIZE,
};

pub struct Info {
    // Header info
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub extra_features: Vec<ExtraFeature>,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub cgb_flag: CgbFlag,
    pub sgb_flag: bool,

    // File info
    pub file_size: usize,
}

impl TryFrom<&Vec<u8>> for Info {
    type Error = CartridgeError;

    fn try_from(rom: &Vec<u8>) -> Result<Self, Self::Error> {
        let title_bytes = rom
            .get(TITLE_ADDRESS_BEGIN..=TITLE_ADDRESS_END)
            .ok_or(CartridgeError::InvalidRom)?;

        let cartridge_type_code = *rom
            .get(CARTRIDGE_TYPE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let rom_size_code = *rom
            .get(ROM_BANKS_CODE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let ram_size_code = *rom
            .get(RAM_BANKS_CODE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let cgb_flag_code = *rom
            .get(CGB_FLAG_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let sgb_flag_code = *rom
            .get(SGB_FLAG_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let old_licensee_code = *rom
            .get(OLD_LICENSEE_CODE_ADDRESS)
            .ok_or(CartridgeError::InvalidRom)?;

        let new_licensee_code_bytes = rom
            .get(NEW_LICENSEE_CODE_ADDRESS_BEGIN..=NEW_LICENSEE_CODE_ADDRESS_END)
            .ok_or(CartridgeError::InvalidRom)?;

        let title = get_title(title_bytes);
        let rom_banks = get_rom_banks(rom_size_code)?;
        let ram_banks = get_ram_banks(ram_size_code)?;
        let cartridge_type = CartridgeType::try_from((cartridge_type_code, ram_banks))?;
        let extra_features = ExtraFeature::get_features(cartridge_type_code);
        let cgb_flag = CgbFlag::from(cgb_flag_code);
        let sgb_flag = sgb_flag::from(sgb_flag_code);
        let new_licensee_code = new_licensee_code::get_new_licensee_code(new_licensee_code_bytes);

        let file_size = rom.len();

        // Print debug info. Maybe show this elsewhere?
        info!("**Cartridge info**");
        info!("Title: {title}");
        info!("Type: {cartridge_type}");
        info!(
            "Extra features: {}",
            extra_features
                .iter()
                .map(ExtraFeature::to_string)
                .collect::<Vec<String>>()
                .join("+")
        );
        info!("ROM banks: {rom_banks}");
        info!("RAM banks: {ram_banks}");
        info!("CGB flag: {cgb_flag}");
        info!("SGB flag: {sgb_flag:?}");
        info!("Old licensee code: {old_licensee_code:#04X}");
        info!("New licensee code: {new_licensee_code}");

        Ok(Self {
            title,
            cartridge_type,
            extra_features,
            rom_banks,
            ram_banks,
            cgb_flag,
            sgb_flag,
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
}

mod cartridge_type;
mod cgb_flag;
mod extra_features;
mod new_licensee_code;
mod old_licensee_code;
mod ram_size;
mod rom_size;
mod sgb_flag;
mod title;
