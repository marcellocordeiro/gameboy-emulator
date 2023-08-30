#![warn(clippy::pedantic, clippy::perf, clippy::all, clippy::complexity)]
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
    clippy::cast_possible_truncation, // Intentional, but may be possible to mitigate.
    clippy::verbose_bit_mask, // As per the docs, LLVM may not be able to generate better code.
    clippy::cast_possible_wrap,
)]

use constants::{Button, PALETTE};
use cpu::Cpu;

pub struct GameBoy {
    pub cpu: Cpu,
}

impl Default for GameBoy {
    fn default() -> Self {
        Self::new()
    }
}

impl GameBoy {
    pub fn new() -> Self {
        let mut cpu = Cpu::default();

        #[cfg(not(feature = "bootrom"))]
        cpu.skip_bootrom();

        Self { cpu }
    }

    pub fn reset(&mut self) {
        *self = GameBoy::new();
    }

    pub fn load_cartridge(&mut self, rom: Vec<u8>) -> Result<(), cartridge::Error> {
        self.reset();

        self.cpu.memory.load_cartridge(rom)
    }

    pub fn run_frame(&mut self) {
        self.cpu.cycles = 0;
        while self.cpu.cycles < (70224 * 2) {
            self.cpu.step();
        }
    }

    pub fn key_down(&mut self, key: Button) {
        self.cpu.memory.key_down(key);
    }

    pub fn key_up(&mut self, key: Button) {
        self.cpu.memory.key_up(key);
    }

    #[allow(clippy::identity_op)]
    pub fn draw(&self, frame: &mut [u8]) {
        let fb = self.cpu.memory.borrow_framebuffer();

        for (i, pixel) in fb.iter().enumerate() {
            frame[(i * 4) + 0] = PALETTE[*pixel as usize];
            frame[(i * 4) + 1] = PALETTE[*pixel as usize];
            frame[(i * 4) + 2] = PALETTE[*pixel as usize];
            frame[(i * 4) + 3] = 0xFF;
        }
    }
}

pub mod audio;
pub mod cartridge;
pub mod constants;
pub mod cpu;
pub mod graphics;
pub mod joypad;
pub mod memory;
pub mod serial;
pub mod timer;
