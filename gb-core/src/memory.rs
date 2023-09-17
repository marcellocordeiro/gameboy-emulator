use log::info;

use crate::{
    audio::Audio,
    cartridge::{error::Error as CartridgeError, info::CgbFlag, Cartridge},
    graphics::Graphics,
    joypad::Joypad,
    serial::Serial,
    timer::Timer,
};

use self::{
    bootrom::Bootrom, high_ram::HighRam, interrupts::Interrupts, oam_dma::OamDma,
    speed_switch::SpeedSwitch, undocumented_registers::UndocumentedRegisters, vram_dma::VramDma,
    work_ram::WorkRam,
};

#[derive(Default)]
pub struct Memory {
    pub bootrom: Bootrom,

    pub wram: WorkRam,
    pub hram: HighRam,

    pub cartridge: Option<Cartridge>,
    pub graphics: Graphics,
    pub audio: Audio,

    pub joypad: Joypad,
    pub serial: Serial,
    pub timer: Timer,

    pub speed_switch: SpeedSwitch,

    pub interrupts: Interrupts,

    pub undocumented_registers: UndocumentedRegisters,

    pub oam_dma: OamDma,
    pub vram_dma: VramDma,
}

impl Memory {
    pub fn reset(&mut self) {
        self.bootrom = Bootrom::default();
        self.wram = WorkRam::default();
        self.hram = HighRam::default();
        self.graphics = Graphics::default();
        self.audio = Audio::default();
        self.serial = Serial::default();
        self.timer = Timer::default();
        self.speed_switch = SpeedSwitch::default();
        self.undocumented_registers = UndocumentedRegisters::default();
        self.oam_dma = OamDma::default();
        self.vram_dma = VramDma::default();

        if let Some(cartridge) = self.cartridge.as_mut() {
            cartridge.reset();

            if cfg!(feature = "cgb")
                && (cfg!(feature = "bootrom") || cartridge.info.cgb_flag.has_cgb_support())
            {
                self.set_cgb_mode(true);
            }
        };
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        let cartridge = Cartridge::new(rom)?;

        if cfg!(feature = "cgb")
            && (cfg!(feature = "bootrom") || cartridge.info.cgb_flag.has_cgb_support())
        {
            self.set_cgb_mode(true);
        }

        self.cartridge = Some(cartridge);

        Ok(())
    }

    pub fn set_cgb_mode(&mut self, value: bool) {
        info!("{} CGB mode.", if value { "Enabling" } else { "Disabling" });

        self.wram.set_cgb_mode(value);
        self.graphics.set_cgb_mode(value);
        self.speed_switch.set_cgb_mode(value);
        self.undocumented_registers.set_cgb_mode(value);
    }

    pub fn skip_bootrom(&mut self) {
        self.bootrom.disable();
        self.graphics.skip_bootrom();
        self.timer.skip_bootrom();
        self.interrupts.skip_bootrom();
    }

