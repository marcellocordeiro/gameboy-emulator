use std::sync::Arc;

pub use self::{
    cgb_flag::CgbFlag,
    extra_features::ExtraFeature,
    mbc_type::MbcType,
    ram_banks::RAM_BANK_SIZE,
    rom_banks::ROM_BANK_SIZE,
};
use self::{
    compatibility_palettes::CompatibilityPalettes,
    licensee_code::LicenseeCode,
    title::Title,
};
use crate::constants::ONE_KIB;

pub struct Cartridge {
    pub rom: Arc<Box<[u8]>>,

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

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self, self::error::Error> {
        let rom = Arc::<Box<[u8]>>::from(rom.into_boxed_slice());
        let header = header::from_rom(&rom)?;

        let title = Title::from_header(header)?;

        let cgb_flag = CgbFlag::from_header(header);

        let licensee_code = LicenseeCode::from_header(header)?;

        let rom_banks = rom_banks::from_header(header)?;
        let ram_banks = ram_banks::from_header(header)?;
        let mbc_type = MbcType::from_header(header)?;
        let extra_features = ExtraFeature::from_header(header);
        let sgb_flag = sgb_flag::from_header(header);

        // Print debug info. Maybe show this elsewhere?
        log::info!("**Cartridge info**");
        log::info!("Title: {}", title.as_string());
        log::info!("Type: {mbc_type}");
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
        .and_then(|c| {
            c.validate();
            Ok(c)
        })
    }

    fn validate(&self) {
        // MBC2 always contains RAM, even when `ram_banks == 0`.
        if self.mbc_type != MbcType::Mbc2 {
            let has_ram = self.extra_features.contains(&ExtraFeature::Ram);
            let has_battery = self.extra_features.contains(&ExtraFeature::Battery);

            assert_eq!(self.ram_banks > 0, has_ram);
            assert!(has_ram || !has_battery);
        }

        assert_eq!(
            self.rom.len(),
            self.rom_banks * (16 * ONE_KIB),
            "ROM length = {} KiB, with ROM banks = {}. Expected {} KiB.",
            self.rom.len(),
            self.rom.len() / ONE_KIB,
            self.rom_banks * 16
        );
    }

    pub fn dmg_compatibility_palettes(&self) -> CompatibilityPalettes {
        CompatibilityPalettes::from_header_info(&self.licensee_code, &self.title)
    }
}

mod cgb_flag;
pub mod compatibility_palettes;
pub mod error;
mod extra_features;
mod header;
mod licensee_code;
mod mbc_type;
mod ram_banks;
mod rom_banks;
mod sgb_flag;
mod title;
