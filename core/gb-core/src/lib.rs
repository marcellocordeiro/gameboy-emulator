use std::sync::mpsc;

use cartridge::error::Error as CartridgeError;
pub use constants::*;
use cpu::Cpu;
use memory::Memory;
pub use memory::MemoryInterface;
pub use utils::{button::Button, color::Color};

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoy {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();
        let mut memory = Memory::default();

        if !cfg!(feature = "bootrom") {
            cpu.skip_bootrom();
            memory.skip_bootrom();
        }

        Self { cpu, memory }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
        self.memory.reset();

        if !cfg!(feature = "bootrom") {
            self.cpu.skip_bootrom();
            self.memory.skip_bootrom();
        }
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        self.reset();

        self.memory.load_cartridge(rom)
    }

    pub fn cartridge_inserted(&self) -> bool {
        self.memory.cartridge.is_some()
    }

    pub fn get_battery(&self) -> Option<&[u8]> {
        if let Some(cartridge) = self.memory.cartridge.as_ref() {
            return Some(cartridge.get_battery());
        }

        None
    }

    pub fn load_battery(&mut self, file: Vec<u8>) {
        if let Some(cartridge) = self.memory.cartridge.as_mut() {
            cartridge.load_battery(file);
        }
    }

    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
    }

    pub fn run_frame(&mut self) {
        self.cpu.run_frame(&mut self.memory);
    }

    pub fn key_down(&mut self, key: Button) {
        self.memory.joypad.key_down(key);
    }

    pub fn key_up(&mut self, key: Button) {
        self.memory.joypad.key_up(key);
    }

    pub fn draw_into_frame_rgba8888(&self, frame: &mut ScreenPixels) {
        self.memory.ppu.screen().draw_into_frame_rgba8888(frame);
    }

    pub fn draw_into_frame_bgra8888(&self, frame: &mut ScreenPixels) {
        self.memory.ppu.screen().draw_into_frame_bgra8888(frame);
    }

    pub fn add_serial_channel(&mut self, channel: mpsc::Sender<u8>) {
        self.memory.serial.add_sender(channel);
    }
}

mod audio;
mod cartridge;
mod constants;
mod cpu;
mod joypad;
mod memory;
mod ppu;
mod serial;
mod timer;
mod utils;