    pub fn tick(&mut self) {
        if self.speed_switch.in_double_speed() {
            panic!("CGB double speed not yet supported.");
        }

        if let Some((source, destination)) = self.oam_dma.advance() {
            let value = self.read(source);
            self.graphics.oam.write(destination, value);
        }

        for _ in 0..4 {
            self.timer.tick();
            self.graphics.tick();
        }

        if self.graphics.vblank_irq {
            self.interrupts.request_vblank();
            self.graphics.vblank_irq = false;
        }

        if self.graphics.stat_irq {
            self.interrupts.request_lcd_stat();
            self.graphics.stat_irq = false;
        }

        if self.timer.irq {
            self.interrupts.request_timer();
            self.timer.irq = false;
        }

        if self.serial.irq {
            self.interrupts.request_serial();
            self.serial.irq = false;
        }

        if self.joypad.irq {
            self.interrupts.request_joypad();
            self.joypad.irq = false;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF if self.bootrom.is_active() => self.bootrom.read(address),

            #[cfg(feature = "cgb")]
            0x0200..=0x08FF if self.bootrom.is_active() => self.bootrom.read(address),

            0x0000..=0x7FFF => self
                .cartridge
                .as_ref()
                .expect("Cartridge should be loaded")
                .read_rom(address),

            0x8000..=0x9FFF => self.graphics.read_vram(address),

            0xA000..=0xBFFF => self
                .cartridge
                .as_ref()
                .expect("Cartridge should be loaded")
                .read_ram(address),

            0xC000..=0xFDFF => self.wram.read(address),
            0xFE00..=0xFE9F => self.graphics.read_oam(address),

            0xFEA0..=0xFEFF => unreachable!("Accessing prohibited area: {:#06x}", address),

            // I/O registers.
            0xFF00 => self.joypad.read(),

            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),

            0xFF0F => self.interrupts.read_flags(),

            0xFF10..=0xFF14 => self.audio.read(address),
            0xFF16..=0xFF3F => self.audio.read(address),

            0xFF40 => self.graphics.read_lcdc(),
            0xFF41 => self.graphics.read_stat(),
            0xFF42 => self.graphics.read_scy(),
            0xFF43 => self.graphics.read_scx(),
            0xFF44 => self.graphics.read_ly(),
            0xFF45 => self.graphics.read_lyc(),
            0xFF46 => self.oam_dma.read(),
            0xFF47 => self.graphics.read_bgp(),
            0xFF48 => self.graphics.read_obp0(),
            0xFF49 => self.graphics.read_obp1(),
            0xFF4A => self.graphics.read_wy(),
            0xFF4B => self.graphics.read_wx(),

            0xFF4C => 0xFF,                          // (CGB) KEY0: CGB mode.
            0xFF4D => self.speed_switch.read(),      // (CGB) KEY1: Prepare speed switch.
            0xFF4F => self.graphics.vram.read_vbk(), // (CGB) VRAM bank selection.

            0xFF50 => self.bootrom.read_status(),

            // (CGB) VRAM DMA.
            0xFF51 => self.vram_dma.read_hdma1(),
            0xFF52 => self.vram_dma.read_hdma2(),
            0xFF53 => self.vram_dma.read_hdma3(),
            0xFF54 => self.vram_dma.read_hdma4(),
            0xFF55 => self.vram_dma.read_hdma5(),

            // (CGB) BG / OBJ Palettes.
            0xFF68 => self.graphics.read_bcps(),
            0xFF69 => self.graphics.read_bcpd(),
            0xFF6A => self.graphics.read_ocps(),
            0xFF6B => self.graphics.read_ocpd(),

            0xFF6C => self.graphics.read_opri(),

            0xFF70 => self.wram.read_svbk(), // (CGB) WRAM bank selection.

            0xFF80..=0xFFFE => self.hram.read(address),

            0xFFFF => self.interrupts.read_enable(),

            0xFF72 => self.undocumented_registers.read_0xff72(),
            0xFF73 => self.undocumented_registers.read_0xff73(),
            0xFF74 => self.undocumented_registers.read_0xff74(),
            0xFF75 => self.undocumented_registers.read_0xff75(),

            0xFF76..=0xFF77 => self.audio.read(address),

            0xFF03 => 0xFF,          // Unused.
            0xFF08 => 0xFF,          // Unused.
            0xFF09 => 0xFF,          // Unused.
            0xFF0A => 0xFF,          // Unused.
            0xFF0B => 0xFF,          // Unused.
            0xFF0C => 0xFF,          // Unused.
            0xFF0D => 0xFF,          // Unused.
            0xFF0E => 0xFF,          // Unused.
            0xFF15 => 0xFF,          // Unused.
            0xFF4E => 0xFF,          // Unused.
            0xFF56 => 0xFF,          // (CGB) RP: Infrared.
            0xFF57..=0xFF67 => 0xFF, // Unused.
            0xFF6D..=0xFF6F => 0xFF, // Unused.
            0xFF71..=0xFF71 => 0xFF, // Unused.
            0xFF78..=0xFF7F => 0xFF, // Unused.
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x00FF if self.bootrom.is_active() => (),

            0x0000..=0x7FFF => self
                .cartridge
                .as_mut()
                .expect("Cartridge should be loaded")
                .write_rom(address, value),

            0x8000..=0x9FFF => self.graphics.write_vram(address, value),

            0xA000..=0xBFFF => self
                .cartridge
                .as_mut()
                .expect("Cartridge should be loaded")
                .write_ram(address, value),

            0xC000..=0xFDFF => self.wram.write(address, value),
            0xFE00..=0xFE9F => self.graphics.write_oam(address, value),

            0xFEA0..=0xFEFF => (), // Prohibited area, but some games will attempt to write here.

            // I/O registers.
            0xFF00 => self.joypad.write(value),

            0xFF01..=0xFF02 => self.serial.write(address, value),
            0xFF04..=0xFF07 => self.timer.write(address, value),

            0xFF0F => self.interrupts.write_flags(value),

            0xFF10..=0xFF14 => self.audio.write(address, value),
            0xFF16..=0xFF3F => self.audio.write(address, value),

            0xFF40 => self.graphics.write_lcdc(value),
            0xFF41 => self.graphics.write_stat(value),
            0xFF42 => self.graphics.write_scy(value),
            0xFF43 => self.graphics.write_scx(value),
            0xFF44 => println!("[video.rs] LY is read-only."),
            0xFF45 => self.graphics.write_lyc(value),
            0xFF46 => self.oam_dma.write(value),
            0xFF47 => self.graphics.write_bgp(value),
            0xFF48 => self.graphics.write_obp0(value),
            0xFF49 => self.graphics.write_obp1(value),
            0xFF4A => self.graphics.write_wy(value),
            0xFF4B => self.graphics.write_wx(value),

            0xFF4C => self.set_cgb_mode(CgbFlag::from(value).has_cgb_support()), // (CGB) KEY0: CGB mode.
            0xFF4D => self.speed_switch.write(value), // (CGB) KEY1: Prepare speed switch.

            0xFF4F => self.graphics.vram.write_vbk(value), // (CGB) VRAM bank selection.

            0xFF50 => self.bootrom.write_status(value),

            // (CGB) VRAM DMA.
            0xFF51 => self.vram_dma.write_hdma1(value),
            0xFF52 => self.vram_dma.write_hdma2(value),
            0xFF53 => self.vram_dma.write_hdma3(value),
            0xFF54 => self.vram_dma.write_hdma4(value),
            0xFF55 => self.vram_dma.write_hdma5(value),

            // (CGB) BG / OBJ Palettes.
            0xFF68 => self.graphics.write_bcps(value),
            0xFF69 => self.graphics.write_bcpd(value),
            0xFF6A => self.graphics.write_ocps(value),
            0xFF6B => self.graphics.write_ocpd(value),

            0xFF6C => self.graphics.write_opri(value),

            0xFF70 => self.wram.write_svbk(value), // (CGB) WRAM bank selection.

            0xFF80..=0xFFFE => self.hram.write(address, value),

            0xFFFF => self.interrupts.write_enable(value),

            0xFF72 => self.undocumented_registers.write_0xff72(value),
            0xFF73 => self.undocumented_registers.write_0xff73(value),
            0xFF74 => self.undocumented_registers.write_0xff74(value),
            0xFF75 => self.undocumented_registers.write_0xff75(value),

            0xFF76..=0xFF77 => self.audio.write(address, value),

            0xFF03 => (),          // Unused.
            0xFF08 => (),          // Unused.
            0xFF09 => (),          // Unused.
            0xFF0A => (),          // Unused.
            0xFF0B => (),          // Unused.
            0xFF0C => (),          // Unused.
            0xFF0D => (),          // Unused.
            0xFF0E => (),          // Unused.
            0xFF15 => (),          // Unused.
            0xFF4E => (),          // Unused.
            0xFF56 => (),          // (CGB) RP: Infrared.
            0xFF57..=0xFF67 => (), // Unused.
            0xFF6D..=0xFF6F => (), // Unused.
            0xFF71..=0xFF71 => (), // Unused.
            0xFF78..=0xFF7F => (), // Unused.
        }
    }
}

mod bootrom;
mod high_ram;
mod interrupts;
mod oam_dma;
mod speed_switch;
mod undocumented_registers;
mod vram_dma;
mod work_ram;
