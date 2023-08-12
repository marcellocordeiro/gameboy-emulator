use crate::{
    cartridge::Cartridge,
    constants::{Button, HEIGHT, WIDTH},
    graphics::Graphics,
    joypad::Joypad,
    serial::Serial,
    sound::Sound,
    timer::Timer,
};

use self::{bootrom::Bootrom, high_ram::HighRam, interrupts::Interrupts, work_ram::WorkRam};

#[derive(Default)]
pub struct Memory {
    bootrom: Bootrom,

    wram: WorkRam,
    hram: HighRam,

    cartridge: Cartridge,
    graphics: Graphics,
    sound: Sound,

    joypad: Joypad,
    serial: Serial,
    timer: Timer,

    pub interrupts: Interrupts,
}

impl Memory {
    pub fn load_cartridge(&mut self, rom: Vec<u8>) {
        self.cartridge.load_cartridge(rom);
    }

    pub fn skip_bootrom(&mut self) {
        self.bootrom.disable();
        self.graphics.skip_bootrom();
        self.timer.skip_bootrom();
        self.interrupts.skip_bootrom();
    }

    pub fn tick(&mut self) {
        if let Some((source, destination)) = self.graphics.oam_dma.advance() {
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

            0x0000..=0x7FFF => self.cartridge.read_rom(address),
            0x8000..=0x9FFF => self.graphics.read(address),
            0xA000..=0xBFFF => self.cartridge.read_ram(address),
            0xC000..=0xFDFF => self.wram.read(address),
            0xFE00..=0xFE9F => self.graphics.read(address),

            0xFEA0..=0xFEFF => unreachable!("Accessing prohibited area: {:#06x}", address),

            0xFF00..=0xFF00 => self.joypad.read(),
            0xFF01..=0xFF02 => self.serial.read(address),
            0xFF04..=0xFF07 => self.timer.read(address),
            0xFF10..=0xFF3F => self.sound.read(address),
            0xFF40..=0xFF4B => self.graphics.read(address),

            0xFF4D => 0xFF, // TODO: (CGB) KEY1: Prepare speed switch.
            0xFF4F => 0xFF, // TODO: (CGB) VRAM Bank Select.

            0xFF50 => self.bootrom.read_status(),

            0xFF51..=0xFF55 => 0xFF, // TODO: (CGB) VRAM DMA.
            0xFF68..=0xFF69 => 0xFF, // TODO: (CGB) BG / OBJ Palettes.
            0xFF70 => 0xFF,          // TODO: (CGB) WRAM Bank Select.

            0xFF80..=0xFFFE => self.hram.read(address),

            // Interrupt.
            0xFF0F => self.interrupts.read_flags(),
            0xFFFF => self.interrupts.read_enable(),

            0xFF56 => 0xFF, // RP: Infrared.

            0xFF03 => 0xFF,          // Unused.
            0xFF08 => 0xFF,          // Unused.
            0xFF09 => 0xFF,          // Unused.
            0xFF0A => 0xFF,          // Unused.
            0xFF0B => 0xFF,          // Unused.
            0xFF0C => 0xFF,          // Unused.
            0xFF0D => 0xFF,          // Unused.
            0xFF0E => 0xFF,          // Unused.
            0xFF4C => 0xFF,          // Unused.
            0xFF4E => 0xFF,          // Unused.
            0xFF57..=0xFF67 => 0xFF, // Unused.
            0xFF6A..=0xFF6F => 0xFF, // Unused.
            0xFF71..=0xFF7F => 0xFF, // Unused.
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x00FF if self.bootrom.is_active() => (),

            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0x8000..=0x9FFF => self.graphics.write(address, value),
            0xA000..=0xBFFF => self.cartridge.write_ram(address, value),
            0xC000..=0xFDFF => self.wram.write(address, value),
            0xFE00..=0xFE9F => self.graphics.write(address, value),

            0xFEA0..=0xFEFF => (), // Prohibited area, but some games will attempt to write here.

            0xFF00..=0xFF00 => self.joypad.write(value),
            0xFF01..=0xFF02 => self.serial.write(address, value),
            0xFF04..=0xFF07 => self.timer.write(address, value),
            0xFF10..=0xFF3F => self.sound.write(address, value),
            0xFF40..=0xFF4B => self.graphics.write(address, value),

            0xFF4D => (), // TODO: (CGB) KEY1: Prepare speed switch.
            0xFF4F => (), // TODO: (CGB) VRAM Bank Select.

            0xFF50 => self.bootrom.write_status(value),

            0xFF51..=0xFF55 => (), // TODO: (CGB) VRAM DMA.
            0xFF68..=0xFF69 => (), // TODO: (CGB) BG / OBJ Palettes.
            0xFF70 => (),          // TODO: (CGB) WRAM Bank Select.

            0xFF80..=0xFFFE => self.hram.write(address, value),

            0xFF0F => self.interrupts.write_flags(value),
            0xFFFF => self.interrupts.write_enable(value),

            0xFF56 => (), // RP: Infrared.

            0xFF03 => (),          // Unused.
            0xFF08 => (),          // Unused.
            0xFF09 => (),          // Unused.
            0xFF0A => (),          // Unused.
            0xFF0B => (),          // Unused.
            0xFF0C => (),          // Unused.
            0xFF0D => (),          // Unused.
            0xFF0E => (),          // Unused.
            0xFF4C => (),          // Unused.
            0xFF4E => (),          // Unused.
            0xFF57..=0xFF67 => (), // Unused.
            0xFF6A..=0xFF6F => (), // Unused.
            0xFF71..=0xFF7F => (), // Unused.
        }
    }

    pub fn borrow_framebuffer(&self) -> &[u8; WIDTH * HEIGHT] {
        &self.graphics.framebuffer
    }

    pub fn key_down(&mut self, key: Button) {
        self.joypad.key_down(key);
    }

    pub fn key_up(&mut self, key: Button) {
        self.joypad.key_up(key);
    }
}

mod bootrom;
mod high_ram;
mod interrupts;
mod work_ram;
