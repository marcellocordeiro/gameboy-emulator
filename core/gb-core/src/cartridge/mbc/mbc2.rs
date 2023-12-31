use super::MbcInterface;
use crate::cartridge::info::{CartridgeType, Info, ROM_BANK_SIZE};

pub struct Mbc2 {
    rom: Vec<u8>,
    ram: Box<[u8; 512]>,

    rom_bank_mask: usize,

    ram_enable: bool,

    rom_bank: u8,
}

impl Mbc2 {
    pub fn new(rom: Vec<u8>, info: &Info) -> Self {
        assert_eq!(info.cartridge_type, CartridgeType::Mbc2);

        let rom_bank_mask = info.rom_banks - 1;

        Self {
            rom,
            ram: Box::new([0; 512]),

            rom_bank_mask,

            ram_enable: false,

            rom_bank: 0x01,
        }
    }

    fn rom_0x4000_0x7fff_offset(&self) -> usize {
        ROM_BANK_SIZE * ((self.rom_bank as usize) & self.rom_bank_mask)
    }
}

impl MbcInterface for Mbc2 {
    fn reset(&mut self) {
        self.ram.fill(0);
        self.ram_enable = false;
        self.rom_bank = 0x01;
    }

    fn get_battery(&self) -> &[u8] {
        self.ram.as_ref()
    }

    fn load_battery(&mut self, file: Vec<u8>) {
        self.ram = if let Ok(file) = file.try_into() {
            file
        } else {
            log::error!("Error loading the battery backed RAM.");
            return;
        }
    }

    fn read_rom_bank_0(&self, address: u16) -> u8 {
        let address = address as usize;
        self.rom[address]
    }

    fn read_rom_bank_x(&self, address: u16) -> u8 {
        let address = (address - 0x4000) as usize;
        let offset = self.rom_0x4000_0x7fff_offset();

        self.rom[address + offset]
    }

    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enable {
            return 0xFF;
        }

        let address = (address - 0xA000) as usize;

        0b1111_0000 | self.ram[address & 0x01FF]
    }

    fn write_rom(&mut self, address: u16, value: u8) {
        if address >= 0x4000 {
            return;
        }

        // If the bit 8 (LSB from the upper address byte) is set,
        // select `ram_enable`. Otherwise, select `rom_bank`.
        if address & 0x0100 == 0 {
            self.ram_enable = (value & 0b1111) == 0x0A;
        } else {
            self.rom_bank = value & 0b1111;
            if self.rom_bank == 0 {
                self.rom_bank = 1;
            }
        }
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        if !self.ram_enable {
            return;
        }

        let address = (address - 0xA000) as usize;

        self.ram[address & 0x01FF] = value;
    }
}
