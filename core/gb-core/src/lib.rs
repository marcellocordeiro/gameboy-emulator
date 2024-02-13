use std::sync::{mpsc, Arc};

use cartridge::error::Error as CartridgeError;
pub use constants::*;
use cpu::Cpu;
use log::error;
use memory::Memory;
pub use memory::MemoryInterface;
pub use utils::{button::Button, color::Color};

pub struct GameBoy {
    cpu: Cpu,
    memory: Memory,

    rom: Option<Arc<Box<[u8]>>>,
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

        Self {
            cpu,
            memory,
            rom: None,
        }
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn reset(&mut self) {
        self.cpu = Cpu::default();
        self.memory = Memory::default();

        if let Some(rom) = self.rom.clone() {
            let result = self.memory.load_cartridge(rom);

            if let Err(error) = result {
                error!("{error}");
            }
        }

        if !cfg!(feature = "bootrom") {
            self.cpu.skip_bootrom();
            self.memory.skip_bootrom();
        }
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) -> Result<(), CartridgeError> {
        if self.rom.is_some() {
            self.rom = None;
            self.reset();
        }

        let rom = Arc::<Box<[u8]>>::from(rom.into_boxed_slice());

        self.memory.load_cartridge(rom.clone())?;
        self.rom = Some(rom);

        Ok(())
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

    pub fn set_joypad_button(&mut self, button: Button, value: bool) {
        self.memory.joypad.set_joypad_button(button, value);
    }

    pub fn joypad_button_down(&mut self, key: Button) {
        self.memory.joypad.joypad_button_down(key);
    }

    pub fn joypad_button_up(&mut self, key: Button) {
        self.memory.joypad.joypad_button_up(key);
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
