use std::sync::Arc;

use log::info;

use self::{
    bootrom::Bootrom,
    high_ram::HighRam,
    interrupts::Interrupts,
    speed_switch::SpeedSwitch,
    undocumented_registers::UndocumentedRegisters,
    work_ram::WorkRam,
};
use crate::{
    audio::Audio,
    cartridge::{
        error::Error as CartridgeError,
        info::{CgbFlag, Info},
        Cartridge,
    },
    joypad::Joypad,
    ppu::Ppu,
    serial::Serial,
    timer::Timer,
    utils::macros::{device_is_cgb, in_cgb_mode},
    DeviceConfig,
    DeviceModel,
    OptionalCgbComponent,
};

pub trait MemoryInterface {
    fn force_cycle(&mut self) {}

    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read_cycle(&mut self, address: u16) -> u8;
    fn write_cycle(&mut self, address: u16, value: u8);

    fn speed_switch(&self) -> &SpeedSwitch;
    fn speed_switch_mut(&mut self) -> &mut SpeedSwitch;

    fn interrupts(&self) -> &Interrupts;
    fn interrupts_mut(&mut self) -> &mut Interrupts;
}

#[derive(Default)]
pub struct Memory {
    bootrom: Bootrom,

    wram: WorkRam,
    hram: HighRam,

    pub(crate) cartridge: Option<Cartridge>,
    pub ppu: Ppu,
    audio: Audio,

    pub(crate) joypad: Joypad,
    pub serial: Serial,
    timer: Timer,

    speed_switch: SpeedSwitch,

    pub interrupts: Interrupts,

    undocumented_registers: UndocumentedRegisters,

    device_config: DeviceConfig,
}

impl MemoryInterface for Memory {
    fn force_cycle(&mut self) {
        self.cycle();
    }

    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF if self.bootrom.is_active() => self.bootrom.read(address),

            0x0200..=0x08FF if self.bootrom.is_active() && device_is_cgb!(self) => {
                self.bootrom.read(address)
            }

            0x0000..=0x3FFF => self
                .cartridge
                .as_ref()
                .expect("Cartridge should be loaded")
                .read_rom_bank_0(address),

            0x4000..=0x7FFF => self
                .cartridge
                .as_ref()
                .expect("Cartridge should be loaded")
                .read_rom_bank_x(address),

            0x8000..=0x9FFF => self.ppu.read_vram(address),

            0xA000..=0xBFFF => self
                .cartridge
                .as_ref()
                .expect("Cartridge should be loaded")
                .read_ram(address),

            0xC000..=0xFDFF => self.wram.read(address),
            0xFE00..=0xFE9F => self.ppu.read_oam(address),

            0xFEA0..=0xFEFF => unreachable!("Accessing prohibited area: {:#06x}", address),

