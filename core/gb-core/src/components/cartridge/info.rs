use std::sync::Arc;

use cgb_flag::CgbFlag;
use compatibility_palettes::CompatibilityPalettes;
use extra_features::ExtraFeature;
use itertools::Itertools;
use licensee_code::LicenseeCode;
use mbc_type::MbcType;
use title::Title;

use super::error::CartridgeError;
use crate::components::cartridge::info::rom_banks::ROM_BANK_SIZE;

pub struct Info {
    pub rom: Arc<[u8]>,

    // Header info
    pub title: Title,
    pub mbc_type: MbcType,
    pub extra_features: Box<[ExtraFeature]>,
    pub rom_banks: usize,
    pub ram_banks: usize,
    pub cgb_flag: CgbFlag,
    pub sgb_flag: bool,
    pub licensee_code: LicenseeCode,
}

impl Info {
    pub fn new(rom: Arc<[u8]>) -> Result<Self, CartridgeError> {
        let header = header::from_rom(&rom)?;

        let title = Title::from_header(header);
        let cgb_flag = CgbFlag::from_header(header);
        let licensee_code = LicenseeCode::from_header(header);
        let rom_banks = rom_banks::from_header(header)?;
        let ram_banks = ram_banks::from_header(header)?;
        let mbc_type = MbcType::from_header(header)?;
        let extra_features = ExtraFeature::from_header(header)?;
        let sgb_flag = sgb_flag::from_header(header);

        // Print debug info. Maybe show this elsewhere?
        log::info!(target: "cartridge", "**Cartridge info**");
        log::info!(target: "cartridge", "Title: {title}");
        log::info!(target: "cartridge", "MBC type: {mbc_type}");
        log::info!(
            target: "cartridge",
            "Extra features: {extra_features}",
            extra_features = extra_features.iter().format("+")
        );
        log::info!(target: "cartridge", "ROM size: {rom_banks} banks");
        log::info!(target: "cartridge", "RAM banks: {ram_banks} banks");
        log::info!(target: "cartridge", "CGB flag: {cgb_flag}");
        log::info!(target: "cartridge", "SGB flag: {sgb_flag}");
        log::info!(target: "cartridge", "Old licensee code: {:#04X}", licensee_code.old_code());
        log::info!(
            target: "cartridge",
            "New licensee code: {}",
            licensee_code.new_code().unwrap_or("--")
        );

        Ok(Self {
            rom,
            title,
            mbc_type,
            extra_features,
            rom_banks,
            ram_banks,
            cgb_flag,
            sgb_flag,
            licensee_code,
        })
        .inspect(|c| {
            c.validate();
        })
    }

    fn validate(&self) {
        let actual_rom_size = self.rom.len();
        let expected_rom_size = self.rom_banks * ROM_BANK_SIZE;

        if actual_rom_size != expected_rom_size {
            log::warn!(
                "ROM length = {actual_rom_size} KiB, with ROM banks = {rom_banks}. Expected {expected_rom_size} KiB",
                rom_banks = self.rom_banks,
            );
        }

        // MBC2 always contains RAM, even when `ram_banks == 0`.
        if self.mbc_type != MbcType::Mbc2 {
            let has_ram = self.extra_features.contains(&ExtraFeature::Ram);
            let has_battery = self.extra_features.contains(&ExtraFeature::Battery);

            if (self.ram_banks > 0) != has_ram {
                log::warn!("RAM banks = {} but has_ram = {}", self.ram_banks, has_ram);
            }

            if has_battery && !has_ram {
                log::warn!("Supports battery backed RAM but does not have RAM");
            }
        }
    }

    #[must_use]
    pub fn dmg_compatibility_palettes(&self) -> CompatibilityPalettes {
        CompatibilityPalettes::from_header_info(&self.licensee_code, &self.title)
    }
}

pub mod cgb_flag;
pub mod compatibility_palettes;
pub mod extra_features;
pub mod header;
pub mod licensee_code;
pub mod mbc_type;
pub mod ram_banks;
pub mod rom_banks;
pub mod sgb_flag;
pub mod title;
