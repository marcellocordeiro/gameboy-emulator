// TODO: better failable returns

use log::info;

use crate::cartridge::Error as CartridgeError;

use self::{
    cartridge_type::CARTRIDGE_TYPE_ADDRESS,
    cgb_flag::CGB_FLAG_ADDRESS,
    ram_size::{get_ram_banks, RAM_BANKS_CODE_ADDRESS},
    rom_size::{get_rom_banks, ROM_BANKS_CODE_ADDRESS},
    sgb_flag::SGB_FLAG_ADDRESS,
    title::{get_title, TITLE_ADDRESS_BEGIN, TITLE_ADDRESS_END},
};

pub use self::{
    cartridge_type::CartridgeType, cgb_flag::CgbFlag, ram_size::RAM_BANK_SIZE,
    rom_size::ROM_BANK_SIZE,
};

pub struct Info {
    pub title: String,
    pub cartridge_type: CartridgeType,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub cgb_flag: CgbFlag,
    pub sgb_flag: bool,
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

        let title = get_title(title_bytes);
        let rom_banks = get_rom_banks(rom_size_code)?;
        let ram_banks = get_ram_banks(ram_size_code)?;
        let cartridge_type = CartridgeType::try_from((cartridge_type_code, ram_banks))?;
        let cgb_flag = CgbFlag::from(cgb_flag_code);
        let sgb_flag = sgb_flag::from(sgb_flag_code);

        info!("**Cartridge info**");
        info!("Title: {title}");
        info!("Type: {cartridge_type}");
        info!("ROM banks: {rom_banks}");
        info!("RAM banks: {ram_banks}");
        info!("CGB flag: {cgb_flag}");

        Ok(Self {
            title,
            cartridge_type,
            rom_banks,
            ram_banks,
            cgb_flag,
            sgb_flag,
        })
    }
}

mod cartridge_type;
mod cgb_flag;
mod ram_size;
mod rom_size;
mod sgb_flag;
mod title;