            // I/O registers.
            0xFF00 => self.joypad.read(),

            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),

            0xFF0F => self.interrupts.read_flags(),

            0xFF10..=0xFF14 => self.audio.read(address),
            0xFF16..=0xFF3F => self.audio.read(address),

            0xFF40 => self.ppu.read_lcdc(),
            0xFF41 => self.ppu.read_stat(),
            0xFF42 => self.ppu.read_scy(),
            0xFF43 => self.ppu.read_scx(),
            0xFF44 => self.ppu.read_ly(),
            0xFF45 => self.ppu.read_lyc(),
            0xFF46 => self.ppu.oam_dma.read(),
            0xFF47 => self.ppu.read_bgp(),
            0xFF48 => self.ppu.read_obp0(),
            0xFF49 => self.ppu.read_obp1(),
            0xFF4A => self.ppu.read_wy(),
            0xFF4B => self.ppu.read_wx(),

            0xFF4C => 0xFF, // (CGB) KEY0: CGB mode.

            0xFF4D => self.speed_switch.read(), // (CGB) KEY1: Prepare speed switch.

            0xFF4F => self.ppu.vram.read_vbk(), // (CGB) VRAM bank selection.

            0xFF50 => self.bootrom.read_status(),

            // (CGB) VRAM DMA.
            0xFF51 => self.ppu.vram_dma.read_hdma1(),
            0xFF52 => self.ppu.vram_dma.read_hdma2(),
            0xFF53 => self.ppu.vram_dma.read_hdma3(),
            0xFF54 => self.ppu.vram_dma.read_hdma4(),
            0xFF55 => self.ppu.vram_dma.read_hdma5(),

            // (CGB) BG / OBJ Palettes.
            0xFF68 => self.ppu.read_bcps(),
            0xFF69 => self.ppu.read_bcpd(),
            0xFF6A => self.ppu.read_ocps(),
            0xFF6B => self.ppu.read_ocpd(),

            0xFF6C => self.ppu.read_opri(),

            0xFF70 => self.wram.read_svbk(), // (CGB) WRAM bank selection.

            0xFF72 => self.undocumented_registers.read_0xff72(),
            0xFF73 => self.undocumented_registers.read_0xff73(),
            0xFF74 => self.undocumented_registers.read_0xff74(),
            0xFF75 => self.undocumented_registers.read_0xff75(),

            0xFF76 | 0xFF77 => self.audio.read(address),

            0xFF80..=0xFFFE => self.hram.read(address),

            0xFFFF => self.interrupts.read_enable(),

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

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x00FF if self.bootrom.is_active() => (),

            0x0000..=0x7FFF => self
                .cartridge
                .as_mut()
                .expect("Cartridge should be loaded")
                .write_rom(address, value),

            0x8000..=0x9FFF => self.ppu.write_vram(address, value),

            0xA000..=0xBFFF => self
                .cartridge
                .as_mut()
                .expect("Cartridge should be loaded")
                .write_ram(address, value),

            0xC000..=0xFDFF => self.wram.write(address, value),
            0xFE00..=0xFE9F => self.ppu.write_oam(address, value),

            0xFEA0..=0xFEFF => (), // Prohibited area, but some games will attempt to write here.

            // I/O registers.
            0xFF00 => self.joypad.write(value),

            0xFF01 | 0xFF02 => self.serial.write(address, value),
            0xFF04..=0xFF07 => self.timer.write(address, value),

            0xFF0F => self.interrupts.write_flags(value),

            0xFF10..=0xFF14 => self.audio.write(address, value),
            0xFF16..=0xFF3F => self.audio.write(address, value),

            0xFF40 => self.ppu.write_lcdc(value),
            0xFF41 => self.ppu.write_stat(value),
            0xFF42 => self.ppu.write_scy(value),
            0xFF43 => self.ppu.write_scx(value),
            0xFF44 => (),
            0xFF45 => self.ppu.write_lyc(value),
            0xFF46 => self.ppu.oam_dma.write(value),
            0xFF47 => self.ppu.write_bgp(value),
            0xFF48 => self.ppu.write_obp0(value),
            0xFF49 => self.ppu.write_obp1(value),
            0xFF4A => self.ppu.write_wy(value),
            0xFF4B => self.ppu.write_wx(value),

            0xFF4C => self.set_cgb_mode(CgbFlag::from_code(value).has_cgb_support()), // (CGB) KEY0: CGB mode.

            0xFF4D => self.speed_switch.write(value), // (CGB) KEY1: Prepare speed switch.

            0xFF4F => self.ppu.vram.write_vbk(value), // (CGB) VRAM bank selection.

            0xFF50 => self.bootrom.write_status(value),

            // (CGB) VRAM DMA.
            0xFF51 => self.ppu.vram_dma.write_hdma1(value),
            0xFF52 => self.ppu.vram_dma.write_hdma2(value),
            0xFF53 => self.ppu.vram_dma.write_hdma3(value),
            0xFF54 => self.ppu.vram_dma.write_hdma4(value),
            0xFF55 => self.ppu.vram_dma.write_hdma5(value),

            // (CGB) BG / OBJ Palettes.
            0xFF68 => self.ppu.write_bcps(value),
            0xFF69 => self.ppu.write_bcpd(value),
            0xFF6A => self.ppu.write_ocps(value),
            0xFF6B => self.ppu.write_ocpd(value),

            0xFF6C => self.ppu.write_opri(value),

            0xFF70 => self.wram.write_svbk(value), // (CGB) WRAM bank selection.

            0xFF72 => self.undocumented_registers.write_0xff72(value),
            0xFF73 => self.undocumented_registers.write_0xff73(value),
            0xFF74 => self.undocumented_registers.write_0xff74(value),
            0xFF75 => self.undocumented_registers.write_0xff75(value),

            0xFF76 | 0xFF77 => self.audio.write(address, value),

            0xFF80..=0xFFFE => self.hram.write(address, value),

            0xFFFF => self.interrupts.write_enable(value),

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

    fn read_cycle(&mut self, address: u16) -> u8 {
        self.cycle();
        self.read(address)
    }

    fn write_cycle(&mut self, address: u16, value: u8) {
        self.cycle();
        self.write(address, value);
    }

    fn speed_switch(&self) -> &SpeedSwitch {
        &self.speed_switch
    }

    fn speed_switch_mut(&mut self) -> &mut SpeedSwitch {
        &mut self.speed_switch
    }

    fn interrupts(&self) -> &Interrupts {
        &self.interrupts
    }

    fn interrupts_mut(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }
}

