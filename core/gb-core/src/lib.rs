#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::match_same_arms,
    clippy::cast_lossless,
    clippy::unused_self,
    clippy::similar_names,
    clippy::enum_glob_use,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::collapsible_if,
    clippy::new_without_default,
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use cartridge::error::Error as CartridgeError;
use constants::ScreenPixels;
use cpu::Cpu;
use memory::{Memory, MemoryInterface};
use utils::button::Button;

pub struct GameBoy {
    pub cpu: Cpu,
    pub memory: Memory,
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
        self.cpu.cycles = 0;
        while self.cpu.cycles < (70224 * 2) {
            self.cpu.step(&mut self.memory);
        }
    }

    pub fn key_down(&mut self, key: Button) {
        self.memory.joypad.key_down(key);
    }

    pub fn key_up(&mut self, key: Button) {
        self.memory.joypad.key_up(key);
    }

    pub fn draw_into_frame_rgba8888(&self, frame: &mut ScreenPixels) {
        self.memory.screen().draw_into_frame_rgba8888(frame);
    }

    pub fn draw_into_frame_bgra8888(&self, frame: &mut ScreenPixels) {
        self.memory.screen().draw_into_frame_bgra8888(frame);
    }
}

#[derive(Default)]
pub struct GameBoyDummy<Mem: MemoryInterface> {
    pub cpu: Cpu,
    pub memory: Mem,
}

impl<Mem: MemoryInterface> GameBoyDummy<Mem> {
    pub fn step(&mut self) {
        self.cpu.step(&mut self.memory);
    }
}

pub(crate) mod audio;
pub(crate) mod cartridge;
pub mod constants;
pub mod cpu;
pub(crate) mod joypad;
pub mod memory;
pub(crate) mod ppu;
pub(crate) mod serial;
pub(crate) mod timer;
pub mod utils;