impl OptionalCgbComponent for Memory {
    fn with_device_model(model: DeviceModel) -> Self {
        let device_config = DeviceConfig {
            model,
            ..Default::default()
        };

        let wram = WorkRam::with_device_model(model);
        let ppu = Ppu::with_device_model(model);
        let speed_switch = SpeedSwitch::with_device_model(model);
        let undocumented_registers = UndocumentedRegisters::with_device_model(model);

        Self {
            wram,
            ppu,
            speed_switch,
            undocumented_registers,
            device_config,
            ..Default::default()
        }
    }

    fn set_cgb_mode(&mut self, value: bool) {
        log::info!("{} CGB mode.", if value { "Enabling" } else { "Disabling" });

        assert!(self.device_config.model == DeviceModel::Cgb);

        self.wram.set_cgb_mode(value);
        self.ppu.set_cgb_mode(value);
        self.speed_switch.set_cgb_mode(value);
        self.undocumented_registers.set_cgb_mode(value);

        self.device_config.cgb_mode = value;
    }
}

impl Memory {
    fn cycle(&mut self) {
        // TODO: properly implement double speed.
        /*assert!(
            !self.speed_switch.in_double_speed(),
            "CGB double speed not yet supported."
        );*/

        self.perform_oam_dma();
        self.perform_vram_dma();

        for _ in 0..4 {
            self.timer.tick();
            self.ppu.tick();
        }

        if self.ppu.vblank_irq {
            self.interrupts.request_vblank();
            self.ppu.vblank_irq = false;
        }

        if self.ppu.stat_irq {
            self.interrupts.request_lcd_stat();
            self.ppu.stat_irq = false;
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

    pub(crate) fn load_cartridge(&mut self, rom: Arc<Box<[u8]>>) -> Result<(), CartridgeError> {
        let cartridge = Cartridge::new(rom)?;

        if device_is_cgb!(self) {
            if self.bootrom.is_loaded() {
                self.set_cgb_mode(true);
            } else {
                self.handle_post_bootrom_setup(&cartridge.info);
            }
        }

        self.cartridge = Some(cartridge);

        Ok(())
    }

    pub(crate) fn use_bootrom(&mut self, bootrom: Arc<Box<[u8]>>) {
        self.bootrom.insert(self.device_config.model, bootrom);
        info!("Bootrom loaded.");
    }

    pub(crate) fn skip_bootrom(&mut self) {
        self.bootrom.disable();
        self.ppu.skip_bootrom();
        self.timer.skip_bootrom();
        self.interrupts.skip_bootrom();
    }

    pub(crate) fn handle_post_bootrom_setup(&mut self, info: &Info) {
        if info.cgb_flag.has_cgb_support() {
            self.set_cgb_mode(true);
        }

        self.ppu.handle_post_bootrom_setup(info);
    }

    fn perform_oam_dma(&mut self) {
        if let Some((source, destination)) = self.ppu.oam_dma.perform_dma() {
            let value = self.read(source);
            self.ppu.oam.write(destination, value);
        }
    }

    fn perform_vram_dma(&mut self) {
        if !in_cgb_mode!(self) {
            return;
        }

        if let Some(offsets) = self.ppu.vram_dma.perform_gdma() {
            let source = self.ppu.vram_dma.source();
            let destination = self.ppu.vram_dma.destination();

            for offset in offsets {
                let value = self.read(source + offset);
                self.ppu.vram.write(destination + offset, value);
            }
        }

        if self.ppu.in_hblank() {
            if let Some(offsets) = self.ppu.vram_dma.perform_hdma() {
                let source = self.ppu.vram_dma.source();
                let destination = self.ppu.vram_dma.destination();

                for offset in offsets {
                    let value = self.read(source + offset);
                    self.ppu.vram.write(destination + offset, value);
                }
            }
        }
    }
}

mod bootrom;
mod high_ram;
pub mod interrupts;
pub mod speed_switch;
mod undocumented_registers;
mod work_ram;
